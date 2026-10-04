#[cfg(test)]
mod tests {
    use crate::loader::{
        load_dids,
        load_dtcs,
        load_ecu,
        load_services,
    };

    use crate::validation::{
        validate_dids,
        validate_dtcs,
        validate_ecu,
        validate_services,
    };

    #[test]
    fn real_ecu_model_is_valid() {
        let model = load_ecu("../../model/ecu.yaml")
            .expect("failed to load ecu.yaml");

        validate_ecu(&model)
            .expect("ecu.yaml validation failed");
    }

    #[test]
    fn real_services_model_is_valid() {
        let model = load_services("../../model/services.yaml")
            .expect("failed to load services.yaml");

        validate_services(&model)
            .expect("services.yaml validation failed");
    }

    #[test]
    fn real_dids_model_is_valid() {
        let model = load_dids("../../model/dids.yaml")
            .expect("failed to load dids.yaml");

        validate_dids(&model)
            .expect("dids.yaml validation failed");
    }

    #[test]
    fn real_dtcs_model_is_valid() {
        let model = load_dtcs("../../model/dtcs.yaml")
            .expect("failed to load dtcs.yaml");

        validate_dtcs(&model)
            .expect("dtcs.yaml validation failed");
    }
}