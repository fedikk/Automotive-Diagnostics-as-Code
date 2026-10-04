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
    let model_dir = Path::new("model");

    println!("Validating diagnostic model...");

    let ecu = match load_ecu(model_dir.join("ecu.yaml")) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("ERROR: failed to load ecu.yaml: {error}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(error) = validate_ecu(&ecu) {
        eprintln!("ERROR: ECU validation failed: {error}");
        return ExitCode::FAILURE;
    }

    let services = match load_services(model_dir.join("services.yaml")) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("ERROR: failed to load services.yaml: {error}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(error) = validate_services(&services) {
        eprintln!("ERROR: services validation failed: {error}");
        return ExitCode::FAILURE;
    }

    let dids = match load_dids(model_dir.join("dids.yaml")) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("ERROR: failed to load dids.yaml: {error}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(error) = validate_dids(&dids) {
        eprintln!("ERROR: DIDs validation failed: {error}");
        return ExitCode::FAILURE;
    }

    let dtcs = match load_dtcs(model_dir.join("dtcs.yaml")) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("ERROR: failed to load dtcs.yaml: {error}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(error) = validate_dtcs(&dtcs) {
        eprintln!("ERROR: DTCs validation failed: {error}");
        return ExitCode::FAILURE;
    }

    println!("Diagnostic model is valid.");

    ExitCode::SUCCESS
}

fn print_help() {
    println!("Automotive Diagnostics as Code");
    println!();
    println!("Usage:");
    println!("  cargo run -p diagnostics-cli -- validate");
}