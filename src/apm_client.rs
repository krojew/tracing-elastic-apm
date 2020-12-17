use tracing_distributed::Span;

use crate::{config::Authorization, model::Transaction};

pub(crate) struct ApmClient {
    apm_address: String,
    authorization: Option<Authorization>,
}

impl ApmClient {
    pub fn new(apm_address: String, authorization: Option<Authorization>) -> Self {
        ApmClient {
            apm_address,
            authorization,
        }
    }
}
