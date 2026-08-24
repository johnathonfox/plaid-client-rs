# ADR-0005: Incremental Hand-Curated Endpoint Coverage

## Status
Proposed

## Context
ADR-0001 chose OpenAPI generation as the source of truth for models, but
the generator pipeline (`scripts/generate_from_openapi.sh`) is still a stub.
The project needs usable endpoints now; wiring up `progenitor` or
`openapi-generator` and curating its full output is a large, separable effort.

## Decision
Build core endpoint coverage incrementally with hand-curated models, in
priority order of the canonical Plaid integration flow:

1. `/link/token/create` — start Link
2. `/item/public_token/exchange` — exchange after Link
3. `/auth/get` — Auth product data
4. `/accounts/balance/get` — balances
5. `/transactions/sync` — transaction sync (cursor-based)
6. `/webhook_verification_key/get` — required by ADR-0004

Models live in `src/models/<product>.rs`, hand-written per existing style
(`serde` derive, doc comments, `String` for timestamps). Each endpoint gets
wiremock integration tests. The OpenAPI generator pipeline remains planned
work under ADR-0001; when it lands, generated models will replace
hand-curated ones product-by-product. This ADR narrows, but does not
replace, ADR-0001.

## Consequences
- **Positive**: Delivers a usable client for the core flows without
  blocking on generator tooling.
- **Positive**: Hand-curated models set the idiom standard generated code
  must later match.
- **Negative**: Hand-written models can drift from Plaid's API until the
  generator pipeline lands.
- **Negative**: Some churn when generated models replace curated ones.

## Alternatives Considered
- **Set up the generator pipeline first**: Rejected for now — large up-front
  cost, and the curated models define the idiom target anyway.
- **Full hand-written client, drop ADR-0001**: Rejected — long-term
  maintenance burden ADR-0001 already ruled out.

## References
- [ADR-0001](0001-use-openapi-generation.md)
- [Plaid API docs](https://plaid.com/docs/api/)
