use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::{AppPaths, AppResult, read_file_if_exists},
    blocking::blocklist::BlocklistBundle,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProtectionMode {
    Inactive,
    Active,
    Degraded,
    Recovering,
}

impl ProtectionMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Inactive => "Inactive",
            Self::Active => "Active",
            Self::Degraded => "Degraded",
            Self::Recovering => "Recovering",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Normal,
    Warning,
    Critical,
}

impl RiskLevel {
    pub fn label(self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Warning => "Warning",
            Self::Critical => "Critical",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SafetyStatus {
    Pass,
    Warn,
    Fail,
}

impl SafetyStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Pass => "Pass",
            Self::Warn => "Warn",
            Self::Fail => "Fail",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyCheckSummary {
    pub check_id: String,
    pub timestamp: DateTime<Utc>,
    pub status: SafetyStatus,
    pub connectivity_ready: bool,
    pub recovery_ready: bool,
    pub issues: Vec<String>,
    pub recommended_action: String,
}

impl SafetyCheckSummary {
    pub fn new(
        status: SafetyStatus,
        connectivity_ready: bool,
        recovery_ready: bool,
        issues: Vec<String>,
        recommended_action: impl Into<String>,
    ) -> Self {
        Self {
            check_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            status,
            connectivity_ready,
            recovery_ready,
            issues,
            recommended_action: recommended_action.into(),
        }
    }

    pub fn risk_level(&self) -> RiskLevel {
        match self.status {
            SafetyStatus::Pass => RiskLevel::Normal,
            SafetyStatus::Warn => RiskLevel::Warning,
            SafetyStatus::Fail => RiskLevel::Critical,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeState {
    pub mode: ProtectionMode,
    pub status_summary: String,
    pub risk_level: RiskLevel,
    pub last_message: Option<String>,
    pub runtime_pid: Option<u32>,
    pub runtime_addr: Option<SocketAddr>,
    pub snapshot_id: Option<String>,
    pub last_transition_at: DateTime<Utc>,
    pub blocklist_version: String,
    pub blocklist_domain_count: usize,
    pub last_safety_check: Option<SafetyCheckSummary>,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            mode: ProtectionMode::Inactive,
            status_summary: "Protection is inactive. Run safety checks before enabling."
                .to_owned(),
            risk_level: RiskLevel::Normal,
            last_message: Some(
                "Sentinel is ready to inspect safety and activate protection.".to_owned(),
            ),
            runtime_pid: None,
            runtime_addr: None,
            snapshot_id: None,
            last_transition_at: Utc::now(),
            blocklist_version: "unloaded".to_owned(),
            blocklist_domain_count: 0,
            last_safety_check: None,
        }
    }
}

impl RuntimeState {
    pub fn refresh_bundle(&mut self, bundle: &BlocklistBundle) {
        self.blocklist_version = bundle.version.clone();
        self.blocklist_domain_count = bundle.domain_count;
    }
}

pub struct StateStore {
    paths: AppPaths,
}

impl StateStore {
    pub fn new(paths: AppPaths) -> Self {
        Self { paths }
    }

    pub fn load(&self) -> AppResult<RuntimeState> {
        match read_file_if_exists(&self.paths.state_file)? {
            Some(content) => serde_json::from_str(&content).into_diagnostic(),
            None => Ok(RuntimeState::default()),
        }
    }

    pub fn save(&self, state: &RuntimeState) -> AppResult<()> {
        let payload = serde_json::to_string_pretty(state).into_diagnostic()?;
        std::fs::write(&self.paths.state_file, payload).into_diagnostic()?;
        Ok(())
    }
}
