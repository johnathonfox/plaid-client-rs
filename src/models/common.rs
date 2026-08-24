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
