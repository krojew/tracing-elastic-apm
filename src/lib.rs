//! Elastic APM ingest API support layer.
//!
//! Use the `new_layer` function to create the layer with given `Config`.

use anyhow::Result as AnyResult;
use tracing_core::Subscriber;
use tracing_subscriber::registry::LookupSpan;

use crate::{apm::config::Config, layer::ApmLayer};


mod layer;
mod visitor;
mod span_context;
mod trace_context;
pub mod middleware;
pub mod apm;
pub mod interceptor;
mod span_ext;

/// Constructs a new telemetry layer for given APM configuration.
pub fn new_layer<S>(config: Config) -> AnyResult<ApmLayer<S>> 
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    ApmLayer::new(config)
}
