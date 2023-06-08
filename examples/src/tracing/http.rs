use std::{net::SocketAddr,};

use axum::{routing::{get, post} , Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tracing::{metadata::LevelFilter};
use tracing_elastic_apm::{middleware::{inject_trace_context, apm_tracing_layer}, interceptor::{TonicTraceInterceptor}};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter};


use tonic::{
    codegen::InterceptedService,
    transport::{Channel, Endpoint},
};


#[derive(Deserialize,Default,Serialize,Debug)]
struct SignInRequest {
    email: String,
    password: String
}

#[derive(Deserialize,Default,Serialize)]
struct Token {
    token: String,
    expired_at: u64
}

#[derive(Deserialize,Default,Serialize)]
struct AccessAndRefreshToken {
    access_token: Token,
    refresh_token: Token
}

// the input to our `create_user` handler
#[derive(Deserialize,Default)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}


pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::{greeter_client::GreeterClient};

use crate::hello_world::{CreateUserRequest, CreateUserReply};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    let apm_layer = tracing_elastic_apm::new_layer(
        tracing_elastic_apm::apm::config::Config::from_env()
    ).unwrap();
    let filter = EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy();

    let stdout = tracing_subscriber::fmt::layer().pretty().compact().with_level(true);
    let subscriber = tracing_subscriber::registry().with(filter).with(stdout).with(apm_layer);
    subscriber.init();

    
    let rest_addr = SocketAddr::from(([0, 0, 0, 0], 50092));
    tracing::info!("Setting up the HTTP server {}",rest_addr);
    // build our application with a route
    let app = axum::Router::new()
        .route("/", get(root))
        .route("/baidu", get(baidu))
        .route("/users/grpc", post(create_user_grpc))
        .layer(
            ServiceBuilder::new()
                .layer(apm_tracing_layer())
                // .map_request(update_span_path)
                .map_response(inject_trace_context),
        );

    let _http_server = axum::Server::bind(&rest_addr)
        .serve(app.into_make_service()).await.unwrap();
    
    Ok(())
}


// basic handler that responds with a static string
async fn root() -> String {
    let result = huawei().await;
    result
}
#[tracing::instrument(fields(url= "https://huawei.com"))]
async fn huawei() -> String {
    let body = reqwest::get("https://huawei.com").await.unwrap();
    body.text().await.unwrap()
}

async fn baidu() -> String {
    let body = reqwest::get("https://m.baidu.com").await.unwrap();
    
    body.text().await.unwrap()
}

async fn create_user_grpc(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {

    

    let request = tonic::Request::new(CreateUserRequest {
        username: payload.username
    });

    let response = grpc_create_user(request).await.unwrap();
    let res = response.into_inner();
    let result = User {
        id: res.id as u64,
        username: res.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(result.into()))
}

#[tracing::instrument]
async fn grpc_create_user(request:tonic::Request<CreateUserRequest>) -> tonic::Result<tonic::Response<CreateUserReply>, tonic::Status>  {
    let channel = Endpoint::from_static("grpc://localhost:50091")
        .connect()
        .await.unwrap();

    let mut client: GreeterClient<InterceptedService<Channel, TonicTraceInterceptor>> =
        GreeterClient::with_interceptor(channel, TonicTraceInterceptor);
    let response: tonic::Response<CreateUserReply> = client.create_user(request).await.unwrap();
    Ok(response)
}





