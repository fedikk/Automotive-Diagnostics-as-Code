use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EcuDefinition {
    pub ecu: Ecu,
}

#[derive(Debug, Deserialize)]
pub struct Ecu {
    pub id: String,
    pub name: String,
    pub description: String,
    pub diagnostic: DiagnosticConfiguration,
}

#[derive(Debug, Deserialize)]
pub struct DiagnosticConfiguration {
    pub addressing: Addressing,
    pub sessions: Sessions,
}

#[derive(Debug, Deserialize)]
pub struct Addressing {
    #[serde(rename = "type")]
    pub addressing_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Sessions {
    pub default: u8,
    pub programming: u8,
    pub extended: u8,
}

#[derive(Debug, Deserialize)]
pub struct ServicesDefinition {
    pub services: Vec<DiagnosticService>,
}

#[derive(Debug, Deserialize)]
pub struct DiagnosticService {
    pub id: String,
    pub name: String,
    pub uds_sid: u8,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct DidsDefinition {
    pub dids: Vec<Did>,
}

#[derive(Debug, Deserialize)]
pub struct Did {
    pub id: String,
    pub uds_did: u16,
    pub name: String,
    pub description: String,
    pub data_type: String,
    pub length: u16,
    pub access: DidAccess,
}

#[derive(Debug, Deserialize)]
pub struct DidAccess {
    pub read: bool,
}

#[derive(Debug, Deserialize)]
pub struct DtcDefinition {
    pub dtcs: Vec<Dtc>,
}

#[derive(Debug, Deserialize)]
pub struct Dtc {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: DtcSeverity,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DtcSeverity {
    Info,
    Warning,
    Critical,
}