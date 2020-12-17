use crate::model::{Cloud, Framework, Language, Process, Runtime, ServiceNode, System, User};

pub struct Service {
    version: Option<String>,
    environment: Option<String>,
    language: Option<Language>,
    runtime: Option<Runtime>,
    framework: Option<Framework>,
    node: Option<ServiceNode>,
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

#[derive(Default)]
pub struct Config {
    pub(crate) apm_address: String,
    pub(crate) secret_token: Option<String>,
    pub(crate) service: Option<Service>,
    pub(crate) process: Option<Process>,
    pub(crate) system: Option<System>,
    pub(crate) user: Option<User>,
    pub(crate) cloud: Option<Cloud>,
}

impl Config {
    pub fn new(apm_address: String) -> Self {
        Config {
            apm_address,
            ..Default::default()
        }
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
