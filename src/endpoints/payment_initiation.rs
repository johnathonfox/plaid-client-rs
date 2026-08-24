//! Payment Initiation product endpoints.

use crate::models::payment_initiation::{
    PaymentAmount, PaymentCreateResponse, PaymentGetResponse,
    PaymentInitiationPaymentCreateRequest, PaymentInitiationPaymentGetRequest,
    PaymentInitiationPaymentListRequest, PaymentInitiationRecipientCreateRequest,
    PaymentInitiationRecipientGetRequest, PaymentListResponse, RecipientBacs,
    RecipientCreateResponse, RecipientGetResponse,
};
use crate::{PlaidClient, PlaidError};

impl PlaidClient {
    /// Call `/payment_initiation/recipient/create` to create a payment
    /// recipient.
    ///
    /// Provide `iban` for SEPA payments or `bacs` for UK domestic
    /// payments; at least one is required.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn payment_initiation_recipient_create(
        &self,
        name: &str,
        iban: Option<&str>,
        bacs: Option<RecipientBacs>,
    ) -> Result<RecipientCreateResponse, PlaidError> {
        let request = PaymentInitiationRecipientCreateRequest { name, iban, bacs };
        self.post("/payment_initiation/recipient/create", &request)
            .await
    }

    /// Call `/payment_initiation/recipient/get` to fetch a previously
    /// created payment recipient.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn payment_initiation_recipient_get(
        &self,
        recipient_id: &str,
    ) -> Result<RecipientGetResponse, PlaidError> {
        let request = PaymentInitiationRecipientGetRequest { recipient_id };
        self.post("/payment_initiation/recipient/get", &request)
            .await
    }

    /// Call `/payment_initiation/payment/create` to create a payment to
    /// a recipient.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn payment_initiation_payment_create(
        &self,
        recipient_id: &str,
        reference: &str,
        amount_value: f64,
        currency: &str,
    ) -> Result<PaymentCreateResponse, PlaidError> {
        let request = PaymentInitiationPaymentCreateRequest {
            recipient_id,
            reference,
            amount: PaymentAmount {
                value: amount_value,
                currency: currency.to_string(),
            },
        };
        self.post("/payment_initiation/payment/create", &request)
            .await
    }

    /// Call `/payment_initiation/payment/get` to check the status of a
    /// payment.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn payment_initiation_payment_get(
        &self,
        payment_id: &str,
    ) -> Result<PaymentGetResponse, PlaidError> {
        let request = PaymentInitiationPaymentGetRequest { payment_id };
        self.post("/payment_initiation/payment/get", &request).await
    }

    /// Call `/payment_initiation/payment/list` to list created payments,
    /// most recent first.
    ///
    /// Pass `None` as `cursor` for the first page, then follow
    /// `next_cursor` until it is `None`.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn payment_initiation_payment_list(
        &self,
        count: Option<u32>,
        cursor: Option<String>,
    ) -> Result<PaymentListResponse, PlaidError> {
        let request = PaymentInitiationPaymentListRequest { count, cursor };
        self.post("/payment_initiation/payment/list", &request)
            .await
    }
}
