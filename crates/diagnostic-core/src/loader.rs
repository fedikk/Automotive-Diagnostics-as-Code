use std::fs;
use std::path::Path;

use crate::error::ModelError;
use crate::model::{
    DidsDefinition,
    DtcDefinition,
    EcuDefinition,
    ServicesDefinition,
};

pub fn load_ecu(path: impl AsRef<Path>) -> Result<EcuDefinition, ModelError> {
    let content = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&content)?)
}

pub fn load_services(
    path: impl AsRef<Path>,
) -> Result<ServicesDefinition, ModelError> {
    let content = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&content)?)
}

pub fn load_dids(
    path: impl AsRef<Path>,
) -> Result<DidsDefinition, ModelError> {
    let content = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&content)?)
}

pub fn load_dtcs(
    path: impl AsRef<Path>,
) -> Result<DtcDefinition, ModelError> {
    let content = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&content)?)
}