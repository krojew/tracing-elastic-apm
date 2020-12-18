use std::{
    fmt::{Display, Formatter, Result},
    ops::Deref,
    sync::Arc,
};

use reqwest::{header, Client};
use serde_json::Value;
use tracing::*;

use crate::config::Authorization;

pub(crate) struct Batch {
    metadata: Value,
    transaction: Option<Value>,
    span: Option<Value>,
    error: Option<Value>,
}

impl Display for Batch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{}", self.metadata)?;

        if let Some(transaction) = &self.transaction {
            writeln!(f, "{}", transaction)?;
        }

        if let Some(span) = &self.span {
            writeln!(f, "{}", span)?;
        }

        if let Some(error) = &self.error {
            writeln!(f, "{}", error)?;
        }

        Ok(())
    }
}

impl Batch {
    pub fn new(
        metadata: Value,
        transaction: Option<Value>,
        span: Option<Value>,
        error: Option<Value>,
    ) -> Self {
        Batch {
            metadata,
            transaction,
            span,
            error,
        }
    }
}

pub(crate) struct ApmClient {
    apm_address: Arc<String>,
    authorization: Option<Arc<String>>,
    client: Client,
}

impl ApmClient {
    pub fn new(apm_address: String, authorization: Option<Authorization>) -> Self {
        let authorization = authorization
            .map(|authorization| match authorization {
                Authorization::SecretToken(token) => format!("Bearer {}", token),
                Authorization::ApiKey(key) => {
                    base64::encode(format!("ApiKey {}:{}", key.id, key.key))
                }
            })
            .map(Arc::new);

        ApmClient {
            apm_address: Arc::new(apm_address),
            authorization,
            client: Client::new(),
        }
    }

    pub fn send_batch(&self, batch: Batch) {
        let client = self.client.clone();
        let apm_address = self.apm_address.clone();
        let authorization = self.authorization.clone();

        tokio::spawn(async move {
            let mut request = client
                .post(&format!("{}/intake/v2/events", apm_address))
                .body(batch.to_string());

            if let Some(authorization) = &authorization {
                request = request.header(header::AUTHORIZATION, authorization.deref());
            }

            let result = request.send().await;
            if let Err(error) = result {
                error!(error = %error, "Error sending batch to APM!");
            }
        });
    }
}
