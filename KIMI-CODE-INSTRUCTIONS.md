# Instructions for Kimi Code

Working agreements for agent sessions in this repo (`plaid-client-rs`, an async Rust
client for the Plaid API, living at `github.com/johnathonfox/plaid-client-rs`).

## Start here

- Read `AGENTS.md` — it points at `docs/agents/` (issue tracker, triage
  labels, domain docs). Issues are GitHub Issues; use the `gh` CLI.
- Read `CONTEXT.md` and relevant `docs/adr/` entries before design work, if
  they exist. Significant architectural changes require a new ADR (use
  `docs/adr/template.md`, register it in `docs/adr/README.md`).

## Build, test, verify

```bash
cargo check --all-targets
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all --check
```

A change is not done until all four pass. `cargo deny check` and
`cargo audit` run in CI; run them locally when touching dependencies.

## Conventions

- **Commits**: Conventional Commits (`feat:`, `fix:`, `docs:`, `refactor:`,
  `test:`, `chore:`). See `CONTRIBUTING.md`.
- **Secrets**: credentials and tokens are always `secrecy::SecretString`.
  Never add a `Clone` impl, a `Debug` that exposes the inner value, or log
  them via `tracing`.
- **Errors**: library code returns `PlaidError` (see `src/error.rs`); don't
  introduce `anyhow` in library code.
- **API surface**: models live in `src/models/`, generated from Plaid's
  OpenAPI spec per ADR-0001 (`scripts/generate_from_openapi.sh`); until the
  generator pipeline lands, models are hand-curated incrementally per
  ADR-0005. Hand-edit generated code only for Rust idioms, and say so in
  the commit message.
- **Middleware**: cross-cutting request behavior (logging, retry, rate
  limiting) goes through the middleware chain in `src/middleware/`
  (ADR-0003), registered on `Config::middleware` — not bolted into
  endpoint code.
- **Webhooks**: webhook handling must verify Plaid's ES256 JWT via
  `PlaidClient::verify_webhook` (ADR-0004); never parse webhook bodies
  without verification.
- **Async stack**: `tokio` + `reqwest` (rustls) per ADR-0002. Don't add a
  second runtime or HTTP client.

## Housekeeping

- Keep the `## Agent skills` block in `AGENTS.md` and the files under
  `docs/agents/` in sync when conventions change.
- Keep `CHANGELOG.md` current (Keep a Changelog format, semver).
