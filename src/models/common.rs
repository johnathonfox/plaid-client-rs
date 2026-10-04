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

/// Parse one raw JSON scalar exactly: a number token (`182.31`, `1.5e-7`)
/// or a quoted numeric string (`"182.31"`). Never passes through `f64`.
fn parse_raw_decimal(raw: &str) -> Result<rust_decimal::Decimal, String> {
    let text = raw.trim().trim_matches('"');
    let parsed = if text.contains(['e', 'E']) {
        rust_decimal::Decimal::from_scientific(text)
    } else {
        rust_decimal::Decimal::from_str_exact(text)
    };
    parsed.map_err(|e| format!("invalid decimal amount {text:?}: {e}"))
}

/// Serde helpers for required [`Decimal`](rust_decimal::Decimal) money
/// fields (ADR-0006).
///
/// Deserializing reads the raw JSON token, so `0.1` is exactly `0.1`. This
/// needs the `serde_json` deserializer itself: it does not work through a
/// [`serde_json::Value`] (whose numbers are already `f64`) or under
/// `#[serde(flatten)]` (which buffers content the same way). The client
/// decodes responses from the raw body text for this reason. The wire
/// format on serialize stays a JSON number.
pub(crate) mod decimal_json {
    use rust_decimal::Decimal;
    use serde::{de::Error as _, Deserialize, Deserializer, Serializer};
    use serde_json::value::RawValue;

    pub fn serialize<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        rust_decimal::serde::float::serialize(value, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: &RawValue = Deserialize::deserialize(deserializer)?;
        super::parse_raw_decimal(raw.get()).map_err(D::Error::custom)
    }
}

/// Serde helpers for optional [`Decimal`](rust_decimal::Decimal) money
/// fields (ADR-0006). Same exact decoding and constraints as
/// [`decimal_json`]; `null` and (with `#[serde(default)]`) an absent key
/// both read `None`.
pub(crate) mod decimal_option_json {
    use rust_decimal::Decimal;
    use serde::{de::Error as _, Deserialize, Deserializer, Serializer};
    use serde_json::value::RawValue;

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
        let raw: Option<&RawValue> = Deserialize::deserialize(deserializer)?;
        match raw {
            None => Ok(None),
            Some(r) if r.get().trim() == "null" => Ok(None),
            Some(r) => super::parse_raw_decimal(r.get())
                .map(Some)
                .map_err(D::Error::custom),
        }
    }
}

#[cfg(test)]
mod decimal_tests {
    use rust_decimal::Decimal;
    use serde::Deserialize;
    use std::str::FromStr;

    #[derive(Deserialize)]
    struct Holder {
        #[serde(with = "super::decimal_json")]
        required: Decimal,
        #[serde(default, with = "super::decimal_option_json")]
        optional: Option<Decimal>,
    }

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    #[test]
    fn number_tokens_are_exact() {
        // Ordinary two-decimal amounts survive an f64 round trip, but
        // anything past ~15 significant digits does not: through f64 these
        // read 0.12345678901234568 and 12345678901234.568.
        let h: Holder = serde_json::from_str(
            r#"{"required": 0.123456789012345678, "optional": 12345678901234.567}"#,
        )
        .unwrap();
        assert_eq!(h.required.to_string(), "0.123456789012345678");
        assert_eq!(h.optional, Some(d("12345678901234.567")));
    }

    #[test]
    fn null_absent_strings_and_exponents() {
        let h: Holder =
            serde_json::from_str(r#"{"required": "-42.10", "optional": null}"#).unwrap();
        assert_eq!(h.required, d("-42.10"));
        assert_eq!(h.optional, None);
        let h: Holder = serde_json::from_str(r#"{"required": 1.5e-3}"#).unwrap();
        assert_eq!(h.required, d("0.0015"));
        assert_eq!(h.optional, None);
    }

    #[test]
    fn non_numbers_are_rejected() {
        assert!(serde_json::from_str::<Holder>(r#"{"required": true}"#).is_err());
        assert!(serde_json::from_str::<Holder>(r#"{"required": "abc"}"#).is_err());
    }
}
