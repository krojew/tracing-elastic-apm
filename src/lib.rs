use crate::{config::Config, layer::ApmLayer};

mod apm_client;
pub mod config;
mod layer;
mod model;
mod visitor;

/// Constructs a new telemetry layer for given APM configuration.
pub fn new_layer(service_name: String, config: Config) -> ApmLayer {
    ApmLayer::new(config, service_name.into())
}
