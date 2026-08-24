# plaid-client-rs

[![CI](https://github.com/johnathonfox/plaid-client-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/johnathonfox/plaid-client-rs/actions)
[![Docs](https://docs.rs/plaid-client-rs/badge.svg)](https://docs.rs/plaid-client-rs)
[![Crates.io](https://img.shields.io/crates/v/plaid-client-rs.svg)](https://crates.io/crates/plaid-client-rs)

A modern, async, type-safe Rust client for the [Plaid API](https://plaid.com/docs/api/).

## Features

- ✅ Async/await via `tokio` + `reqwest`
- ✅ Strongly-typed errors matching Plaid's error taxonomy
- ✅ Secret-safe (tokens wrapped with `secrecy`)
- 🚧 Full OpenAPI coverage (generated + hand-curated) — see ADR-0001
- 🚧 Middleware support (logging, retry, rate-limiting)
- 🚧 Mock-server integration tests

## Quick Start

```rust
use plaid_client_rs::{PlaidClient, Config, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PlaidClient::new(Config {
        client_id: "your-client-id".into(),
        secret: "your-secret".into(),
        environment: Environment::Sandbox,
        ..Config::default()
    })?;

    // Use the client...
    Ok(())
}
```

## Installation

```toml
[dependencies]
plaid-client-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Documentation

- [API Documentation](https://docs.rs/plaid-client-rs)
- [User Guide](https://johnathonfox.github.io/plaid-client-rs)
- [Architecture Decision Records](docs/adr/)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
