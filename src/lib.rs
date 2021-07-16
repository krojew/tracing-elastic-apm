//! Elastic APM ingest API support layer.
//!
//! Use the `new_layer` function to create the layer with given `Config`.

use crate::{config::Config, layer::ApmLayer};
use anyhow::Result as AnyResult;

mod apm_client;
pub mod config;
mod layer;
pub mod model;
mod visitor;

/// Constructs a new telemetry layer for given APM configuration.
pub fn new_layer(service_name: String, config: Config) -> AnyResult<ApmLayer> {
    ApmLayer::new(config, service_name.into())
}
