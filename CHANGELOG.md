# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
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
- OpenAPI-generated request/response models
- Error types matching Plaid API taxonomy
