use tracing::span;
use tracing_core::Dispatch;
use crate::{trace_context::TraceContext};

// this function "remembers" the types of the subscriber so that we
// can downcast to something aware of them without knowing those
// types at the callsite.
//
// See https://github.com/tokio-rs/tracing/blob/4dad420ee1d4607bad79270c1520673fa6266a3d/tracing-error/src/layer.rs
pub(crate) struct WithContext(
    pub fn(&Dispatch, &span::Id, f: &mut dyn FnMut(&mut TraceContext)),
);

impl WithContext {
    // This function allows a function to be called in the context of the
    // "remembered" subscriber.
    pub(crate) fn with_context<'a>(
        &self,
        dispatch: &'a tracing::Dispatch,
        id: &span::Id,
        mut f: impl FnMut(&mut TraceContext),
    ) {
        (self.0)(dispatch, id, &mut f)
    }
}




pub trait ApmSpanExt {
    fn context(&self) -> TraceContext;
    // fn mut_context(&self,  f: impl FnMut(&mut TraceContext, &mut ApmVisitor) );
    // fn extract(&self, headers: &http::HeaderMap);
}

impl ApmSpanExt for tracing::Span {

    // fn extract(&self, headers: &http::HeaderMap) {
    //     self.with_subscriber(|(id, subscriber)| {
    //         if let Some(get_context) = subscriber.downcast_ref::<WithContext>() {
    //             get_context.with_context(subscriber, id, |ctx| {
    //                 println!(">>>ctx.extract={:?}",ctx);
    //                 ctx.extract(headers);
    //                 println!(">>>ctx.extract={:?}",ctx);
    //             })
    //         }
    //     });
    // }

    // fn mut_context(&self, mut f: impl FnMut(&mut TraceContext, &mut ApmVisitor) ) {
    //     self.with_subscriber(|(id, subscriber)| {
    //         if let Some(get_context) = subscriber.downcast_ref::<WithContext>() {
    //             get_context.with_context(subscriber, id, |_ctx,_visitor| {
    //                f(_ctx,_visitor)
    //             })
    //         }
    //     });
    // }

    fn context(&self) -> TraceContext {
        let mut ctx:Option<TraceContext> = None;
        self.with_subscriber(|(id, subscriber)| {
            if let Some(get_context) = subscriber.downcast_ref::<WithContext>() {
                get_context.with_context(subscriber, id, |_ctx| {
                    ctx = Some(_ctx.clone());
                })
            }
        });
        ctx.unwrap_or_default()
    }
}
