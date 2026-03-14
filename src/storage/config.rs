use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};

use crate::app::{AppPaths, AppResult, read_file_if_exists};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub upstream_dns: String,
    pub local_dns_ip: String,
    pub confirm_before_network_changes: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            upstream_dns: "1.1.1.1:53".to_owned(),
            local_dns_ip: "127.0.0.1".to_owned(),
            confirm_before_network_changes: true,
        }
    }
}

pub struct ConfigStore {
    paths: AppPaths,
}

impl ConfigStore {
    pub fn new(paths: AppPaths) -> Self {
        Self { paths }
    }

    pub fn load(&self) -> AppResult<AppConfig> {
        match read_file_if_exists(&self.paths.config_file)? {
            Some(content) => toml::from_str(&content).into_diagnostic(),
            None => {
                let config = AppConfig::default();
                self.save(&config)?;
                Ok(config)
            }
        }
    }

    pub fn save(&self, config: &AppConfig) -> AppResult<()> {
        let payload = toml::to_string_pretty(config).into_diagnostic()?;
        std::fs::write(&self.paths.config_file, payload).into_diagnostic()?;
        Ok(())
    }
}
