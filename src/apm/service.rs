use crate::apm::model::{Language, Runtime, Framework, ServiceNode};


/**  sample data for golang application
 * service
 * service.environment          development
 * service.framework.name       grpc
 * service.framework.version    1.51.0
 * service.language.name        go
 * service.language.version     go1.18.2
 * service.name                 onboardingd
 * service.node.name            5966237c45724a60e97b32f30d319ee6aff060e3f468030eadb0bc7bfc65ac14
 * service.runtime.name         gc
 * service.runtime.version      go1.18.2
 */
#[derive(Default, Clone)]
pub struct Service {
    pub(crate) name: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) environment: Option<String>,
    pub(crate) language: Option<Language>,
    pub(crate) runtime: Option<Runtime>,
    pub(crate) framework: Option<Framework>,
    pub(crate) node: Option<ServiceNode>,
}

impl Service {
    pub fn new(
        name: Option<String>,
        version: Option<String>,
        environment: Option<String>,
        language: Option<Language>,
        runtime: Option<Runtime>,
        framework: Option<Framework>,
        node: Option<ServiceNode>,
    ) -> Self {
        Service {
            name,
            version,
            environment,
            language,
            runtime,
            framework,
            node,
        }
    }
}