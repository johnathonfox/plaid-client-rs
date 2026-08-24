# Authentication

All Plaid API requests require a `client_id` and `secret`. These are provided
when you create an account on the [Plaid Dashboard](https://dashboard.plaid.com).

```rust
use plaid_client_rs::{Config, Environment, PlaidClient};
use secrecy::SecretString;

let client = PlaidClient::new(Config {
    client_id: SecretString::from(std::env::var("PLAID_CLIENT_ID").unwrap()),
    secret: SecretString::from(std::env::var("PLAID_SECRET").unwrap()),
    environment: Environment::Sandbox,
    ..Config::default()
}).unwrap();
```

> **Security**: Never hard-code your credentials. Use environment variables
> or a secrets manager.
