use std::{time::{Duration, Instant, UNIX_EPOCH, SystemTime}, marker::PhantomData, any::TypeId};

use anyhow::Result as AnyResult;

use serde_json::{json};
use tracing::{
    span::{Attributes, Record, self},
    Event, Id, Level, Subscriber,
};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

use crate::{span_ext::WithContext};

use super::*;

use super::{
    visitor::{ApmVisitor}, 
    span_context::{SpanContext, SpanId}, 
    trace_context::{TraceContext, TraceId}, 
    apm::{
        metadata::Metadata, 
        client::{ApmClient, Batch},
        config::Config,
    },
};


/// Telemetry capability that publishes events and spans to Elastic APM.
pub struct ApmLayer<S> {
    client: ApmClient,
    meta: Metadata,

    ignore_urls_re: Option<regex::Regex>,

    get_context: WithContext,
    _subscriber: PhantomData<S>,
}

impl<S> Layer<S> for ApmLayer<S>
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let last_timestamp = Instant::now();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64;
        let span_ref: tracing_subscriber::registry::SpanRef<S> = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span_ref.extensions_mut();
        let mut visitor = ApmVisitor::default();
        attrs.record(&mut visitor);
        // let metadata = attrs.metadata();

        let name = span_ref.name().to_string();
        //create apm::model::span & traceContext, pushed to span_ref.extensions
        if let Some(parent_id) = span_ref.parent().map(|parent_ref| parent_ref.id()) {
            
            let parent_span_ref = ctx.span(&parent_id).expect("Span parent not found!");
            let parent_extensions = parent_span_ref.extensions();
            let parent_trace_ctx = parent_extensions.get::<TraceContext>().expect("Trace context not found!");
            
            let trace_ctx = TraceContext {
                trace_id: parent_trace_ctx.trace_id,
                transaction_id: parent_trace_ctx.transaction_id,
                span_id: SpanId::rand(),
                parent_id: Some(parent_trace_ctx.span_id),
            };
            
            if !self.is_filter_urls(&mut visitor) {
                let new_span = apm::span::create_span(timestamp,&name, &mut visitor, &trace_ctx);
                extensions.insert(new_span);
            }
           
            
            extensions.insert(trace_ctx);
            extensions.insert(visitor);
            extensions.insert(SpanContext {
                duration: Duration::new(0, 0),
                last_timestamp: last_timestamp,
                // ..SpanContext::empty_context()
            }); 
        } else {

            let trace_id = TraceId::rand();
            let id_u64 = ( u128::from_le_bytes(trace_id.to_bytes())  ) as u64;
            let span_id = SpanId::from(id_u64.to_le_bytes());

            let mut trace_ctx = TraceContext {
                trace_id,
                span_id,
                transaction_id: span_id,
                ..Default::default()
            };
            if let Some((remote_trace_id,remote_parent_id)) = apm::fields::extract_traceparent(&mut visitor) {
                trace_ctx.trace_id = remote_trace_id;
                trace_ctx.parent_id = Some(remote_parent_id);
            }
            if !self.is_filter_urls(&mut visitor) {
                let new_transaction = apm::transaction::create_transaction(timestamp,&name,&mut visitor,&trace_ctx);
                extensions.insert(new_transaction);
            }
            
            extensions.insert(trace_ctx);
            extensions.insert(visitor);
            extensions.insert(SpanContext {
                duration: Duration::new(0, 0),
                last_timestamp: last_timestamp,
                // ..SpanContext::empty_context()
            });
            
        }
    }

    fn on_record(&self, span: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        // visit record
        let span_ref = ctx.span(span).expect("Span not found!");
        let mut extensions = span_ref.extensions_mut();
        let visitor = extensions.get_mut::<ApmVisitor>().expect("Visitor not found!");
        values.record(visitor);
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // elastic apm Error only support error logs
        let metadata = event.metadata();
        if metadata.level() != &Level::ERROR {
            return;
        }

        let parent_id = if let Some(parent_id) = event.parent() {
            // explicit parent
            Some(parent_id.clone())
        } else if event.is_root() {
            // don't bother checking thread local if span is explicitly root according to this fn
            None
        } else if let Some(parent_id) = ctx.current_span().id() {
            // implicit parent from thread local ctx
            Some(parent_id.clone())
        } else {
            // no parent span, thus this is a root span
            None
        };
        if let Some(parent_id) = &parent_id {
            let span_ref = ctx.span(parent_id).expect("Span not found!");
            let extensions = span_ref.extensions();
            let trace_ctx = extensions.get::<TraceContext>().expect("Trace context not found!");
            let mut visitor = ApmVisitor::default();
            event.record(&mut visitor);

            let error_log = apm::error_log::create_error_log(&mut visitor,&event, &trace_ctx);
            let metadata = self.meta.create_metadata(&visitor, metadata);
            let batch = Batch::new(metadata, None, None, Some(json!(error_log)));
            self.client.send_batch(batch);
        }
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        
        let span_ref = ctx.span(id).expect("Span not found!");
        let mut extensions = span_ref.extensions_mut();
        let span_ctx = extensions.get_mut::<SpanContext>().expect("Span context not found!");
        span_ctx.last_timestamp = Instant::now();
        
        // let metadata = span_ref.metadata();
    }

    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {

        //compute & set duration
        let timestamp = Instant::now();
        let span_ref = ctx.span(id).expect("Span not found!");
        let mut extensions = span_ref.extensions_mut();

        let span_ctx = extensions.get_mut::<SpanContext>().expect("Span context not found!");
        span_ctx.duration += timestamp.saturating_duration_since(span_ctx.last_timestamp);
    }


    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        
        //clear & remove Apm things
        let span_ref = ctx.span(&id).expect("Span not found!");
        let mut extensions = span_ref.extensions_mut();
        let mut visitor = extensions.remove::<ApmVisitor>().expect("Visitor not found!");
        let span_ctx = extensions.remove::<SpanContext>().expect("Span context not found!");
        let trace_context = extensions.remove::<TraceContext>().expect("Trace context not found!");

       
        let duration = span_ctx.duration.as_micros() as f32 / 1000.;

        //send Batch
        let batch = if let Some(mut span) = extensions.remove::<apm::model::Span>() {
            span.duration = duration;
            apm::span::close_span(&mut span,&mut visitor,&trace_context);
            let metadata = self.meta.create_metadata(&visitor, span_ref.metadata());
            Batch::new(metadata, None, Some(json!(span)), None)
        } else if let Some(mut transaction) = extensions.remove::<apm::model::Transaction>() {
            transaction.duration = duration;
            apm::transaction::close_transaction(&mut transaction,&mut visitor,&trace_context);
            let metadata = self.meta.create_metadata(&visitor, span_ref.metadata());
            Batch::new(metadata, Some(json!(transaction)), None, None)
        } else {
            return;
        };
        
        self.client.send_batch(batch);
       
    }


    // SAFETY: this is safe because the `WithContext` function pointer is valid
    // for the lifetime of `&self`.
    unsafe fn downcast_raw(&self, id: TypeId) -> Option<*const ()> {
        match id {
            id if id == TypeId::of::<Self>() => Some(self as *const _ as *const ()),
            id if id == TypeId::of::<WithContext>() => {
                Some(&self.get_context as *const _ as *const ())
            }
            _ => None,
        }
    }
}

