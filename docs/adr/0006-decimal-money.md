# ADR-0006: Decimal Money via rust_decimal

## Status
Accepted

## Context
Monetary fields (`Transaction.amount`, `Balances`, holdings, payment
amounts, …) were modeled as `f64`. Binary floats cannot represent common
decimal currency values exactly, which is a correctness hazard for a
financial client (rounding drift in sums and comparisons). Plaid's API
returns currency values as JSON decimal numbers.

## Decision
Use [`rust_decimal::Decimal`](https://docs.rs/rust_decimal) for all
monetary fields. Each field is annotated
`#[serde(with = "rust_decimal::serde::float")]` (or `float_option`) so
the JSON wire format stays numeric (matching Plaid's schema) — the
`serde-with-float` feature only provides these helpers; the stock impl
serializes as a string. `Decimal` is re-exported at the crate root so
users don't need a direct dependency.

- Money fields (balances, amounts, values, prices, quantities) → `Decimal`,
  serialized as JSON numbers. Required fields use
  `#[serde(with = "rust_decimal::serde::float")]`; optional fields use an
  in-house `models::common::decimal_option_json` helper, because
  rust_decimal's `float_option` breaks under `#[serde(flatten)]` and drops
  the implicit default for missing keys.
- Percentages (APR, interest rates) stay `f64` — they are not money and
  float precision is adequate.
- Transfer amounts stay `String` — that is Plaid's wire format for the
  Transfer product.

## Consequences
- **Positive**: Exact decimal arithmetic and comparison for money.
- **Positive**: Matches established practice: plaid-java uses
  `BigDecimal`, and the Rust `rplaid` client offers the same conversion
  via a `decimal` feature.
- **Negative**: Adds the `rust_decimal` dependency.
- **Negative**: `serde-with-float` converts through `f64` at the JSON
  boundary, so extreme values (>2^53 mantissa) can lose precision on
  the wire; acceptable for Plaid's data ranges.

## Alternatives Considered
- **Keep `f64`** (plaid-node, plaid-python, and generated Rust clients
  do this): Rejected — inherits binary-float rounding bugs for money.
- **`bigdecimal` crate**: Rejected — arbitrary precision is unnecessary
  here; `rust_decimal` is faster and the de facto Rust standard.
- **Minor units (integer cents)**: Rejected — requires a currency
  exponent lookup per value; Plaid already sends decimal units.

## References
- [rust_decimal](https://docs.rs/rust_decimal)
- [rplaid `decimal` feature](https://docs.rs/rplaid)
- [Plaid API overview — currency values](https://plaid.com/docs/api/)
