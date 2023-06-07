use serde_json::{Value, json};

use crate::{apm::config::Config, visitor::ApmVisitor, apm::*};

use super::util::{build_process, build_system};


pub struct Metadata {
    pub json_metadata: Value,
}

impl Metadata {
    pub fn new(config: &Config) -> Self {
        let conf_service = &config.service.clone().unwrap();
        let metadata = model::Metadata {
            service: model::Service {
                name: conf_service.clone().name.unwrap(),
                version: conf_service.clone().version,
                environment: conf_service.clone().environment,
                language: conf_service.clone().language,
                runtime: conf_service.clone().runtime,
                framework: conf_service.clone().framework,
                agent: model::Agent {
                    name: "otlp".to_string(),
                    version: "unknown".to_string(),
                    ephemeral_id: None,
                },
                node: conf_service.clone().node,
            },
            process: build_process(),
            system: build_system(),
            user: config.user.clone(),
            cloud: config.cloud.clone(),
            labels: None,
        };
        Metadata {
            json_metadata: json!(metadata)
        }
    }


    pub(crate) fn create_metadata(
        &self,
        visitor: &ApmVisitor,
        meta: &'static tracing::Metadata<'static>,
    ) -> Value {
        //static metadata
        let mut json_metadata = self.json_metadata.clone();
    
        //add dynamic elements
        if !visitor.0.is_empty() {
            json_metadata["labels"] = json!(visitor.0);
            json_metadata["labels"]["level"] = json!(meta.level().to_string());
            json_metadata["labels"]["target"] = json!(meta.target().to_string());
        }
    
        json_metadata
    }
}

