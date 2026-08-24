use plaid_client_rs::models::link::{LinkTokenCreateRequest, LinkTokenUser};
use plaid_client_rs::{Config, Environment, PlaidClient};
use secrecy::{ExposeSecret, SecretString};

/// The full sandbox Link flow: create a Link token, simulate a user
/// completing Link with a sandbox public token, and exchange it for an
/// access token.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PlaidClient::new(Config {
        client_id: SecretString::from(std::env::var("PLAID_CLIENT_ID")?),
        secret: SecretString::from(std::env::var("PLAID_SECRET")?),
        environment: Environment::Sandbox,
        ..Config::default()
    })?;

    // 1. Create a Link token to initialize Plaid Link in your frontend.
    let link_token = client
        .link_token_create(&LinkTokenCreateRequest {
            client_name: "My App".to_string(),
            language: "en".to_string(),
            country_codes: vec!["US".to_string()],
            products: vec!["auth".to_string()],
            user: LinkTokenUser {
                client_user_id: "user-123".to_string(),
            },
        })
        .await?;
    println!("Link token created: {}", link_token.link_token);

    // 2. In sandbox only: simulate the user completing Link by creating
    //    a public token directly.
    let public_token = client
        .sandbox_public_token_create("ins_109508", &["auth".to_string()])
        .await?;
    println!("Public token created: {}", public_token.public_token);

    // 3. Exchange the public token for an access token.
    let exchange = client
        .item_public_token_exchange(&public_token.public_token)
        .await?;
    println!(
        "Access token obtained for item {}: {}",
        exchange.item_id,
        exchange.access_token.expose_secret()
    );

    Ok(())
}
