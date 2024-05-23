//! APM data model.

#[cfg(feature = "valuable")]
use std::collections::HashMap;

#[cfg(not(feature = "valuable"))]
use fxhash::FxHashMap;
use serde::Serialize;
use serde_json::Value;
#[cfg(feature = "valuable")]
use valuable::Valuable;

#[cfg(not(feature = "valuable"))]
pub type Tags = FxHashMap<String, Value>;
#[cfg(feature = "valuable")]
pub type Tags = HashMap<String, Value>;

#[cfg(not(feature = "valuable"))]
pub type Headers = FxHashMap<String, String>;
#[cfg(feature = "valuable")]
pub type Headers = HashMap<String, String>;

#[cfg(feature = "valuable")]
struct VisitHeaders {
    headers: HashMap<String, String>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for VisitHeaders {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Mappable(v) = value {
            v.visit(self)
        }
    }

    fn visit_entry(&mut self, key: valuable::Value<'_>, value: valuable::Value<'_>) {
        if let (valuable::Value::String(k), valuable::Value::String(v)) = (key, value) {
            self.headers.insert(k.to_string(), v.to_string());
        }
    }
}

#[cfg(not(feature = "valuable"))]
type Marks = FxHashMap<String, FxHashMap<String, f32>>;
#[cfg(feature = "valuable")]
type Marks = HashMap<String, HashMap<String, f32>>;

