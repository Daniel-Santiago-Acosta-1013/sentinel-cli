use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};

use crate::{
    app::{AppPaths, AppResult, read_file_if_exists},
    core::rules::RuleEntry,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub allow_rules: Vec<RuleEntry>,
}

impl AppConfig {
    pub fn add_allow_rule(&mut self, domain: String) -> bool {
        if self.allow_rules.iter().any(|rule| rule.value == domain) {
            return false;
        }
        if let Ok(rule) = RuleEntry::allow(&domain) {
            self.allow_rules.push(rule);
            return true;
        }
        false
    }

    pub fn remove_allow_rule(&mut self, domain: &str) -> bool {
        let len_before = self.allow_rules.len();
        self.allow_rules.retain(|rule| rule.value != domain);
        len_before != self.allow_rules.len()
    }

    pub fn user_rules(&self) -> Vec<RuleEntry> {
        self.allow_rules.clone()
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
            None => Ok(AppConfig::default()),
        }
    }

    pub fn save(&self, config: &AppConfig) -> AppResult<()> {
        let content = toml::to_string_pretty(config).into_diagnostic()?;
        std::fs::write(&self.paths.config_file, content).into_diagnostic()?;
        Ok(())
    }
}
