use std::{net::SocketAddr,};

use axum::{routing::{get, post} , Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tracing::{Span, metadata::LevelFilter};
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
        tracing_elastic_apm::apm::config::Config::from_env().with_ignore_urls("/baidu".to_string())
    ).unwrap();
    let filter = EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy();

    let stdout = tracing_subscriber::fmt::layer().pretty().compact().with_level(true);
    let subscriber = tracing_subscriber::registry().with(filter).with(stdout).with(apm_layer);
    // tracing::subscriber::set_global_default(subscriber);
    // tracing_subscriber::registry().init();
    subscriber.init();

    // println!(">>>subscriber={:?}",subscriber);

    
    let rest_addr = SocketAddr::from(([0, 0, 0, 0], 50092));
    tracing::info!("Setting up the HTTP server {}",rest_addr);
    // let rest = axum::Router::new()
    //     .merge(v2::http::router());

    // build our application with a route
    let app = axum::Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/baidu", get(baidu))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/users/grpc", post(create_user_grpc))
        .route("/auth/email/sign_in", post(email_sign_in))
        .layer(
            ServiceBuilder::new()
                .layer(apm_tracing_layer())
                // .map_request(update_span_path)
                .map_response(inject_trace_context),
        );

    // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

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

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = mock_create_user(payload);

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}


async fn create_user_grpc(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {

    let channel = Endpoint::from_static("grpc://localhost:50091")
        .connect()
        .await.unwrap();

    let mut client: GreeterClient<InterceptedService<Channel, TonicTraceInterceptor>> =
        GreeterClient::with_interceptor(channel, TonicTraceInterceptor);

    let request = tonic::Request::new(CreateUserRequest {
        username: payload.username
    });

    let response: tonic::Response<CreateUserReply> = client.create_user(request).await.unwrap();
    let r = response.into_inner();
    
    let result = User {
        id: r.id as u64,
        username: r.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(result))
}

#[tracing::instrument]
async fn post_sign_in(payload: SignInRequest) ->  Result<reqwest::Response,reqwest::Error>{
    let client = reqwest::Client::new();
    let json_request = serde_json::to_string(&payload).unwrap();
    let response = client.post("https://d.test-coral.app/v2/auth/email/sign_in")
    .header("Content-Type", "application/json")
    .body(json_request)
    .send()
    .await;
    response    
}

async fn email_sign_in(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<SignInRequest>,
) -> (StatusCode, Json<AccessAndRefreshToken>) {
    
    
    let res = post_sign_in(payload).await;
    match res {
        Ok(result) => {
            
            if result.status().is_success() {
                let json = result.text().await.unwrap();
                let token_payload: AccessAndRefreshToken = serde_json::from_str(json.as_str()).unwrap();
                return (StatusCode::OK, Json(token_payload))
            }else {
                tracing::error!("error={:?}",result.status());
                return (result.status(), Json(AccessAndRefreshToken::default()))
            }
        },
        Err(e) => {
            tracing::error!("{:?}",e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(AccessAndRefreshToken::default()))
        }
    }
}


#[tracing::instrument(skip(payload), 
fields(span_type = "external", span.subtype= "http",http.url = "https://sgr.test-coral.app", destination.service.name="https://sgr.test-coral.app", destination.service_type="external", destination.address = "sgr.test-coral.app", destination.port = 443))]
fn mock_create_user(payload:CreateUser) -> User {
    let span = &Span::current();
    span.record("http.status_code", 200);
    println!(">>>>>>>>>span={:?}", span);
    User {
        id: 1337,
        username: payload.username,
    }
}