impl<S> ApmLayer<S> 
where
S: Subscriber + for<'span> LookupSpan<'span>,
{
    pub(crate) fn new(config: Config) -> AnyResult<Self> {
        let cfg = config.clone();
        let root_cert_path = config.root_cert_path.clone();
        let allow_invalid_certs = config.allow_invalid_certs.clone();
        let authorization = config.authorization;
        let apm_address = config.apm_address.clone();
        let ignore_urls = config.ignore_urls.clone();
       
        Ok(Self {
            client: ApmClient::new(
                apm_address,
                authorization,
                allow_invalid_certs,
                root_cert_path,
            )?,
            meta: Metadata::new(&cfg),
            ignore_urls_re:ignore_urls.map(|x| regex::Regex::new(x.as_str()).unwrap() ),
            get_context: WithContext(Self::get_context),
            _subscriber: PhantomData,
        })
    }

    fn is_filter_urls(&self, visitor: &mut ApmVisitor) -> bool{
        match self.ignore_urls_re.clone() {
            Some(re) => {
                match apm::fields::extract_http_pathname(visitor) {
                    Some(path) => {
                        re.is_match(path.as_str())
                    },
                    None => false,
                }
            }
            None => false,
        }
    }

    fn get_context(
        dispatch: &tracing::Dispatch,
        id: &span::Id,
        f: &mut dyn FnMut(&mut TraceContext),
    ) {
        let subscriber = dispatch
            .downcast_ref::<S>()
            .expect("subscriber should downcast to expected type; this is a bug!");
        let span_ref = subscriber
            .span(id)
            .expect("registry should have a span for the current ID");

        let mut extensions = span_ref.extensions_mut();
        // let mut extensions2 = span_ref.extensions_mut();
        // let visitor = extensions2.get_mut::<ApmVisitor>().expect("Visitor not found!");
        let trace_ctx = extensions.get_mut::<TraceContext>().expect("Trace context not found!");
        f(trace_ctx);
    }
}


