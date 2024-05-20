//! APM data model.

use fxhash::FxHashMap;
use serde::Serialize;
use serde_json::Value;

pub type Tags = FxHashMap<String, Value>;

/// Name and version of the programming language used.
#[derive(Default, Serialize, Debug)]
pub struct Language {
    pub name: String,
    pub version: Option<String>,
}

/// Name and version of the language runtime running this service.
#[derive(Default, Serialize, Debug)]
pub struct Runtime {
    pub name: String,
    pub version: String,
}

/// Name and version of the web framework used.
#[derive(Default, Serialize, Debug)]
pub struct Framework {
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Agent {
    pub name: String,
    pub version: String,
    pub ephemeral_id: Option<String>,
}

#[derive(Default, Serialize, Debug)]
/// Unique meaningful name of the service node.
pub struct ServiceNode {
    pub configured_name: Option<String>,
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

#[derive(Default, Serialize, Debug)]
pub struct Response {
    pub status_code: Option<i32>,
    pub transfer_size: Option<f32>,
    pub encoded_body_size: Option<f32>,
    pub decoded_body_size: Option<f32>,
    pub headers: Option<FxHashMap<String, String>>,
}

#[derive(Default, Serialize, Debug)]
pub struct Socket {
    pub encrypted: Option<bool>,
    pub remote_address: Option<String>,
}

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

#[derive(Default, Serialize, Debug)]
pub struct Request {
    pub body: Option<String>,
    pub headers: Option<FxHashMap<String, String>>,
    pub http_version: Option<String>,
    pub method: String,
    pub socket: Option<Socket>,
    pub url: Url,
}

#[derive(Default, Serialize, Debug)]
pub struct Page {
    pub referer: Option<String>,
    pub url: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Queue {
    pub name: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct Age {
    pub ms: Option<i32>,
}

#[derive(Default, Serialize, Debug)]
pub struct Message {
    pub queue: Option<Queue>,
    pub age: Option<Age>,
    pub body: Option<String>,
    pub headers: Option<FxHashMap<String, String>>,
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
    pub marks: Option<FxHashMap<String, FxHashMap<String, f32>>>,
    pub sampled: Option<bool>,
}

#[derive(Default, Serialize, Debug)]
pub struct DestinationService {
    #[serde(rename = "type")]
    pub service_type: String,
    pub name: String,
    pub resource: String,
}

#[derive(Default, Serialize, Debug)]
pub struct Destination {
    pub address: Option<String>,
    pub port: Option<i32>,
    pub service: Option<DestinationService>,
}

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

#[derive(Default, Serialize, Debug)]
pub struct Http {
    pub url: Option<String>,
    pub status_code: Option<i32>,
    pub method: Option<String>,
    pub response: Option<Response>,
}

#[derive(Default, Serialize, Debug)]
pub struct Target {
    #[serde(rename = "type")]
    pub target_type: String,
    pub name: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct ServiceOrigin {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
}

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
