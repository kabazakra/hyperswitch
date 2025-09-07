use common_enums::enums;
use common_utils::types::StringMinorUnit;
use hyperswitch_domain_models::{
    payment_method_data::{BankDebitData, Card, PayLaterData, PaymentMethodData, WalletData},
    router_data::{ConnectorAuthType, PaymentMethodToken, RouterData},
    router_flow_types::refunds::{Execute, RSync},
    router_request_types::ResponseId,
    router_response_types::{PaymentsResponseData, RefundsResponseData},
    types::{PaymentsAuthorizeRouterData, RefundsRouterData, TokenizationRouterData},
};
use hyperswitch_interfaces::errors;
use masking::{Secret, PeekInterface};
use serde::{Deserialize, Serialize};


use crate::{
    types::{RefundsResponseRouterData, ResponseRouterData},
    utils::RouterData as _,
};

//TODO: Fill the struct with respective fields
pub struct SquearesandboxRouterData<T> {
    pub amount: StringMinorUnit, // The type of amount that a connector accepts, for example, String, i64, f64, etc.
    pub router_data: T,
}

impl<T> From<(StringMinorUnit, T)> for SquearesandboxRouterData<T> {
    fn from((amount, item): (StringMinorUnit, T)) -> Self {
        //Todo :  use utils to convert the amount to the type of amount that a connector accepts
        Self {
            amount,
            router_data: item,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, PartialEq)]
pub struct SquearesandboxPaymentsRequest {
    amount_money: SquearesandboxPaymentsAmountData,
    idempotency_key: Secret<String>,
    source_id: Secret<String>,
    autocomplete: bool,
    external_details: SquearesandboxPaymentsRequestExternalDetails,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SquearesandboxPaymentsAmountData {
    amount: i64,
    currency: enums::Currency,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SquearesandboxPaymentsRequestExternalDetails {
    source: String,
    #[serde(rename = "type")]
    source_type: String,
}

#[derive(Default, Debug, Serialize, Eq, PartialEq)]
pub struct SquearesandboxCard {
    number: cards::CardNumber,
    expiry_month: Secret<String>,
    expiry_year: Secret<String>,
    cvc: Secret<String>,
    complete: bool,
}

impl TryFrom<&SquearesandboxRouterData<&PaymentsAuthorizeRouterData>>
    for SquearesandboxPaymentsRequest
{
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(
        item: &SquearesandboxRouterData<&PaymentsAuthorizeRouterData>,
    ) -> Result<Self, Self::Error> {

        let source = match item.router_data.get_payment_method_token()? {
            PaymentMethodToken::Token(pm_token) => Ok(pm_token),
            _ => Err(errors::ConnectorError::MissingRequiredField {
                field_name: "payment_method_token",
            }),
        }?;

        Ok(Self {
            amount_money: SquearesandboxPaymentsAmountData {
                amount: item.amount.to_string().parse().unwrap_or(0),
                currency: item.router_data.request.currency,
            },
            idempotency_key: Secret::new("key".to_string()),
            source_id: source, // Use the source variable you extracted
            autocomplete: true,
            external_details: SquearesandboxPaymentsRequestExternalDetails {
                source: "card".to_string(),
                source_type: "card".to_string(),
            },
        })
    }
}

//TODO: Fill the struct with respective fields
// Auth Struct
pub struct SquearesandboxAuthType {
    pub(super) api_key: Secret<String>,
}

impl TryFrom<&ConnectorAuthType> for SquearesandboxAuthType {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(auth_type: &ConnectorAuthType) -> Result<Self, Self::Error> {
        match auth_type {
            ConnectorAuthType::HeaderKey { api_key } => Ok(Self {
                api_key: api_key.to_owned(),
            }),
            _ => Err(errors::ConnectorError::FailedToObtainAuthType.into()),
        }
    }
}
// PaymentsResponse
//TODO: Append the remaining status flags
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SquearesandboxPaymentStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<SquearesandboxPaymentStatus> for common_enums::AttemptStatus {
    fn from(item: SquearesandboxPaymentStatus) -> Self {
        match item {
            SquearesandboxPaymentStatus::Succeeded => Self::Charged,
            SquearesandboxPaymentStatus::Failed => Self::Failure,
            SquearesandboxPaymentStatus::Processing => Self::Authorizing,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SquearesandboxPaymentsResponse {
    status: SquearesandboxPaymentStatus,
    id: String,
}

impl<F, T> TryFrom<ResponseRouterData<F, SquearesandboxPaymentsResponse, T, PaymentsResponseData>>
    for RouterData<F, T, PaymentsResponseData>
{
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: ResponseRouterData<F, SquearesandboxPaymentsResponse, T, PaymentsResponseData>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            status: common_enums::AttemptStatus::from(item.response.status),
            response: Ok(PaymentsResponseData::TransactionResponse {
                resource_id: ResponseId::ConnectorTransactionId(item.response.id),
                redirection_data: Box::new(None),
                mandate_reference: Box::new(None),
                connector_metadata: None,
                network_txn_id: None,
                connector_response_reference_id: None,
                incremental_authorization_allowed: None,
                charges: None,
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
// REFUND :
// Type definition for RefundRequest
#[derive(Default, Debug, Serialize)]
pub struct SquearesandboxRefundRequest {
    pub amount: StringMinorUnit,
}

impl<F> TryFrom<&SquearesandboxRouterData<&RefundsRouterData<F>>> for SquearesandboxRefundRequest {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: &SquearesandboxRouterData<&RefundsRouterData<F>>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            amount: item.amount.to_owned(),
        })
    }
}

// Type definition for Refund Response

#[allow(dead_code)]
#[derive(Debug, Copy, Serialize, Default, Deserialize, Clone)]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
            //TODO: Review mapping
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    id: String,
    status: RefundStatus,
}

impl TryFrom<RefundsResponseRouterData<Execute, RefundResponse>> for RefundsRouterData<Execute> {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: RefundsResponseRouterData<Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            response: Ok(RefundsResponseData {
                connector_refund_id: item.response.id.to_string(),
                refund_status: enums::RefundStatus::from(item.response.status),
            }),
            ..item.data
        })
    }
}

impl TryFrom<RefundsResponseRouterData<RSync, RefundResponse>> for RefundsRouterData<RSync> {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: RefundsResponseRouterData<RSync, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            response: Ok(RefundsResponseData {
                connector_refund_id: item.response.id.to_string(),
                refund_status: enums::RefundStatus::from(item.response.status),
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct SquearesandboxErrorResponse {
    pub status_code: u16,
    pub code: String,
    pub message: String,
    pub reason: Option<String>,
    pub network_advice_code: Option<String>,
    pub network_decline_code: Option<String>,
    pub network_error_message: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct SquearesandboxTokenizeData {
    client_id: Secret<String>,
    session_id: Secret<String>,
    card_data: SquearesandboxCardData,
}

#[derive(Debug, Serialize)]
pub struct SquearesandboxCardData {
    cvv: Secret<String>,
    exp_month: Secret<u16>,
    exp_year: Secret<u16>,
    number: cards::CardNumber,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum SquearesandboxTokenRequest {
    Card(SquearesandboxTokenizeData)
}

impl TryFrom<&TokenizationRouterData> for SquearesandboxTokenRequest {
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(item: &TokenizationRouterData) -> Result<Self, Self::Error> {
        let card_data = match &item.request.payment_method_data {
            PaymentMethodData::Card(card) => card,
            _ => return Err(errors::ConnectorError::NotImplemented("Only card payments supported".to_string()).into()),
        };

        Ok(Self::Card(SquearesandboxTokenizeData {
            client_id: Secret::new("dummy_client_id".to_string()),
            session_id: Secret::new("dummy_session_id".to_string()),
            card_data: SquearesandboxCardData {
                cvv: card_data.card_cvc.clone(),
                exp_month: Secret::new(card_data.card_exp_month.peek().parse().unwrap_or(12)),
                exp_year: Secret::new(card_data.card_exp_year.peek().parse().unwrap_or(2025)),
                number: card_data.card_number.clone(),
            },
        }))
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SquearesandboxTokenResponse {
    pub card_nonce: String,
}

impl TryFrom<&SquearesandboxTokenResponse> for PaymentsResponseData {
    type Error = error_stack::Report<errors::ConnectorError>;

    fn try_from(item: &SquearesandboxTokenResponse) -> Result<Self, Self::Error> {
        Ok(PaymentsResponseData::TokenizationResponse {
            token: item.card_nonce.clone(),
        })
    }
}
