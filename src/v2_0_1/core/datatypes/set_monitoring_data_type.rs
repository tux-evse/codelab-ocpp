use crate::v2_0_1::core::enumerations::monitor_enum_type::MonitorEnumType;

use super::{component_type::ComponentType, variable_type::VariableType};

/// Class to hold parameters of SetVariableMonitoring request.
/// SetMonitoringDataType is used by: SetVariableMonitoringRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringDataType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<bool>,
    pub value: f64,
    #[serde(rename = "type")]
    pub kind: MonitorEnumType,
    pub severity: u8,
    pub component: ComponentType,
    pub variable: VariableType,
}