/// Name and version of the programming language used.
#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Language {
    pub name: String,
    pub version: Option<String>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Language {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("name") {
            self.name = v.to_string()
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("version") {
            self.version = Some(v.to_string())
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
/// Name and version of the language runtime running this service.
#[derive(Default, Serialize, Debug)]
pub struct Runtime {
    pub name: String,
    pub version: String,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Runtime {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("name") {
            self.name = v.to_string()
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("version") {
            self.version = v.to_string()
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
/// Name and version of the web framework used.
#[derive(Default, Serialize, Debug)]
pub struct Framework {
    pub name: Option<String>,
    pub version: Option<String>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Framework {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("name") {
            self.name = Some(v.to_string())
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("version") {
            self.version = Some(v.to_string())
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Agent {
    pub name: String,
    pub version: String,
    pub ephemeral_id: Option<String>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Agent {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("name") {
            self.name = v.to_string()
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("version") {
            self.version = v.to_string()
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("ephemeral_id") {
            self.ephemeral_id = Some(v.to_string())
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
/// Unique meaningful name of the service node.
pub struct ServiceNode {
    pub configured_name: Option<String>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for ServiceNode {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("configured_name") {
            self.configured_name = Some(v.to_string())
        };
    }
}

#[derive(Default, Serialize, Debug)]
pub struct Service {
    pub name: String,
    pub version: Option<String>,
    pub environment: Option<String>,
    pub language: Option<Language>,
    pub runtime: Option<Runtime>,
    pub framework: Option<Framework>,
    pub agent: Agent,
    pub node: Option<ServiceNode>,
}

/// Process information.
#[derive(Default, Serialize, Debug)]
pub struct Process {
    /// Process ID of the service.
    pub pid: i32,
    /// Parent process ID of the service.
    pub ppid: Option<i32>,
    pub title: Option<String>,
    /// Command line arguments used to start this process.
    pub argv: Option<Vec<String>>,
}

#[derive(Default, Serialize, Debug)]
pub struct Container {
    /// Container ID.
    pub id: String,
}

#[derive(Default, Serialize, Debug)]
pub struct Node {
    /// Kubernetes node name.
    pub name: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Pod {
    /// Kubernetes pod name.
    pub name: Option<String>,
    /// Kubernetes pod uid.
    pub uid: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Kubernetes {
    /// Kubernetes namespace.
    pub namespace: Option<String>,
    pub pod: Option<Pod>,
    pub node: Option<Node>,
}

#[derive(Default, Serialize, Debug)]
pub struct System {
    pub architecture: Option<String>,
    pub hostname: Option<String>,
    pub detected_hostname: Option<String>,
    pub configured_hostname: Option<String>,
    pub platform: Option<String>,
    pub container: Option<Container>,
    pub kubernetes: Option<Kubernetes>,
}

#[derive(Default, Serialize, Debug)]
pub struct User {
    /// Identifier of the logged in user, e.g. the primary key of the user.
    pub id: Option<String>,
    /// Email of the logged in user.
    pub email: Option<String>,
    /// The username of the logged in user.
    pub username: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Account {
    /// Cloud account ID.
    pub id: Option<String>,
    /// Cloud account name.
    pub name: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Machine {
    /// Cloud instance/machine type.
    #[serde(rename = "type")]
    pub machine_type: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Project {
    /// Cloud project ID.
    pub id: Option<String>,
    /// Cloud project name.
    pub name: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Instance {
    /// Cloud instance/machine ID.
    pub id: Option<String>,
    /// Cloud instance/machine name.
    pub name: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Cloud {
    pub account: Option<Account>,
    /// Cloud availability zone name. e.g. us-east-1a.
    pub availability_zone: Option<String>,
    pub instance: Option<Instance>,
    pub machine: Option<Machine>,
    pub project: Option<Project>,
    /// Cloud provider name. e.g. aws, azure, gcp, digitalocean.
    pub provider: String,
    /// Cloud region name. e.g. us-east-1.
    pub region: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Metadata {
    pub service: Service,
    pub process: Option<Process>,
    pub system: Option<System>,
    pub user: Option<User>,
    pub cloud: Option<Cloud>,
    pub labels: Option<Tags>,
}

#[derive(Default, Serialize, Debug)]
pub struct SpanCount {
    pub started: i32,
    pub dropped: Option<i32>,
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Response {
    pub status_code: Option<i32>,
    pub transfer_size: Option<f32>,
    pub encoded_body_size: Option<f32>,
    pub decoded_body_size: Option<f32>,
    pub headers: Option<Headers>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Response {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::I32(v)) = named_values.get_by_name("status_code") {
            self.status_code = Some(*v)
        };
        if let Some(valuable::Value::F32(v)) = named_values.get_by_name("transfer_size") {
            self.transfer_size = Some(*v)
        };
        if let Some(valuable::Value::F32(v)) = named_values.get_by_name("encoded_body_size") {
            self.encoded_body_size = Some(*v)
        };
        if let Some(valuable::Value::F32(v)) = named_values.get_by_name("decoded_body_size") {
            self.decoded_body_size = Some(*v)
        };
        if let Some(valuable::Value::Mappable(headers)) = named_values.get_by_name("headers") {
            let mut visit = VisitHeaders {
                headers: HashMap::new(),
            };
            headers.visit(&mut visit);
            self.headers = Some(visit.headers)
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Socket {
    pub encrypted: Option<bool>,
    pub remote_address: Option<String>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Socket {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::Bool(v)) = named_values.get_by_name("encrypted") {
            self.encrypted = Some(*v)
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("remote_address") {
            self.remote_address = Some(v.to_string())
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Url {
    pub raw: Option<String>,
    pub protocol: Option<String>,
    pub full: Option<String>,
    pub hostname: Option<String>,
    pub port: Option<i32>,
    pub pathname: Option<String>,
    pub search: Option<String>,
    pub hash: Option<String>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Url {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("raw") {
            self.raw = Some(v.to_string())
        }
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("protocol") {
            self.protocol = Some(v.to_string())
        }
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("full") {
            self.full = Some(v.to_string())
        }
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("hostname") {
            self.hostname = Some(v.to_string())
        }
        if let Some(valuable::Value::I32(v)) = named_values.get_by_name("port") {
            self.port = Some(*v)
        }
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("pathname") {
            self.pathname = Some(v.to_string())
        }
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("search") {
            self.search = Some(v.to_string())
        }
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("hash") {
            self.hash = Some(v.to_string())
        }
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Request {
    pub body: Option<String>,
    pub headers: Option<Headers>,
    pub http_version: Option<String>,
    pub method: String,
    pub socket: Option<Socket>,
    pub url: Url,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Request {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("body") {
            self.body = Some(v.to_string())
        };
        if let Some(valuable::Value::Mappable(headers)) = named_values.get_by_name("headers") {
            let mut visit = VisitHeaders {
                headers: HashMap::new(),
            };
            headers.visit(&mut visit);
            self.headers = Some(visit.headers)
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("http_version") {
            self.http_version = Some(v.to_string())
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("method") {
            self.method = v.to_string()
        };
        if let Some(valuable::Value::Structable(socket)) = named_values.get_by_name("socket") {
            let mut visit_socket = Socket::default();
            socket.visit(&mut visit_socket);
            self.socket = Some(visit_socket);
        };
        if let Some(valuable::Value::Structable(url)) = named_values.get_by_name("url") {
            url.visit(&mut self.url);
        };
    }
}

#[derive(Default, Serialize, Debug)]
pub struct Page {
    pub referer: Option<String>,
    pub url: Option<String>,
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Queue {
    pub name: Option<String>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Queue {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("name") {
            self.name = Some(v.to_string())
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Age {
    pub ms: Option<i32>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Age {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::I32(v)) = named_values.get_by_name("ms") {
            self.ms = Some(*v)
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Message {
    pub queue: Option<Queue>,
    pub age: Option<Age>,
    pub body: Option<String>,
    pub headers: Option<Headers>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Message {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::Structable(v)) = named_values.get_by_name("queue") {
            let mut visit = Queue::default();
            v.visit(&mut visit);
            self.queue = Some(visit)
        };
        if let Some(valuable::Value::Structable(v)) = named_values.get_by_name("age") {
            let mut visit = Age::default();
            v.visit(&mut visit);
            self.age = Some(visit)
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("body") {
            self.body = Some(v.to_string())
        };
        if let Some(valuable::Value::Mappable(headers)) = named_values.get_by_name("headers") {
            let mut visit = VisitHeaders {
                headers: HashMap::new(),
            };
            headers.visit(&mut visit);
            self.headers = Some(visit.headers)
        };
    }
}

#[derive(Default, Serialize, Debug)]
pub struct TransactionContext {
    pub response: Option<Response>,
    pub request: Option<Request>,
    pub tags: Option<Tags>,
    pub user: Option<User>,
    pub page: Option<Page>,
    pub service: Option<Service>,
    pub message: Option<Message>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
pub enum Outcome {
    Success,
    Failure,
    Unknown,
}

#[derive(Default, Serialize, Debug)]
pub struct Transaction {
    pub timestamp: Option<u64>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub id: String,
    pub trace_id: String,
    pub parent_id: Option<String>,
    pub sample_rate: Option<f32>,
    pub span_count: SpanCount,
    pub context: Option<TransactionContext>,
    pub duration: f32,
    pub result: Option<String>,
    pub outcome: Option<Outcome>,
    pub marks: Option<Marks>,
    pub sampled: Option<bool>,
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct DestinationService {
    #[serde(rename = "type")]
    pub service_type: String,
    pub name: String,
    pub resource: String,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for DestinationService {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("service_type") {
            self.service_type = v.to_string()
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("name") {
            self.name = v.to_string()
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("resource") {
            self.resource = v.to_string()
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Destination {
    pub address: Option<String>,
    pub port: Option<i32>,
    pub service: Option<DestinationService>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Destination {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("address") {
            self.address = Some(v.to_string())
        };
        if let Some(valuable::Value::I32(v)) = named_values.get_by_name("port") {
            self.port = Some(*v)
        };
        if let Some(valuable::Value::Structable(v)) = named_values.get_by_name("service") {
            let mut visit = DestinationService::default();
            v.visit(&mut visit);
            self.service = Some(visit)
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Db {
    pub instance: Option<String>,
    pub link: Option<String>,
    pub statement: Option<String>,
    #[serde(rename = "type")]
    pub db_type: Option<String>,
    pub user: Option<String>,
    pub rows_affected: Option<i32>,
}
#[cfg(feature = "valuable")]
impl valuable::Visit for Db {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("instance") {
            self.instance = Some(v.to_string())
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("link") {
            self.link = Some(v.to_string())
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("statement") {
            self.statement = Some(v.to_string())
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("db_type") {
            self.db_type = Some(v.to_string())
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("user") {
            self.user = Some(v.to_string())
        };
        if let Some(valuable::Value::I32(v)) = named_values.get_by_name("rows_affected") {
            self.rows_affected = Some(*v)
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Http {
    pub url: Option<String>,
    pub status_code: Option<i32>,
    pub method: Option<String>,
    pub response: Option<Response>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Http {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("url") {
            self.url = Some(v.to_string())
        };
        if let Some(valuable::Value::I32(v)) = named_values.get_by_name("status_code") {
            self.status_code = Some(*v)
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("method") {
            self.method = Some(v.to_string())
        };
        if let Some(valuable::Value::Structable(v)) = named_values.get_by_name("response") {
            let mut visit = Response::default();
            v.visit(&mut visit);
            self.response = Some(visit)
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct Target {
    #[serde(rename = "type")]
    pub target_type: String,
    pub name: Option<String>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for Target {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("target_type") {
            self.target_type = v.to_string()
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("name") {
            self.name = Some(v.to_string())
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct ServiceOrigin {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for ServiceOrigin {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("id") {
            self.id = Some(v.to_string())
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("name") {
            self.name = Some(v.to_string())
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("version") {
            self.version = Some(v.to_string())
        };
    }
}

#[cfg_attr(feature = "valuable", derive(Valuable))]
#[derive(Default, Serialize, Debug)]
pub struct SpanService {
    pub agent: Option<Agent>,
    pub environment: Option<String>,
    pub framework: Option<Framework>,
    pub id: Option<String>,
    pub language: Option<Language>,
    pub name: Option<String>,
    pub node: Option<ServiceNode>,
    pub origin: Option<ServiceOrigin>,
    pub runtime: Option<Runtime>,
    pub target: Option<Target>,
    pub version: Option<String>,
}

#[cfg(feature = "valuable")]
impl valuable::Visit for SpanService {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        if let valuable::Value::Structable(v) = value {
            v.visit(self)
        }
    }

    fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
        if let Some(valuable::Value::Structable(v)) = named_values.get_by_name("agent") {
            let mut visit = Agent::default();
            v.visit(&mut visit);
            self.agent = Some(visit)
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("environment") {
            self.environment = Some(v.to_string())
        };
        if let Some(valuable::Value::Structable(v)) = named_values.get_by_name("framework") {
            let mut visit = Framework::default();
            v.visit(&mut visit);
            self.framework = Some(visit)
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("id") {
            self.id = Some(v.to_string())
        };
        if let Some(valuable::Value::Structable(v)) = named_values.get_by_name("language") {
            let mut visit = Language::default();
            v.visit(&mut visit);
            self.language = Some(visit)
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("name") {
            self.name = Some(v.to_string())
        };
        if let Some(valuable::Value::Structable(v)) = named_values.get_by_name("node") {
            let mut visit = ServiceNode::default();
            v.visit(&mut visit);
            self.node = Some(visit)
        };
        if let Some(valuable::Value::Structable(v)) = named_values.get_by_name("origin") {
            let mut visit = ServiceOrigin::default();
            v.visit(&mut visit);
            self.origin = Some(visit)
        };
        if let Some(valuable::Value::Structable(v)) = named_values.get_by_name("runtime") {
            let mut visit = Runtime::default();
            v.visit(&mut visit);
            self.runtime = Some(visit)
        };
        if let Some(valuable::Value::Structable(v)) = named_values.get_by_name("target") {
            let mut visit = Target::default();
            v.visit(&mut visit);
            self.target = Some(visit)
        };
        if let Some(valuable::Value::String(v)) = named_values.get_by_name("version") {
            self.version = Some(v.to_string())
        };
    }
}

#[derive(Default, Serialize, Debug)]
pub struct SpanContext {
    pub destination: Option<Destination>,
    pub db: Option<Db>,
    pub http: Option<Http>,
    pub tags: Option<Tags>,
    pub service: SpanService,
    pub message: Option<Message>,
}

#[derive(Default, Serialize, Debug)]
pub struct Span {
    pub timestamp: Option<u64>,
    #[serde(rename = "type")]
    pub span_type: String,
    pub subtype: Option<String>,
    pub id: String,
    pub transaction_id: Option<String>,
    pub trace_id: String,
    pub parent_id: String,
    pub child_ids: Option<Vec<String>>,
    pub start: Option<f32>,
    pub sample_rate: Option<f32>,
    pub action: Option<String>,
    pub outcome: Option<Outcome>,
    pub context: Option<SpanContext>,
    pub duration: f32,
    pub name: String,
    pub sync: Option<bool>,
}

#[derive(Default, Serialize, Debug)]
pub struct Log {
    pub level: Option<String>,
    pub logger_name: Option<String>,
    pub message: String,
    pub param_message: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Exception {
    pub code: Option<String>,
    pub message: Option<String>,
    pub module: Option<String>,
    #[serde(rename = "type")]
    pub exception_type: Option<String>,
    pub handled: Option<bool>,
}

#[derive(Default, Serialize, Debug)]
pub struct ErrorTransaction {
    pub sampled: Option<bool>,
    #[serde(rename = "type")]
    pub transaction_type: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Error {
    pub id: String,
    pub trace_id: Option<String>,
    pub transaction_id: Option<String>,
    pub parent_id: Option<String>,
    pub transaction: Option<ErrorTransaction>,
    pub context: Option<TransactionContext>,
    pub culprit: Option<String>,
    pub exception: Option<Exception>,
    pub log: Option<Log>,
}

#[cfg(all(test, feature = "valuable"))]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        let request = Request {
            headers: Some(HashMap::from([
                ("key".to_string(), "value".to_string()),
                ("key2".to_string(), "value2".to_string()),
            ])),
            method: "method".to_string(),
            url: Url {
                raw: Some("http://full:9090/path".to_string()),
                protocol: Some("http:".to_string()),
                hostname: Some("full:".to_string()),
                port: Some(9090),
                pathname: Some("/path".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        let encoded = request.as_value();
        let mut decoded = Request::default();
        valuable::visit(&encoded, &mut decoded);

        assert_eq!(request.headers, decoded.headers);
        assert_eq!(request.method, decoded.method);

        assert_eq!(request.url.raw, decoded.url.raw);
        assert_eq!(request.url.port, decoded.url.port);
        assert_eq!(request.url.pathname, decoded.url.pathname);
        assert_eq!(request.url.hostname, decoded.url.hostname);
    }
}
