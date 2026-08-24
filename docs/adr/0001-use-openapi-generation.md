# ADR-0001: Use OpenAPI for Model Generation

## Status
Accepted

## Context
The Plaid API has hundreds of request/response types across many products.
Hand-writing all models is error-prone and will drift when Plaid updates their API.

## Decision
Use Plaid's official [OpenAPI specification](https://github.com/plaid/plaid-openapi)
as the source of truth for model generation. We will:

1. Download the spec as part of the build process
2. Generate base models using `progenitor` or `openapi-generator`
3. Hand-curate the output for Rust idioms (strong enums, Option<T>, etc.)
4. Check generated code into the repo (not generate at build time) for reproducibility

## Consequences
- **Positive**: Full API coverage, stays current with Plaid updates
- **Positive**: Reduces manual typing errors
- **Negative**: Generated code may need significant cleanup
- **Negative**: Adds a dependency on OpenAPI generator tooling

## Alternatives Considered
- **Hand-write all models**: Rejected — too much maintenance burden
- **Generate at compile time**: Rejected — adds build-time dependency, slower compiles

## References
- [Plaid OpenAPI GitHub](https://github.com/plaid/plaid-openapi)
- [progenitor](https://github.com/oxidecomputer/progenitor)
