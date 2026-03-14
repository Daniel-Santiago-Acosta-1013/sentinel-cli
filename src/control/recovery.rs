use std::path::PathBuf;

use chrono::{DateTime, Utc};
use miette::{IntoDiagnostic, miette};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::{AppPaths, AppResult, read_file_if_exists},
    blocking::runtime,
    platform::macos::MacOsNetworkManager,
    storage::{
        config::ConfigStore,
        events::{EventKind, EventRecord, EventStore, Severity},
        state::{ProtectionMode, RiskLevel, RuntimeState, StateStore},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkServiceSnapshot {
    pub service: String,
    pub dns_servers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRecoverySnapshot {
    pub snapshot_id: String,
    pub captured_at: DateTime<Utc>,
    pub affected_services: Vec<String>,
    pub dns_state: Vec<NetworkServiceSnapshot>,
    pub routing_state: Vec<String>,
    pub restorable: bool,
}

impl NetworkRecoverySnapshot {
    fn file_path(&self, paths: &AppPaths) -> PathBuf {
        paths.snapshots_dir.join(format!("{}.json", self.snapshot_id))
    }
}

pub struct RecoveryController<'a> {
    paths: &'a AppPaths,
    config_store: &'a ConfigStore,
    state_store: &'a StateStore,
    event_store: &'a EventStore,
}

impl<'a> RecoveryController<'a> {
    pub fn new(
        paths: &'a AppPaths,
        config_store: &'a ConfigStore,
        state_store: &'a StateStore,
        event_store: &'a EventStore,
    ) -> Self {
        Self { paths, config_store, state_store, event_store }
    }

    pub fn capture_snapshot(&self) -> AppResult<NetworkRecoverySnapshot> {
        let manager = MacOsNetworkManager::new(self.paths.clone());
        let services = manager.list_services()?;
        let dns_state = services
            .iter()
            .map(|service| {
                Ok(NetworkServiceSnapshot {
                    service: service.clone(),
                    dns_servers: manager.dns_servers(service)?,
                })
            })
            .collect::<AppResult<Vec<_>>>()?;

        let snapshot = NetworkRecoverySnapshot {
            snapshot_id: Uuid::new_v4().to_string(),
            captured_at: Utc::now(),
            affected_services: services,
            dns_state,
            routing_state: vec!["dns-only".to_owned()],
            restorable: true,
        };
        let payload = serde_json::to_string_pretty(&snapshot).into_diagnostic()?;
        std::fs::write(snapshot.file_path(self.paths), payload).into_diagnostic()?;
        Ok(snapshot)
    }

    pub fn restore_snapshot(&self, snapshot: &NetworkRecoverySnapshot) -> AppResult<()> {
        let manager = MacOsNetworkManager::new(self.paths.clone());
        for service in &snapshot.dns_state {
            manager.set_dns_servers(&service.service, &service.dns_servers)?;
        }
        Ok(())
    }

    pub fn latest_snapshot(&self) -> AppResult<Option<NetworkRecoverySnapshot>> {
        let mut entries = std::fs::read_dir(&self.paths.snapshots_dir)
            .into_diagnostic()?
            .flatten()
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.file_name());
        if let Some(entry) = entries.pop() {
            let content = std::fs::read_to_string(entry.path()).into_diagnostic()?;
            return serde_json::from_str(&content).into_diagnostic().map(Some);
        }
        Ok(None)
    }

    pub fn load_snapshot(
        &self,
        snapshot_id: &str,
    ) -> AppResult<Option<NetworkRecoverySnapshot>> {
        let path = self.paths.snapshots_dir.join(format!("{snapshot_id}.json"));
        match read_file_if_exists(&path)? {
            Some(content) => serde_json::from_str(&content).into_diagnostic().map(Some),
            None => Ok(None),
        }
    }

    pub async fn recover(&self) -> AppResult<RuntimeState> {
        let mut state = self.state_store.load()?;
        if let Some(pid) = state.runtime_pid {
            let _ = runtime::stop_process(pid);
        }

        let snapshot = if let Some(snapshot_id) = state.snapshot_id.as_deref() {
            self.load_snapshot(snapshot_id)?
                .or(self.latest_snapshot()?)
                .ok_or_else(|| miette!("no recoverable network snapshot was found"))?
        } else {
            self.latest_snapshot()?
                .ok_or_else(|| miette!("no recoverable network snapshot was found"))?
        };

        self.restore_snapshot(&snapshot)?;
        let _ = self.config_store.load()?;

        state.mode = ProtectionMode::Inactive;
        state.runtime_pid = None;
        state.runtime_addr = None;
        state.snapshot_id = None;
        state.risk_level = RiskLevel::Normal;
        state.status_summary =
            "Recovery completed. Previous DNS settings were restored.".to_owned();
        state.last_message = Some(
            "Sentinel restored the latest valid snapshot and stopped the local runtime."
                .to_owned(),
        );
        state.last_transition_at = Utc::now();
        self.state_store.save(&state)?;
        self.event_store.append(EventRecord::new(
            EventKind::Recover,
            Severity::Warning,
            "Recovery completed from the latest available snapshot",
        ))?;
        Ok(state)
    }
}
