//! The message GetChargingProfilesRequest can be used by the CSMS to request installed
//! charging profiles from the Charging Station. The charging profiles will then be
//! reported by the Charging Station via ReportChargingProfilesRequest messages.
use crate::v2_0_1::datatypes::charging_profile_criterion_type::ChargingProfileCriterionType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::get_charging_profile_status_enum_type::GetChargingProfileStatusEnumType;

/// The message GetChargingProfilesRequest can be used by the CSMS to request installed
/// charging profiles from the Charging Station. The charging profiles will then be
/// reported by the Charging Station via ReportChargingProfilesRequest messages.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GetChargingProfilesRequest {
    /// Reference identification that is to be used bythe Charging Station in theReportChargingProfilesRequest when provided
    pub request_id: i32,
    /// For which EVSE installed charging profilesSHALL be reported. If 0, only charging profiles installedon the Charging Station itself (the grid connection)SHALL be reported. If omitted, all installed chargingprofiles SHALL be reported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    /// Specifies the charging profile
    pub charging_profile: ChargingProfileCriterionType,
}

/// This contains the field definition of the GetChargingProfilesResponse PDU sent by the Charging Station to the CSMS in response to a GetChargingProfilesRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GetChargingProfilesResponse {
    /// This indicates whether the Charging Station isable to process this request and will sendReportChargingProfilesRequest messages
    pub status: GetChargingProfileStatusEnumType,
    /// Detailed status information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
