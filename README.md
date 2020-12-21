# tracing-elastic-apm

[![crates.io version](https://img.shields.io/crates/v/tracing-elastic-apm.svg)](https://crates.io/crates/tracing-elastic-apm)
[![Documentation (latest release)](https://docs.rs/tracing-elastic-apm/badge.svg)](https://docs.rs/tracing-elastic-apm/)

[Elastic APM](https://www.elastic.co/guide/en/apm/get-started/7.10/index.html) tracing layer. Uses the native ingest API.

## Usage

Add the crate to your Cargo.toml file:

```toml
tracing-elastic-apm = "desired version"
```

Create a new tracing Layer:

```rust
let layer = tracing_elastic_apm::new_layer(
    "ServiceName".to_string(), 
    tracing_elastic_apm::Config::new("APM address".to_string())
);
```

Register the layer:

```rust
tracing_subscriber::registry()
    .with(layer)
    .init();
```

Take a look at `Config` for more configuration options.
