# tracing-elastic-apm

[Elastic APM](https://www.elastic.co/apm) tracing layer. Uses the native ingest API.

This repository is a fork of https://github.com/krojew/tracing-elastic-apm including following improved features:

 - load config from Elastic env as ELASTIC_APM_*
 - HTTP & GRPC Middleware for Axum
 - integrated with built-in metric
 - distributed tracing by TokioTracingInterceptor
 - ignore health check url by ELASTIC_APM_IGNORE_URLS

## Usage

Create a new tracing Layer:

```rust
    let apm_layer = tracing_elastic_apm::new_layer(
        tracing_elastic_apm::apm::config::Config::from_env()
    ).unwrap();
```

Register the layer:

```rust
    let filter = EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy();
    let stdout = tracing_subscriber::fmt::layer().pretty().compact().with_level(true);
    let subscriber = tracing_subscriber::registry().with(filter).with(stdout).with(apm_layer);
    subscriber.init();  
```

Register Middleware:
```rust
    Server::builder()
        .layer(ServiceBuilder::new().layer(apm_tracing_layer_grpc()))
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
```


Take a look at `Config` for more configuration options.

## Supported feature flags

- `default-tls` _(enabled by default)_ - use default TLS backend.
- `rustls-tls` - use Rustls TLS backend.

Please see corresponding flags in the `reqwest` library for more information:
[https://docs.rs/reqwest/0.11.2/reqwest/#optional-features](https://docs.rs/reqwest/0.11.2/reqwest/#optional-features)

## Async and time measurements

APM doesn't support the notion of idle time and only tracks actual span durations. Async code naturally interleaves
spans at await points, which means span start time + duration might be lower than actual span end time as measured by a
wall clock. That in turn means child spans in APM might sometimes start after the parent span start time + duration.
