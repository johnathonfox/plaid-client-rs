# Contributing

Thank you for your interest in contributing to `plaid-client-rs`!

## Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` — New features
- `fix:` — Bug fixes
- `docs:` — Documentation changes
- `refactor:` — Code restructuring
- `test:` — Test additions/changes
- `chore:` — Maintenance tasks

## Architecture Decisions

Significant architectural changes require an ADR in `docs/adr/`.
See the ADR registry at `docs/adr/README.md`.

## Testing

```bash
cargo test --all-features
cargo test --test integration
```

## Code Quality

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo deny check
cargo audit
```
