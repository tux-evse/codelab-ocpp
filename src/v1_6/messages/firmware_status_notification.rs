use crate::v1_6::types::FirmwareStatus;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FirmwareStatusNotificationRequest {
    pub status: FirmwareStatus,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FirmwareStatusNotificationResponse {}
