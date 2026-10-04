# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `Account::mask`, `Balances::limit`, `Balances::unofficial_currency_code`, and `Item::error` (`ItemError`, e.g. `ITEM_LOGIN_REQUIRED`)
- Repository scaffolding and CI pipeline
- Core `PlaidClient` with async HTTP support
- Request pipeline: JSON POST with `client_id`/`secret` body injection and Plaid error-body mapping to `PlaidError::Api`
- Optional `Config::base_url` override for pointing the client at a mock server
- Middleware chain (`Config::middleware`) with `TracingLogger`, `RetryPolicy` (exponential backoff on 429/5xx), and `RateLimiter` built-ins — ADR-0003
- Webhook signature verification (`PlaidClient::verify_webhook`) using ES256 JWTs and `/webhook_verification_key/get` — ADR-0004; adds `PlaidError::WebhookVerification`
- Endpoints: `link_token_create`, `item_public_token_exchange`, `auth_get`, `accounts_balance_get`, `transactions_sync`, `webhook_verification_key_get` — ADR-0005
- `sandbox_public_token_create` endpoint (`/sandbox/public_token/create`)
- Expanded endpoint coverage per ADR-0005: `item_get`, `item_remove`, `institutions_get_by_id`, `institutions_search`, `identity_get`, `investments_holdings_get`, `investments_transactions_get`, `liabilities_get`, `transfer_authorization_create`, `transfer_create`, `transfer_get`, `payment_initiation_recipient_create`, `payment_initiation_recipient_get`, `payment_initiation_payment_create`, `payment_initiation_payment_get`, `payment_initiation_payment_list`, `accounts_get`, `sandbox_item_fire_webhook`, `sandbox_item_reset_login`
- Mock-server integration tests covering endpoints, middleware, and webhook verification
- Architecture diagram (`docs/diagrams/architecture.{mmd,png}`)
- Hand-curated request/response models per ADR-0005 (OpenAPI generation per ADR-0001 is still pending)
- Error types matching Plaid API taxonomy
- Daily CI watch for upstream OpenAPI spec changes (`.github/workflows/openapi-spec-watch.yml`)

### Changed
- **Money decodes exactly** (ADR-0006): decimal fields read the raw JSON token instead of passing through `f64`, which corrupted values past ~15 significant digits (e.g. fractional share quantities). The middleware chain now carries responses as `middleware::ResponseBody` (`Box<RawValue>`) instead of `serde_json::Value`, so custom `Middleware` impls return that type. `IdentityAccount` decodes by hand rather than through `#[serde(flatten)]`.
- `PlaidError::Api` now carries the HTTP `status`; non-Plaid error bodies map to the new `PlaidError::UnexpectedStatus`, and 2xx decode failures to `PlaidError::Decode`
- `RetryPolicy` retries by HTTP status (429/5xx) and connection errors only; timeouts and decode failures are no longer retried, so non-idempotent POSTs can't be duplicated by replay
- `middleware::Request` is no longer `Clone` and its `Debug` redacts the body (it carries injected credentials)
- `reqwest` now builds with `default-features = false` (rustls only, per ADR-0002); `tokio` `time`/`sync` features are declared explicitly
- The Link flow example no longer prints the access token
- Webhook body-hash comparison is now constant-time, and tokens dated in the future (beyond a 30s clock-skew allowance) are rejected per Plaid's freshness rule
- Transfer endpoints take a shared `TransferParams` struct (no more 7–10 positional arguments); `transfer_type`/`network`/`ach_class` are now typed enums (`TransferType`, `TransferNetwork`, `TransferAchClass`) with an `Unknown` fallback for forward compatibility
- `AccountsGetResponse.item` reuses `models::item::Item` (`AccountsGetItem` removed); `IdentityAccount` flattens the shared `Account` model instead of duplicating its fields
- Monetary fields are now `rust_decimal::Decimal` (re-exported at the crate root) instead of `f64` — balances, amounts, prices, values, quantities, payment amounts — serialized as JSON numbers per ADR-0006; percentages stay `f64`
