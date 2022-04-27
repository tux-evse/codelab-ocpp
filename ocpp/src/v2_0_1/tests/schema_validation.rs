#[cfg(test)]
mod tests {
    use chrono::Utc;
    use jsonschema::JSONSchema;
    use crate::v2_0_1::datatypes::charging_station_type::ChargingStationType;
    use crate::v2_0_1::enumerations::boot_reason_enum_type::BootReasonEnumType;
    use crate::v2_0_1::enumerations::registration_status_enum_type::RegistrationStatusEnumType;
    use crate::v2_0_1::messages::boot_notification::{BootNotificationRequest, BootNotificationResponse};
    use crate::v2_0_1::datatypes::id_token_info_type::IdTokenInfoType;
    use crate::v2_0_1::datatypes::id_token_type::IdTokenType;
    use crate::v2_0_1::enumerations::authorization_status_enum_type::AuthorizationStatusEnumType;
    use crate::v2_0_1::enumerations::cancel_reservation_status_enum_type::CancelReservationStatusEnumType;
    use crate::v2_0_1::enumerations::certificate_signed_status_enum_type::CertificateSignedStatusEnumType;
    use crate::v2_0_1::enumerations::change_availability_status_enum_type::ChangeAvailabilityStatusEnumType;
    use crate::v2_0_1::enumerations::clear_cache_status_enum_type::ClearCacheStatusEnumType;
    use crate::v2_0_1::enumerations::id_token_enum_type::IdTokenEnumType;
    use crate::v2_0_1::enumerations::operational_status_enum_type::OperationalStatusEnumType;
    use crate::v2_0_1::messages::authorize::{AuthorizeRequest, AuthorizeResponse};
    use crate::v2_0_1::messages::cancel_reservation::{CancelReservationRequest, CancelReservationResponse};
    use crate::v2_0_1::messages::certificate_signed::{CertificateSignedRequest, CertificateSignedResponse};
    use crate::v2_0_1::messages::change_availability::{ChangeAvailabilityRequest, ChangeAvailabilityResponse};
    use crate::v2_0_1::messages::clear_cache::{ClearCacheRequest, ClearCacheResponse};

    #[test]
    fn validate_authorize_request() {
        let test = AuthorizeRequest {
            certificate: None,
            id_token: IdTokenType {
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: None
            },
            iso_15118_certificate_hash_data: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/AuthorizeRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!(
                    "Instance path: {}", error.instance_path
                );
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_authorize_response() {
        let test = AuthorizeResponse {
            certificate_status: None,
            id_token_info: IdTokenInfoType {
                status: AuthorizationStatusEnumType::Accepted,
                cache_expiry_date_time: None,
                charging_priority: None,
                language1: None,
                evse_id: None,
                language2: None,
                group_id_token: None,
                personal_message: None
            }
        };

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/AuthorizeResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!(
                    "Instance path: {}", error.instance_path
                );
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_bootnotification_request() {
        let test = BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStationType {
                model: "SingleSocketCharger".to_string(),
                vendor_name: "VendorX".to_string(),
                serial_number: None,
                firmware_version: None,
                modem: None,
            },
        };
        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/BootNotificationRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!(
                    "Instance path: {}", error.instance_path
                );
            }
        }
        assert!(compiled.is_valid(&instance));

    }

    #[test]
    fn validate_bootnotification_response() {
        let test = BootNotificationResponse {
            current_time: Utc::now(),
            interval: 10,
            status: RegistrationStatusEnumType::Accepted,
            status_info: None,
        };

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/BootNotificationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!(
                    "Instance path: {}", error.instance_path
                );
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_cancelreservation_request() {
        let test = CancelReservationRequest {
            reservation_id: 0
        };
        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/CancelReservationRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!(
                    "Instance path: {}", error.instance_path
                );
            }
        }
        assert!(compiled.is_valid(&instance));

    }

    #[test]
    fn validate_cancelreservation_response() {
        let test = CancelReservationResponse {
            status: CancelReservationStatusEnumType::Accepted,
            status_info: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/CancelReservationResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!(
                    "Instance path: {}", error.instance_path
                );
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_certificate_signed_request() {
        let test = CertificateSignedRequest {
            certificate_chain: "asd".to_string(),
            certificate_type: None
        };
        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/CertificateSignedRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!(
                    "Instance path: {}", error.instance_path
                );
            }
        }
        assert!(compiled.is_valid(&instance));

    }

    #[test]
    fn validate_certificate_signed_response() {
        let test = CertificateSignedResponse {
            status: CertificateSignedStatusEnumType::Accepted,
            status_info: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/CertificateSignedResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!(
                    "Instance path: {}", error.instance_path
                );
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_change_availability_request() {
        let test = ChangeAvailabilityRequest {
            operational_status: OperationalStatusEnumType::Inoperative,
            evse: None
        };
        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/ChangeAvailabilityRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!(
                    "Instance path: {}", error.instance_path
                );
            }
        }
        assert!(compiled.is_valid(&instance));

    }

    #[test]
    fn validate_change_availability_response() {
        let test = ChangeAvailabilityResponse {
            status: ChangeAvailabilityStatusEnumType::Accepted,
            status_info: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/ChangeAvailabilityResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!(
                    "Instance path: {}", error.instance_path
                );
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_cache_request() {
        let test = ClearCacheRequest {
        };
        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/ClearCacheRequest.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!(
                    "Instance path: {}", error.instance_path
                );
            }
        }
        assert!(compiled.is_valid(&instance));

    }

    #[test]
    fn validate_clear_cache_response() {
        let test = ClearCacheResponse {
            status: ClearCacheStatusEnumType::Accepted,
            status_info: None
        };

        let schema = include_str!("../../../../schemas/ocpp/v2.0.1/ClearCacheResponse.json");
        let schema = serde_json::from_str(&schema).unwrap();
        let instance = serde_json::to_value(&test).unwrap();
        let compiled = JSONSchema::compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!(
                    "Instance path: {}", error.instance_path
                );
            }
        }
        assert!(compiled.is_valid(&instance));
    }
}
