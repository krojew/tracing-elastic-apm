//! Layer configuration.

use crate::model::{Cloud, Framework, Language, Process, Runtime, ServiceNode, System, User};

pub struct Service {
    pub(crate) version: Option<String>,
    pub(crate) environment: Option<String>,
    pub(crate) language: Option<Language>,
    pub(crate) runtime: Option<Runtime>,
    pub(crate) framework: Option<Framework>,
    pub(crate) node: Option<ServiceNode>,
}

impl Service {
    pub fn new(
        version: Option<String>,
        environment: Option<String>,
        language: Option<Language>,
        runtime: Option<Runtime>,
        framework: Option<Framework>,
        node: Option<ServiceNode>,
    ) -> Self {
        Service {
            version,
            environment,
            language,
            runtime,
            framework,
            node,
        }
    }
}

pub struct ApiKey {
    pub(crate) id: String,
    pub(crate) key: String,
}

impl ApiKey {
    pub fn new(id: String, key: String) -> Self {
        ApiKey { id, key }
    }
}

// APM authorization method.
pub enum Authorization {
    SecretToken(String),
    ApiKey(ApiKey),
}

#[derive(Default)]
pub struct Config {
    pub(crate) apm_address: String,
    pub(crate) authorization: Option<Authorization>,
    pub(crate) service: Option<Service>,
    pub(crate) process: Option<Process>,
    pub(crate) system: Option<System>,
    pub(crate) user: Option<User>,
    pub(crate) cloud: Option<Cloud>,
    pub(crate) allow_invalid_certs: Option<bool>,
}

impl Config {
    pub fn new(apm_address: String) -> Self {
        Config {
            apm_address,
            ..Default::default()
        }
    }

    pub fn verify_certs(mut self, verify: bool) -> Self {
        self.allow_invalid_certs = Some(verify);
        self
    }

    pub fn with_authorization(mut self, authorization: Authorization) -> Self {
        self.authorization = Some(authorization);
        self
    }

    pub fn with_service(mut self, service: Service) -> Self {
        self.service = Some(service);
        self
    }

    pub fn with_process(mut self, process: Process) -> Self {
        self.process = Some(process);
        self
    }

    pub fn with_system(mut self, system: System) -> Self {
        self.system = Some(system);
        self
    }

    pub fn with_user(mut self, user: User) -> Self {
        self.user = Some(user);
        self
    }

    pub fn with_cloud(mut self, cloud: Cloud) -> Self {
        self.cloud = Some(cloud);
        self
    }
}
