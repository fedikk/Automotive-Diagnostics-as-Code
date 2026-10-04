use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("failed to read model file: {0}")]
    Io(#[from] io::Error),

    #[error("failed to parse YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("invalid diagnostic model: {0}")]
    Validation(String),
}
