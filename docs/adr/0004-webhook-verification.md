# ADR-0004: Webhook Signature Verification via ES256 JWT

## Status
Proposed

## Context
Plaid signs every webhook delivery with a JWS (JWT, ES256) sent in the
`Plaid-Verification` header. Receivers must fetch the verification key (JWK)
from `/webhook_verification_key/get` using the `kid` in the JWT header,
verify the signature, check the token age (issued within 5 minutes), and
compare the `request_body_sha256` claim against the SHA-256 hex of the raw
request body. Our `src/webhooks/` module is currently empty.

## Decision
Implement verification in `src/webhooks/`:

- `PlaidClient::verify_webhook(jwt, raw_body) -> Result<WebhookEnvelope, PlaidError>`
  (implemented in `src/endpoints/webhooks.rs`, logic in `src/webhooks/`):
  decode the JWT header for `kid`, fetch the JWK through the normal
  authenticated request pipeline, verify the ES256 signature with
  `jsonwebtoken`, enforce the 5-minute `iat` window, and compare the
  `request_body_sha256` claim to the SHA-256 hex of the raw body.
- `WebhookEnvelope`: typed common fields (`webhook_type`, `webhook_code`,
  `item_id`) with a `#[serde(flatten)]` catch-all for product-specific data.
- New dependencies: `jsonwebtoken` (ES256 + JWK support), `sha2`, `hex`.
- Verification keys are fetched per call; caching is left to a future ADR
  if key-fetch latency matters.

## Consequences
- **Positive**: Follows Plaid's documented verification procedure exactly;
  rejection of forged or replayed webhooks is enforced by default.
- **Positive**: `jsonwebtoken` is widely used and supports JWK decoding
  directly.
- **Negative**: Adds `jsonwebtoken`/`sha2`/`hex` to the dependency tree.
- **Negative**: No key caching — one extra HTTP round trip per webhook.

## Alternatives Considered
- **Manual JWS verification with `p256` + `sha2`**: Rejected — re-implements
  JWT parsing/validation that `jsonwebtoken` already gets right.
- **HMAC shared-secret scheme**: Not applicable — Plaid uses asymmetric
  JWS, not HMAC.

## References
- [Plaid webhook verification](https://plaid.com/docs/api/webhooks/webhook-verification/)
- [jsonwebtoken crate](https://docs.rs/jsonwebtoken)
