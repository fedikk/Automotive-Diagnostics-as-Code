use std::path::Path;

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

fn main() {
    let command = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "help".to_string());

    match command.as_str() {
        "validate" => validate_model(),
        _ => print_help(),
    }
}

fn validate_model() {
    let model_dir = Path::new("model");

    println!("Validating diagnostic model...");

    let ecu = load_ecu(model_dir.join("ecu.yaml"))
        .expect("Failed to load model/ecu.yaml");

    validate_ecu(&ecu)
        .expect("ECU model validation failed");

    let services = load_services(model_dir.join("services.yaml"))
        .expect("Failed to load model/services.yaml");

    validate_services(&services)
        .expect("Services model validation failed");

    let dids = load_dids(model_dir.join("dids.yaml"))
        .expect("Failed to load model/dids.yaml");

    validate_dids(&dids)
        .expect("DID model validation failed");

    let dtcs = load_dtcs(model_dir.join("dtcs.yaml"))
        .expect("Failed to load model/dtcs.yaml");

    validate_dtcs(&dtcs)
        .expect("DTC model validation failed");

    println!("Diagnostic model is valid.");
}

fn print_help() {
    println!("Automotive Diagnostics as Code");
    println!();
    println!("Usage:");
    println!("  cargo run -p diagnostics-cli -- validate");
}
