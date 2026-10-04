use std::path::Path;
use std::process::ExitCode;

use diagnostic_core::loader::{
    load_dids,
    load_dtcs,
    load_ecu,
    load_services,
};

use diagnostic_core::validation::{
    validate_dids,
    validate_dtcs,
    validate_ecu,
    validate_services,
};

fn main() -> ExitCode {
    let command = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "help".to_string());

    match command.as_str() {
        "validate" => validate_model(),
        _ => {
            print_help();
            ExitCode::SUCCESS
        }
    }
}

fn validate_model() -> ExitCode {
    match validate_model_result() {
        Ok(()) => {
            println!("Diagnostic model is valid.");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("ERROR: {error}");
            ExitCode::FAILURE
        }
    }
}


fn validate_model_result() -> Result<(), String> {
    let model_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../model");

    println!("Validating diagnostic model...");

    let ecu = load_ecu(model_dir.join("ecu.yaml"))
        .map_err(|error| format!("failed to load ecu.yaml: {error}"))?;

    validate_ecu(&ecu)
        .map_err(|error| format!("ECU validation failed: {error}"))?;

    let services = load_services(model_dir.join("services.yaml"))
        .map_err(|error| format!("failed to load services.yaml: {error}"))?;

    validate_services(&services)
        .map_err(|error| format!("services validation failed: {error}"))?;

    let dids = load_dids(model_dir.join("dids.yaml"))
        .map_err(|error| format!("failed to load dids.yaml: {error}"))?;

    validate_dids(&dids)
        .map_err(|error| format!("DIDs validation failed: {error}"))?;

    let dtcs = load_dtcs(model_dir.join("dtcs.yaml"))
        .map_err(|error| format!("failed to load dtcs.yaml: {error}"))?;

    validate_dtcs(&dtcs)
        .map_err(|error| format!("DTCs validation failed: {error}"))?;

    Ok(())
}


fn print_help() {
    println!("Automotive Diagnostics as Code");
    println!();
    println!("Usage:");
    println!("  cargo run -p diagnostics-cli -- validate");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_model_validation_succeeds() {
        let result = validate_model_result();

        assert!(
            result.is_ok(),
            "diagnostic model should be valid: {result:?}"
        );
    }
}
