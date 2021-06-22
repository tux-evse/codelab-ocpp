use chrono::{DateTime, Utc};

use crate::v2_0_1::core::{
    datatypes::{
        evse_type::EVSEType, id_token_info_type::IdTokenInfoType, id_token_type::IdTokenType,
        message_content_type::MessageContentType, meter_value_type::MeterValueType,
        transaction_type::TransactionType,
    },
    enumerations::{
        transaction_event_enum_type::TransactionEventEnumType,
        trigger_reason_enum_type::TriggerReasonEnumType,
    },
};

/// Sent by the Charging Station to the CSMS to request that the Certificate Authority signs the public key into a certificate.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventRequest {
    pub event_type: TransactionEventEnumType,
    pub timestamp: DateTime<Utc>,
    pub trigger_reason: TriggerReasonEnumType,
    pub seq_no: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_phases_used: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cable_max_current: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i64>,
    pub transaction_info: TransactionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_value: Option<MeterValueType>,
}

/// This contains the field definition of the TransactionEventResponse PDU sent by the CSMS to the Charging Station in response to a TransactionEventRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_info: Option<IdTokenInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_personal_message: Option<MessageContentType>,
}
