# ADR-0003: Hand-Rolled Middleware Chain Around the Request Pipeline

## Status
Proposed

## Context
The README roadmap calls for middleware support (logging, retry, rate-limiting).
Cross-cutting behavior needs to wrap the client's request execution without
each endpoint re-implementing it. The ecosystem option is the
`reqwest-middleware` family of crates; the alternative is a small in-house
abstraction.

## Decision
Implement a minimal onion-style middleware chain in `src/middleware/`
around our own request shape (path + JSON body in, JSON value out):

- `Middleware` trait (`async_trait`): `handle(&self, req, next) -> Result<Value, PlaidError>`.
- `Next` runs the rest of the chain; the terminal sender lives in
  `PlaidClient` and performs the actual HTTP POST.
- Built-ins: `TracingLogger` (request/response/retry observability),
  `RetryPolicy` (exponential backoff on 429 and 5xx), `RateLimiter`
  (minimum interval between requests, enforced by a mutex-held timestamp).
- Middleware is registered on `Config::middleware` (`Vec<Arc<dyn Middleware>>`),
  defaulting to empty — no behavior change for existing users.

## Consequences
- **Positive**: No new framework dependency beyond `async-trait`; the chain
  operates on our own types, so it stays simple and fully testable with
  wiremock.
- **Positive**: Retry/rate-limit/logging compose in user-chosen order.
- **Negative**: Not interoperable with the `reqwest-middleware` ecosystem;
  if users demand tower-style services later, we may need an adapter.
- **Negative**: `async fn` in traits is not object-safe on our MSRV
  (1.75), so we depend on `async-trait`.

## Alternatives Considered
- **`reqwest-middleware` + `reqwest-retry` + `reqwest-tracing`**: Rejected —
  heavier dependency tree, and the abstraction operates on raw `reqwest`
  types rather than our typed request pipeline.
- **Hook-only trait (before/after)**: Rejected — retry and rate limiting
  cannot be expressed as passive hooks; they must wrap the send.
- **No trait, config-only knobs**: Rejected — does not provide the
  extension point the roadmap promises.

## References
- [async-trait crate](https://docs.rs/async-trait)
- [reqwest-middleware](https://docs.rs/reqwest-middleware)
