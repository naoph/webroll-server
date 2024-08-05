use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use snafu::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    workers: Vec<WorkerSpec>,
}

impl Config {
    pub fn from_file(path: PathBuf) -> Result<Self, ConfigError> {
        let raw = std::fs::read_to_string(path)
            .context(ReadFromDiscSnafu)?;
        let config: Self = ron::from_str(&raw)
            .context(ParseSnafu)?;
        if config.validate() {
            Ok(config)
        } else {
            Err(ConfigError::ValidationError)
        }
    }

    pub fn from_env(path: &str) -> Result<Self, ConfigError> {
        let config_path = std::env::var(path)
            .context(GetEnvSnafu { var: path.to_string() })?;
        let config_path = PathBuf::from(config_path);
        Self::from_file(config_path)
    }

    pub fn validate(&self) -> bool {
        self.workers.len() > 0
    }

    pub fn workers(&self) -> Vec<WorkerSpec> {
        self.workers.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkerSpec {
    nickname: String,
    root: url::Url,
    auth_token: String,
}

impl WorkerSpec {
    pub fn nickname(&self) -> &str {
        &self.nickname
    }
}

#[derive(Debug, Snafu)]
pub enum ConfigError {
    #[snafu(display("Error reading from disc"))]
    ReadFromDiscError { source: std::io::Error },

    #[snafu(display("Error parsing from serialized format"))]
    ParseError { source: ron::error::SpannedError },

    #[snafu(display("Environment variable {var} is not set"))]
    GetEnvError { source: std::env::VarError, var: String },

    #[snafu(display("Config failed validation"))]
    ValidationError,
}
