//! Common identifier types shared across Plaid products.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize, Serializer};

/// Serialize a borrowed [`SecretString`] field as a plain string.
///
/// `SecretString` has no `Serialize` impl (its inner `str` is unsized),
/// so secret-bearing request models use this via `serialize_with`.
/// The field is `&'a SecretString`, hence the double reference (serde
/// passes a reference to the field).
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn serialize_secret_string<S>(
    secret: &&SecretString,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(secret.expose_secret())
}

/// A Plaid item ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemId(pub String);

/// A Plaid account ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(pub String);

/// A Plaid request ID (included in all responses for support).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestId(pub String);

/// Serde helpers for optional [`Decimal`](rust_decimal::Decimal) money
/// fields (ADR-0006).
///
/// `rust_decimal::serde::float_option` breaks under `#[serde(flatten)]`
/// (serde buffers flattened content and calls `visit_some` with nulls),
/// so optional decimal fields use this module instead. Wire format is a
/// JSON number (or `null`), same as Plaid's schema.
pub(crate) mod decimal_option_json {
    use rust_decimal::Decimal;
    use serde::{Deserialize, Deserializer, Serializer};

    // serde's `with` convention passes a reference to the field.
    #[allow(clippy::ref_option)]
    pub fn serialize<S>(value: &Option<Decimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        rust_decimal::serde::float_option::serialize(value, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Option::<serde_json::Value>::deserialize(deserializer)? {
            None | Some(serde_json::Value::Null) => Ok(None),
            Some(serde_json::Value::Number(number)) => {
                let float = number
                    .as_f64()
                    .ok_or_else(|| serde::de::Error::custom("amount is not a finite number"))?;
                Decimal::try_from(float)
                    .map(Some)
                    .map_err(serde::de::Error::custom)
            }
            // Tolerate string-encoded decimals, rust_decimal's own
            // default wire format.
            Some(serde_json::Value::String(text)) => text
                .parse::<Decimal>()
                .map(Some)
                .map_err(serde::de::Error::custom),
            Some(other) => Err(serde::de::Error::custom(format!(
                "unexpected JSON value for a decimal amount: {other}"
            ))),
        }
    }
}
