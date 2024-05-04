//! Elastic APM ingest API support layer.
//!
//! Use the `new_layer` function to create the layer with given `Config`.

use anyhow::Result as AnyResult;

use crate::{config::Config, layer::ApmLayer};

mod apm_client;
pub mod config;
pub mod layer;
pub mod model;
mod visitor;

/// Constructs a new telemetry layer for given APM configuration.
pub fn new_layer(service_name: String, config: Config) -> AnyResult<ApmLayer> {
    ApmLayer::new(config, service_name.into())
}
