# ADR-0002: Tokio + Reqwest as Async Stack

## Status
Accepted

## Context
We need an async HTTP client and runtime. Rust has several options.

## Decision
Use `tokio` as the async runtime and `reqwest` as the HTTP client.

- `tokio`: De facto standard, excellent ecosystem, `async fn` in traits support
- `reqwest`: High-level, ergonomic, built on `hyper`, supports `rustls-tls`

## Consequences
- **Positive**: Largest ecosystem, most examples, best documentation
- **Positive**: `reqwest` handles connection pooling, redirects, JSON automatically
- **Negative**: Pulls in many transitive dependencies
- **Negative**: Opinionated — users cannot easily swap to `surf` or `hyper` directly

## Alternatives Considered
- **`hyper` directly**: Rejected — too low-level for this use case
- **`surf` + `async-std`**: Rejected — smaller ecosystem, less mature
- **`ureq` (sync)**: Rejected — Plaid API is latency-tolerant but async is expected in 2026

## References
- [tokio.rs](https://tokio.rs)
- [reqwest docs](https://docs.rs/reqwest)
