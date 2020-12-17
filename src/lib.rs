use rand::prelude::*;
use tracing_distributed::TelemetryLayer;

use crate::{
    config::Config,
    telemetry::{ApmTelemetry, SpanId, TraceId},
};

mod apm_client;
pub mod config;
mod model;
mod telemetry;
mod visitor;

/// Constructs a new telemetry layer for given APM configuration.
pub fn new_layer(
    service_name: &'static str,
    config: Config,
) -> TelemetryLayer<ApmTelemetry, SpanId, TraceId> {
    let instance_id = thread_rng().gen();
    TelemetryLayer::new(service_name, ApmTelemetry::new(config), move |tracing_id| {
        SpanId::new(tracing_id, instance_id)
    })
}
