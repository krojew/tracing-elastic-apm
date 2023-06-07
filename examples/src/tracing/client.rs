pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::{greeter_client::GreeterClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::FmtSubscriber::builder()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_file(true)
                .with_line_number(true)
        )
        .with_max_level(tracing::Level::INFO)
        .init();

    say_hi("Bob".into()).await?;

    Ok(())
}

// #[tracing::instrument]
async fn say_hi(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50091").await?;


    let request = tonic::Request::new(HelloRequest { name });

    tracing::info!(
        message = "Sending request.",
        request = %request.get_ref().name
    );

    let response = client.say_hello(request).await?;

    tracing::info!(
        message = "Got a response.",
        response = %response.get_ref().message
    );

    Ok(())
}