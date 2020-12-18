use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    time::UNIX_EPOCH,
};

use rand::prelude::*;
use serde_json::{json, Value};
use tracing::Id;
use tracing_distributed::{Event, Span as TracingSpan, Telemetry};

use crate::{
    apm_client::{ApmClient, Batch},
    config::Config,
    model::{Agent, Error, Log, Metadata, Service, Span, Transaction},
    visitor::ApmVisitor,
};

pub type TraceId = u128;

#[derive(PartialEq, Eq, Clone)]
pub struct SpanId {
    tracing_id: Id,
    instance_id: u64,
}

impl Display for SpanId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}-{}", self.tracing_id.into_u64(), self.instance_id)
    }
}

impl SpanId {
    #[inline]
    pub(crate) fn new(tracing_id: Id, instance_id: u64) -> Self {
        SpanId {
            tracing_id,
            instance_id,
        }
    }
}

/// Telemetry capability that publishes events and spans to Elastic APM.
pub struct ApmTelemetry {
    client: ApmClient,
    metadata: Value,
}

impl ApmTelemetry {
    pub(crate) fn new(mut config: Config, service_name: String) -> Self {
        let metadata = Metadata {
            service: Service {
                name: service_name,
                version: config
                    .service
                    .as_mut()
                    .and_then(|service| service.version.take()),
                environment: config
                    .service
                    .as_mut()
                    .and_then(|service| service.environment.take()),
                language: config
                    .service
                    .as_mut()
                    .and_then(|service| service.language.take()),
                runtime: config
                    .service
                    .as_mut()
                    .and_then(|service| service.runtime.take()),
                framework: config
                    .service
                    .as_mut()
                    .and_then(|service| service.framework.take()),
                agent: Agent {
                    name: "tracing-elastic-apm".to_string(),
                    version: version::version!().to_string(),
                    ephemeral_id: None,
                },
                node: config
                    .service
                    .as_mut()
                    .and_then(|service| service.node.take()),
            },
            process: config.process,
            system: config.system,
            user: config.user,
            cloud: config.cloud,
            labels: None,
        };

        ApmTelemetry {
            client: ApmClient::new(config.apm_address, config.authorization),
            metadata: json!(metadata),
        }
    }

    fn create_metadata(
        &self,
        visitor: &ApmVisitor,
        meta: &'static tracing::Metadata<'static>,
    ) -> Value {
        let mut metadata = self.metadata.clone();

        if !visitor.0.is_empty() {
            metadata["labels"] = json!(visitor.0);
            metadata["labels"]["level"] = json!(meta.level().to_string());
            metadata["labels"]["target"] = json!(meta.target().to_string());
        }

        metadata
    }
}

impl Telemetry for ApmTelemetry {
    type Visitor = ApmVisitor;
    type TraceId = TraceId;
    type SpanId = SpanId;

    fn mk_visitor(&self) -> Self::Visitor {
        ApmVisitor::default()
    }

    fn report_span(&self, span: TracingSpan<Self::Visitor, Self::SpanId, Self::TraceId>) {
        let metadata = self.create_metadata(&span.values, span.meta);

        let batch = if span.parent_id.is_some() {
            Batch::new(metadata, Some(json!(Span::from(span))), None, None)
        } else {
            Batch::new(metadata, None, Some(json!(Transaction::from(span))), None)
        };

        self.client.send_batch(batch);
    }

    fn report_event(&self, event: Event<Self::Visitor, Self::SpanId, Self::TraceId>) {
        let metadata = self.create_metadata(&event.values, event.meta);
        self.client.send_batch(Batch::new(
            metadata,
            None,
            None,
            Some(json!(Error::from(event))),
        ));
    }
}

impl From<TracingSpan<ApmVisitor, SpanId, TraceId>> for Transaction {
    fn from(value: TracingSpan<ApmVisitor, SpanId, TraceId>) -> Self {
        let timestamp = value.initialized_at.duration_since(UNIX_EPOCH).unwrap();
        let duration = value.completed_at - timestamp;

        Transaction {
            id: value.id.to_string(),
            trace_id: value.trace_id.to_string(),
            parent_id: value.parent_id.map(|id| id.to_string()),
            timestamp: Some(timestamp.as_micros()),
            duration: duration.duration_since(UNIX_EPOCH).unwrap().as_micros() as f32 / 1000.,
            name: Some(value.meta.name().to_string()),
            ..Default::default()
        }
    }
}

impl From<TracingSpan<ApmVisitor, SpanId, TraceId>> for Span {
    fn from(value: TracingSpan<ApmVisitor, SpanId, TraceId>) -> Self {
        let timestamp = value.initialized_at.duration_since(UNIX_EPOCH).unwrap();
        let duration = value.completed_at - timestamp;

        Span {
            id: value.id.to_string(),
            trace_id: value.trace_id.to_string(),
            parent_id: value.parent_id.unwrap().to_string(),
            timestamp: Some(timestamp.as_micros()),
            duration: duration.duration_since(UNIX_EPOCH).unwrap().as_micros() as f32 / 1000.,
            name: value.meta.name().to_string(),
            span_type: "custom".to_string(),
            ..Default::default()
        }
    }
}

impl From<Event<ApmVisitor, SpanId, TraceId>> for Error {
    fn from(value: Event<ApmVisitor, SpanId, TraceId>) -> Self {
        Error {
            id: thread_rng().gen::<u128>().to_string(),
            trace_id: Some(value.trace_id.to_string()),
            parent_id: value.parent_id.map(|id| id.to_string()),
            culprit: Some(value.meta.target().to_string()),
            log: Some(Log {
                level: Some(value.meta.level().to_string()),
                message: value
                    .values
                    .0
                    .get("message")
                    .map(|message| message.to_string())
                    .unwrap_or_default(),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
