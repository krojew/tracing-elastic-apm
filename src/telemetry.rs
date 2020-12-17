use tracing::Id;
use tracing_distributed::{Event, Span, Telemetry};

use crate::{apm_client::ApmClient, config::Config, visitor::ApmVisitor};

pub type TraceId = u128;

#[derive(PartialEq, Eq, Clone)]
pub struct SpanId {
    tracing_id: Id,
    instance_id: u64,
}

impl SpanId {
    #[inline]
    pub fn new(tracing_id: Id, instance_id: u64) -> Self {
        SpanId {
            tracing_id,
            instance_id,
        }
    }
}

/// Telemetry capability that publishes events and spans to Elastic APM.
pub struct ApmTelemetry {
    client: ApmClient,
}

impl ApmTelemetry {
    pub fn new(config: Config) -> Self {
        ApmTelemetry {
            client: ApmClient::new(config.apm_address, config.secret_token),
        }
    }
}

impl Telemetry for ApmTelemetry {
    type Visitor = ApmVisitor;
    type TraceId = TraceId;
    type SpanId = SpanId;

    fn mk_visitor(&self) -> Self::Visitor {
        ApmVisitor::default()
    }

    fn report_span(&self, span: Span<Self::Visitor, Self::SpanId, Self::TraceId>) {
        unimplemented!()
    }

    fn report_event(&self, event: Event<Self::Visitor, Self::SpanId, Self::TraceId>) {
        unimplemented!()
    }
}
