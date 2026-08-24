//! Institutions endpoints.

use crate::models::institutions::{
    InstitutionsGetByIdRequest, InstitutionsGetByIdResponse, InstitutionsSearchRequest,
    InstitutionsSearchResponse,
};
use crate::{PlaidClient, PlaidError};

impl PlaidClient {
    /// Call `/institutions/get_by_id` to fetch a single institution by ID.
    ///
    /// Institutions endpoints authenticate with the client's configured
    /// `client_id` and `secret` only; no access token is required.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn institutions_get_by_id(
        &self,
        institution_id: &str,
        country_codes: &[String],
    ) -> Result<InstitutionsGetByIdResponse, PlaidError> {
        let request = InstitutionsGetByIdRequest {
            institution_id: institution_id.to_string(),
            country_codes: country_codes.to_vec(),
        };
        self.post("/institutions/get_by_id", &request).await
    }

    /// Call `/institutions/search` to search for institutions by name.
    ///
    /// Institutions endpoints authenticate with the client's configured
    /// `client_id` and `secret` only; no access token is required.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn institutions_search(
        &self,
        query: &str,
        products: &[String],
        country_codes: &[String],
    ) -> Result<InstitutionsSearchResponse, PlaidError> {
        let request = InstitutionsSearchRequest {
            query: query.to_string(),
            products: products.to_vec(),
            country_codes: country_codes.to_vec(),
        };
        self.post("/institutions/search", &request).await
    }
}
