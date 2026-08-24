# Error Handling

`plaid-client-rs` uses a custom `PlaidError` type that covers:

- HTTP errors (network issues, timeouts)
- Plaid API errors (returned in the response body)
- Serialization errors

```rust
use plaid_client_rs::PlaidError;

match result {
    Ok(response) => println!("Success!"),
    Err(PlaidError::Api { error_code, .. }) => {
        eprintln!("Plaid error: {}", error_code);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```
