use tracing_distributed::Span;

use crate::model::Transaction;

pub(crate) struct ApmClient {
    apm_address: String,
    secret_token: Option<String>,
}

impl ApmClient {
    pub fn new(apm_address: String, secret_token: Option<String>) -> Self {
        ApmClient {
            apm_address,
            secret_token,
        }
    }
}
