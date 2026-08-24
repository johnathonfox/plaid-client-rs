//! Models for the Institutions endpoints.

use serde::{Deserialize, Serialize};

/// A financial institution supported by Plaid.
// Field names mirror Plaid's JSON schema.
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    /// The Plaid institution ID (e.g. `ins_109508`).
    pub institution_id: String,
    /// The institution's name.
    pub name: String,
    /// The Plaid products the institution supports.
    pub products: Vec<String>,
    /// The country codes in which the institution operates.
    pub country_codes: Vec<String>,
    /// The institution's website URL, if available.
    pub url: Option<String>,
    /// A base64-encoded logo of the institution, if available.
    pub logo: Option<String>,
    /// The institution's primary brand color as a hex string, if available.
    pub primary_color: Option<String>,
}

/// Request for `/institutions/get_by_id`.
#[derive(Debug, Clone, Serialize)]
pub struct InstitutionsGetByIdRequest {
    /// The ID of the institution to fetch.
    pub institution_id: String,
    /// The country codes of the institution to match.
    pub country_codes: Vec<String>,
}

/// Response from `/institutions/get_by_id`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionsGetByIdResponse {
    /// The institution matching the request.
    pub institution: Institution,
    /// A unique identifier for the request.
    pub request_id: String,
}

/// Request for `/institutions/search`.
#[derive(Debug, Clone, Serialize)]
pub struct InstitutionsSearchRequest {
    /// The search query matched against institution names.
    pub query: String,
    /// The Plaid products the institution must support.
    pub products: Vec<String>,
    /// The country codes to search within.
    pub country_codes: Vec<String>,
}

/// Response from `/institutions/search`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionsSearchResponse {
    /// The institutions matching the search.
    pub institutions: Vec<Institution>,
    /// A unique identifier for the request.
    pub request_id: String,
}
