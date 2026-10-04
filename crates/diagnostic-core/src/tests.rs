#[cfg(test)]
mod tests {
    use crate::model::{
        Addressing,
        DiagnosticConfiguration,
        DidsDefinition,
        DtcDefinition,
        Ecu,
        EcuDefinition,
        Sessions,
        ServicesDefinition,
    };

    use crate::validation::{
        validate_dids,
        validate_dtcs,
        validate_ecu,
        validate_services,
    };

    #[test]
    fn valid_ecu_configuration_passes() {
        let model = EcuDefinition {
            ecu: Ecu {
                id: "virtual-ecu".to_string(),
                name: "Virtual Diagnostic ECU".to_string(),
                description: "Test ECU".to_string(),
                diagnostic: DiagnosticConfiguration {
                    addressing: Addressing {
                        addressing_type: "virtual".to_string(),
                    },
                    sessions: Sessions {
                        default: 0x01,
                        programming: 0x02,
                        extended: 0x03,
                    },
                },
            },
        };

        assert!(validate_ecu(&model).is_ok());
    }

    #[test]
    fn invalid_session_is_rejected() {
        let model = EcuDefinition {
            ecu: Ecu {
                id: "virtual-ecu".to_string(),
                name: "Virtual Diagnostic ECU".to_string(),
                description: "Test ECU".to_string(),
                diagnostic: DiagnosticConfiguration {
                    addressing: Addressing {
                        addressing_type: "virtual".to_string(),
                    },
                    sessions: Sessions {
                        default: 0x01,
                        programming: 0x02,
                        extended: 0x99,
                    },
                },
            },
        };

        assert!(validate_ecu(&model).is_err());
    }

    #[test]
    fn empty_services_pass_validation() {
        let model = ServicesDefinition {
            services: vec![],
        };

        assert!(validate_services(&model).is_ok());
    }

    #[test]
    fn duplicate_service_sid_is_rejected() {
        let yaml = r#"
services:
  - id: service-a
    name: Service A
    uds_sid: 0x10
    enabled: true

  - id: service-b
    name: Service B
    uds_sid: 0x10
    enabled: true
"#;

        let model: ServicesDefinition =
            serde_yaml::from_str(yaml).unwrap();

        assert!(validate_services(&model).is_err());
    }

    #[test]
    fn duplicate_did_is_rejected() {
        let yaml = r#"
dids:
  - id: VIN
    uds_did: 0xF190
    name: Vehicle Identification Number
    description: VIN
    data_type: string
    length: 17
    access:
      read: true

  - id: VIN
    uds_did: 0xF191
    name: Another VIN
    description: Duplicate identifier
    data_type: string
    length: 17
    access:
      read: true
"#;

        let model: DidsDefinition =
            serde_yaml::from_str(yaml).unwrap();

        assert!(validate_dids(&model).is_err());
    }

    #[test]
    fn invalid_did_length_is_rejected() {
        let yaml = r#"
dids:
  - id: VIN
    uds_did: 0xF190
    name: Vehicle Identification Number
    description: VIN
    data_type: string
    length: 0
    access:
      read: true
"#;

        let model: DidsDefinition =
            serde_yaml::from_str(yaml).unwrap();

        assert!(validate_dids(&model).is_err());
    }

    #[test]
    fn duplicate_dtc_is_rejected() {
        let yaml = r#"
dtcs:
  - id: P0300
    name: Random Misfire
    description: Misfire detected
    severity: warning

  - id: P0300
    name: Duplicate DTC
    description: Duplicate identifier
    severity: warning
"#;

        let model: DtcDefinition =
            serde_yaml::from_str(yaml).unwrap();

        assert!(validate_dtcs(&model).is_err());
    }
}
