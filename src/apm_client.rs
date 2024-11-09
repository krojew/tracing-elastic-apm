use std::{
    fmt::{Display, Formatter, Result},
    ops::Deref,
    sync::Arc,
};

use anyhow::Result as AnyResult;
use base64::engine::general_purpose;
use base64::Engine;
use reqwest::{header, Client};
use serde_json::{json, Value};
use std::io::Read;
use tokio::runtime;
use tokio::runtime::Runtime;
use tracing::subscriber;
use tracing::subscriber::NoSubscriber;

use crate::config::Authorization;

#[derive(Debug)]
pub(crate) struct Batch {
    metadata: Value,
    transaction: Option<Value>,
    span: Option<Value>,
    error: Option<Value>,
}

impl Display for Batch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{}", json!({ "metadata": self.metadata }))?;

        if let Some(transaction) = &self.transaction {
            writeln!(f, "{}", json!({ "transaction": transaction }))?;
        }

        if let Some(span) = &self.span {
            writeln!(f, "{}", json!({ "span": span }))?;
        }

        if let Some(error) = &self.error {
            writeln!(f, "{}", json!({ "error": error }))?;
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
    runtime: Runtime,
}

impl ApmClient {
    pub fn new(
        apm_address: String,
        authorization: Option<Authorization>,
        allow_invalid_certs: bool,
        root_cert_path: Option<String>,
    ) -> AnyResult<Self> {
        let authorization = authorization
            .map(|authorization| match authorization {
                Authorization::SecretToken(token) => format!("Bearer {}", token),
                Authorization::ApiKey(key) => {
                    format!(
                        "ApiKey {}",
                        general_purpose::STANDARD.encode(format!("{}:{}", key.id, key.key))
                    )
                }
            })
            .map(Arc::new);

        let mut client_builder = reqwest::ClientBuilder::new();
        if allow_invalid_certs {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }
        if let Some(path) = root_cert_path {
            let mut buff = Vec::new();
            std::fs::File::open(path)?.read_to_end(&mut buff)?;
            let cert = reqwest::Certificate::from_pem(&buff)?;
            client_builder = client_builder.add_root_certificate(cert);
        }

        let client = client_builder.build()?;

        // we need a separate runtime, because `hyper` can create spans, which need to be ignored by
        // thread-local `NoSubscriber`
        let runtime = runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?;

        Ok(ApmClient {
            apm_address: Arc::new(apm_address),
            authorization,
            client,
            runtime,
        })
    }

    pub fn send_batch(&self, batch: Batch) {
        let client = self.client.clone();
        let apm_address = self.apm_address.clone();
        let authorization = self.authorization.clone();

        self.runtime.spawn(async move {
            let _subscriber_guard = subscriber::set_default(NoSubscriber::default());
            let mut request = client
                .post(format!("{}/intake/v2/events", apm_address))
                .header(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static("application/x-ndjson"),
                )
                .body(batch.to_string());

            if let Some(authorization) = &authorization {
                request = request.header(header::AUTHORIZATION, authorization.deref());
            }

            let result = request.send().await;
            if let Err(error) = result {
                tracing::warn!(%error, "Error sending batch to APM");
            }
        });
    }
}
