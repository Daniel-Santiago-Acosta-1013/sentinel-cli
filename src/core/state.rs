use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::storage::config::AppConfig;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProtectionMode {
    Inactive,
    Activating,
    Active,
    Degraded,
    Recovering,
    Failed,
}

impl ProtectionMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Activating => "activating",
            Self::Active => "active",
            Self::Degraded => "degraded",
            Self::Recovering => "recovering",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectionState {
    pub mode: ProtectionMode,
    pub started_at: Option<DateTime<Utc>>,
    pub last_transition_at: DateTime<Utc>,
    pub last_error_code: Option<String>,
    pub snapshot_id: Option<String>,
    pub active_rule_count: usize,
    pub active_exclusion_count: usize,
    pub runtime_pid: Option<u32>,
    pub runtime_addr: Option<SocketAddr>,
}

impl Default for ProtectionState {
    fn default() -> Self {
        Self {
            mode: ProtectionMode::Inactive,
            started_at: None,
            last_transition_at: Utc::now(),
            last_error_code: None,
            snapshot_id: None,
            active_rule_count: 0,
            active_exclusion_count: 0,
            runtime_pid: None,
            runtime_addr: None,
        }
    }
}

impl ProtectionState {
    pub fn activate(
        &mut self,
        snapshot_id: String,
        pid: u32,
        runtime_addr: SocketAddr,
        config: &AppConfig,
    ) {
        self.mode = ProtectionMode::Active;
        self.started_at = Some(Utc::now());
        self.last_transition_at = Utc::now();
        self.snapshot_id = Some(snapshot_id);
        self.runtime_pid = Some(pid);
        self.runtime_addr = Some(runtime_addr);
        self.active_rule_count = crate::core::rules::built_in_rules().len();
        self.active_exclusion_count = config.allow_rules.len();
        self.last_error_code = None;
    }

    pub fn deactivate(&mut self, config: &AppConfig) {
        self.mode = ProtectionMode::Inactive;
        self.last_transition_at = Utc::now();
        self.runtime_pid = None;
        self.runtime_addr = None;
        self.snapshot_id = None;
        self.active_rule_count = crate::core::rules::built_in_rules().len();
        self.active_exclusion_count = config.allow_rules.len();
        self.last_error_code = None;
    }
}
