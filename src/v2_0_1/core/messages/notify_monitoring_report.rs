use chrono::{DateTime, Utc};

use crate::v2_0_1::core::datatypes::monitoring_data_type::MonitoringDataType;

/// This contains the field definition of the NotifyMonitoringRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportRequest {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub seq_no: i64,
    pub generated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<MonitoringDataType>,
}

/// Response to a NotifyMonitoringRequest message. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportResponse {}
