use fxhash::FxHashMap;
use serde::Serialize;

/// Name and version of the programming language used.
#[derive(Serialize)]
pub struct Language {
    pub name: String,
    pub version: Option<String>,
}

/// Name and version of the language runtime running this service.
#[derive(Serialize)]
pub struct Runtime {
    pub name: String,
    pub version: String,
}

/// Name and version of the web framework used.
#[derive(Serialize)]
pub struct Framework {
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct Agent {
    pub name: String,
    pub version: String,
    pub ephemeral_id: Option<String>,
}

#[derive(Serialize)]
/// Unique meaningful name of the service node.
pub struct ServiceNode {
    pub configured_name: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct Service {
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
#[derive(Serialize)]
pub struct Process {
    /// Process ID of the service.
    pub pid: i32,
    /// Parent process ID of the service.
    pub ppid: Option<i32>,
    pub title: Option<String>,
    /// Command line arguments used to start this process.
    pub argv: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct Container {
    /// Container ID.
    pub id: String,
}

#[derive(Serialize)]
pub struct Node {
    /// Kubernetes node name.
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct Pod {
    /// Kubernetes pod name.
    pub name: Option<String>,
    /// Kubernetes pod uid.
    pub uid: Option<String>,
}

#[derive(Serialize)]
pub struct Kubernetes {
    /// Kubernetes namespace.
    pub namespace: Option<String>,
    pub pod: Option<Pod>,
    pub node: Option<Node>,
}

#[derive(Serialize)]
pub struct System {
    pub architecture: Option<String>,
    pub hostname: Option<String>,
    pub detected_hostname: Option<String>,
    pub configured_hostname: Option<String>,
    pub platform: Option<String>,
    pub container: Option<Container>,
    pub kubernetes: Option<Kubernetes>,
}

#[derive(Serialize)]
pub struct User {
    /// Identifier of the logged in user, e.g. the primary key of the user.
    pub id: Option<String>,
    /// Email of the logged in user.
    pub email: Option<String>,
    /// The username of the logged in user.
    pub username: Option<String>,
}

#[derive(Serialize)]
pub struct Account {
    /// Cloud account ID.
    pub id: Option<String>,
    /// Cloud account name.
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct Machine {
    /// Cloud instance/machine type.
    #[serde(rename = "type")]
    pub machine_type: Option<String>,
}

#[derive(Serialize)]
pub struct Project {
    /// Cloud project ID.
    pub id: Option<String>,
    /// Cloud project name.
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct Instance {
    /// Cloud instance/machine ID.
    pub id: Option<String>,
    /// Cloud instance/machine name.
    pub name: Option<String>,
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub(crate) struct Metadata {
    pub service: Service,
    pub process: Option<Process>,
    pub system: Option<System>,
    pub user: Option<User>,
    pub cloud: Option<Cloud>,
}

#[derive(Serialize)]
pub(crate) struct SpanCount {
    pub started: i32,
    pub dropped: Option<i32>,
}

#[derive(Serialize)]
pub(crate) struct Response {
    pub status_code: Option<i32>,
    pub transfer_size: Option<f32>,
    pub encoded_body_size: Option<f32>,
    pub decoded_body_size: Option<f32>,
    pub headers: Option<FxHashMap<String, String>>,
}

#[derive(Serialize)]
pub(crate) struct Socket {
    encrypted: Option<bool>,
    remote_address: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct Url {
    raw: Option<String>,
    protocol: Option<String>,
    full: Option<String>,
    hostname: Option<String>,
    port: Option<i32>,
    pathname: Option<String>,
    search: Option<String>,
    hash: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct Request {
    body: Option<String>,
    headers: Option<FxHashMap<String, String>>,
    http_version: Option<String>,
    method: String,
    socket: Option<Socket>,
    url: Url,
}

#[derive(Serialize)]
pub(crate) struct Page {
    referer: Option<String>,
    url: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct Queue {
    name: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct Age {
    ms: Option<i32>,
}

#[derive(Serialize)]
pub(crate) struct Message {
    queue: Option<Queue>,
    age: Option<Age>,
    body: Option<String>,
    headers: Option<FxHashMap<String, String>>,
}

#[derive(Serialize)]
pub(crate) struct Context {
    response: Option<Response>,
    request: Option<Request>,
    tags: Option<FxHashMap<String, String>>,
    user: Option<User>,
    page: Option<Page>,
    service: Option<Service>,
    message: Option<Message>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Outcome {
    Success,
    Failure,
    Unknown,
}

#[derive(Serialize)]
pub(crate) struct Transaction {
    timestamp: Option<i32>,
    name: Option<String>,
    #[serde(rename = "type")]
    transaction_type: Option<String>,
    id: String,
    trace_id: String,
    parent_id: Option<String>,
    sample_rate: Option<f32>,
    span_count: SpanCount,
    context: Option<Context>,
    duration: f32,
    result: Option<String>,
    outcome: Option<Outcome>,
    marks: Option<FxHashMap<String, FxHashMap<String, f32>>>,
    sampled: Option<bool>,
}
