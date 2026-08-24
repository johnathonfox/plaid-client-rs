use plaid_client_rs::{Config, Environment, PlaidClient};
use secrecy::SecretString;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PlaidClient::new(Config {
        client_id: SecretString::from(std::env::var("PLAID_CLIENT_ID")?),
        secret: SecretString::from(std::env::var("PLAID_SECRET")?),
        environment: Environment::Sandbox,
        ..Config::default()
    })?;

    // TODO: Implement /link/token/create call
    println!(
        "Link token flow example — implement me! (client for {})",
        client.base_url()
    );

    Ok(())
}
