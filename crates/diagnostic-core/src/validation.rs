use std::collections::HashSet;

use crate::error::ModelError;
use crate::model::{
    DidsDefinition,
    DtcDefinition,
    EcuDefinition,
    ServicesDefinition,
};

const SUPPORTED_SERVICES: &[u8] = &[
    0x10, // Diagnostic Session Control
    0x11, // ECU Reset
    0x14, // Clear Diagnostic Information
    0x19, // Read DTC Information
    0x22, // Read Data By Identifier
];

const SUPPORTED_SESSIONS: &[u8] = &[
    0x01, // Default
    0x02, // Programming
    0x03, // Extended
];

const SUPPORTED_DATA_TYPES: &[&str] = &[
    "string",
];

pub fn validate_ecu(model: &EcuDefinition) -> Result<(), ModelError> {
    let sessions = &model.ecu.diagnostic.sessions;

    let configured_sessions = [
        sessions.default,
        sessions.programming,
        sessions.extended,
    ];

    for session in configured_sessions {
        if !SUPPORTED_SESSIONS.contains(&session) {
            return Err(ModelError::Validation(format!(
                "unsupported diagnostic session: 0x{session:02X}"
            )));
        }
    }

    Ok(())
}

pub fn validate_services(
    model: &ServicesDefinition,
) -> Result<(), ModelError> {
    let mut ids = HashSet::new();
    let mut sids = HashSet::new();

    for service in &model.services {
        if !ids.insert(&service.id) {
            return Err(ModelError::Validation(format!(
                "duplicate service identifier: {}",
                service.id
            )));
        }

        if !sids.insert(service.uds_sid) {
            return Err(ModelError::Validation(format!(
                "duplicate UDS SID: 0x{:02X}",
                service.uds_sid
            )));
        }

        if !SUPPORTED_SERVICES.contains(&service.uds_sid) {
            return Err(ModelError::Validation(format!(
                "unsupported UDS SID: 0x{:02X}",
                service.uds_sid
            )));
        }
    }

    Ok(())
}

pub fn validate_dids(
    model: &DidsDefinition,
) -> Result<(), ModelError> {
    let mut ids = HashSet::new();
    let mut uds_dids = HashSet::new();

    for did in &model.dids {
        if !ids.insert(&did.id) {
            return Err(ModelError::Validation(format!(
                "duplicate DID identifier: {}",
                did.id
            )));
        }

        if !uds_dids.insert(did.uds_did) {
            return Err(ModelError::Validation(format!(
                "duplicate UDS DID: 0x{:04X}",
                did.uds_did
            )));
        }

        if !SUPPORTED_DATA_TYPES.contains(&did.data_type.as_str()) {
            return Err(ModelError::Validation(format!(
                "unsupported DID data type: {}",
                did.data_type
            )));
        }

        if did.length == 0 {
            return Err(ModelError::Validation(format!(
                "DID {} has invalid length",
                did.id
            )));
        }
    }

    Ok(())
}

pub fn validate_dtcs(
    model: &DtcDefinition,
) -> Result<(), ModelError> {
    let mut ids = HashSet::new();

    for dtc in &model.dtcs {
        if !ids.insert(&dtc.id) {
            return Err(ModelError::Validation(format!(
                "duplicate DTC identifier: {}",
                dtc.id
            )));
        }
    }

    Ok(())
}
