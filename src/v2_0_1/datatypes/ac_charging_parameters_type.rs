//! EV AC charging parameters

/// EV AC charging parameters
///
/// ACChargingParametersType is used by: [ChargingNeedsType](`crate::v2_0_1::datatypes::charging_needs_type::ChargingNeedsType`)
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ACChargingParametersType {
    /// Amount of energy requested (in Wh). This includes energy required for preconditioning.
    pub energy_amount: i32,
    /// Minimum current (amps) supported by theelectric vehicle (per phase).
    pub ev_min_current: i32,
    /// Maximum current (amps) supported by the electric vehicle (per phase). Includes cable capacity.
    pub ev_max_current: i32,
    /// Maximum voltage supported by the electric vehicle
    pub ev_max_voltage: i32,
}
