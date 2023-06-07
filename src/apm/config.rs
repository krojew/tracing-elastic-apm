//! Layer configuration.

use crate::apm::*;
use util::{build_service};
use crate::apm::service::Service;

/// Name for the trace id field, if one needs to be supplied manually.
pub const TRACE_ID_FIELD_NAME: &str = "trace_id";

#[derive(Debug,Clone)]
pub struct ApiKey {
    pub(crate) id: String,
    pub(crate) key: String,
}

impl ApiKey {
    pub fn new(id: String, key: String) -> Self {
        ApiKey { id, key }
    }
}

/// APM authorization method.
#[derive(Debug,Clone)]
pub enum Authorization {
    SecretToken(String),
    ApiKey(ApiKey),
}

#[derive(Default, Clone)]
pub struct Config {
    pub(crate) apm_address: String,
    pub(crate) authorization: Option<Authorization>,
    pub(crate) service: Option<Service>,
    pub(crate) process: Option<model::Process>,
    pub(crate) system: Option<model::System>,
    pub(crate) user: Option<model::User>,
    pub(crate) cloud: Option<model::Cloud>,
    pub(crate) allow_invalid_certs: bool,
    pub(crate) root_cert_path: Option<String>,
    pub(crate) ignore_urls: Option<String>,
}

impl Config {
    pub fn new(apm_address: String) -> Self {
        Config {
            apm_address,
            ..Default::default()
        }
    }

    pub fn from_env() -> Self {
        let apm_address = std::env::var("ELASTIC_APM_SERVER_URL").expect("ELASTIC_APM_SERVER_URL should be setting");
        let apm_secret_token = std::env::var("ELASTIC_APM_SECRET_TOKEN").expect("ELASTIC_APM_SECRET_TOKEN should be setting");
        let ignore_urls = std::env::var("ELASTIC_APM_IGNORE_URLS").ok();
        let authorization = Some(Authorization::SecretToken(apm_secret_token));
        let service  = build_service();
        Config {
            apm_address,
            authorization,
            service,
            ignore_urls,
            ..Default::default()
        }
    }

    pub fn allow_invalid_certificates(mut self, verify: bool) -> Self {
        self.allow_invalid_certs = verify;
        self
    }

    pub fn with_root_cert_path(mut self, cert_path: String) -> Self {
        self.root_cert_path = Some(cert_path);
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

    pub fn with_process(mut self, process: model::Process) -> Self {
        self.process = Some(process);
        self
    }

    pub fn with_system(mut self, system: model::System) -> Self {
        self.system = Some(system);
        self
    }

    pub fn with_user(mut self, user: model::User) -> Self {
        self.user = Some(user);
        self
    }

    pub fn with_cloud(mut self, cloud: model::Cloud) -> Self {
        self.cloud = Some(cloud);
        self
    }

    pub fn with_ignore_urls(mut self, ignore_urls: String) -> Self {
        self.ignore_urls = Some(ignore_urls);
        self
    }
}
