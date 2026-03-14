use tokio::time::{Duration, sleep};

use crate::{
    app::{AppPaths, AppResult, require_privileges},
    blocking::{blocklist::BlocklistBundle, runtime},
    control::{recovery::RecoveryController, safety::SafetyController},
    storage::{
        config::ConfigStore,
        events::{EventKind, EventRecord, EventStore, Severity},
        state::{ProtectionMode, RiskLevel, RuntimeState, StateStore},
    },
};

pub struct ActivationController<'a> {
    paths: &'a AppPaths,
    config_store: &'a ConfigStore,
    state_store: &'a StateStore,
    event_store: &'a EventStore,
    blocklist: &'a BlocklistBundle,
}

impl<'a> ActivationController<'a> {
    pub fn new(
        paths: &'a AppPaths,
        config_store: &'a ConfigStore,
        state_store: &'a StateStore,
        event_store: &'a EventStore,
        blocklist: &'a BlocklistBundle,
    ) -> Self {
        Self { paths, config_store, state_store, event_store, blocklist }
    }

    /// Enables protection only after safety checks and snapshot capture succeed.
    pub async fn enable(&self) -> AppResult<RuntimeState> {
        require_privileges()?;
        let _ = self.config_store.load()?;
        let mut state = self.state_store.load()?;
        if let Some(pid) = state.runtime_pid
            && state.mode == ProtectionMode::Active
            && runtime::process_alive(pid)
        {
            return Ok(state);
        }

        let safety =
            SafetyController::new(self.paths, self.blocklist).run_checks(&state)?;
        if !safety.connectivity_ready || !safety.recovery_ready {
            state.mode = ProtectionMode::Degraded;
            state.risk_level = RiskLevel::Critical;
            state.status_summary = safety.recommended_action.clone();
            state.last_message = Some(safety.issues.join(" | "));
            state.last_safety_check = Some(safety);
            self.state_store.save(&state)?;
            self.event_store.append(EventRecord::new(
                EventKind::Error,
                Severity::Error,
                "Activation blocked because safety checks failed",
            ))?;
            return Ok(state);
        }

        let recovery = RecoveryController::new(
            self.paths,
            self.config_store,
            self.state_store,
            self.event_store,
        );
        let snapshot = recovery.capture_snapshot()?;
        let runtime_pid = runtime::spawn_background()?;
        sleep(Duration::from_millis(450)).await;
        let manager =
            crate::platform::macos::MacOsNetworkManager::new(self.paths.clone());
        let local_dns = vec!["127.0.0.1".to_owned()];
        if let Err(err) = snapshot
            .affected_services
            .iter()
            .try_for_each(|service| manager.set_dns_servers(service, &local_dns))
        {
            let _ = runtime::stop_process(runtime_pid);
            let _ = recovery.restore_snapshot(&snapshot);
            return Err(err);
        }

        state.mode = ProtectionMode::Active;
        state.risk_level = safety.risk_level();
        state.status_summary =
            "Protection is active. Sentinel is serving filtered DNS locally.".to_owned();
        state.last_message = Some(
            "A recovery snapshot was stored before Sentinel changed DNS settings."
                .to_owned(),
        );
        state.runtime_pid = Some(runtime_pid);
        state.runtime_addr = Some(self.paths.runtime_addr()?);
        state.snapshot_id = Some(snapshot.snapshot_id);
        state.last_transition_at = chrono::Utc::now();
        state.refresh_bundle(self.blocklist);
        state.last_safety_check = Some(safety);
        self.state_store.save(&state)?;
        self.event_store.append(EventRecord::new(
            EventKind::Enable,
            Severity::Info,
            "Protection enabled after successful safety checks",
        ))?;
        Ok(state)
    }

    /// Disables protection and restores the last captured DNS snapshot.
    pub async fn disable(&self) -> AppResult<RuntimeState> {
        require_privileges()?;
        let _ = self.config_store.load()?;
        let mut state = self.state_store.load()?;
        if let Some(pid) = state.runtime_pid {
            runtime::stop_process(pid)?;
        }

        let recovery = RecoveryController::new(
            self.paths,
            self.config_store,
            self.state_store,
            self.event_store,
        );
        if let Some(snapshot_id) = state.snapshot_id.clone()
            && let Some(snapshot) = recovery.load_snapshot(&snapshot_id)?
        {
            recovery.restore_snapshot(&snapshot)?;
        }

        state.mode = ProtectionMode::Inactive;
        state.risk_level = RiskLevel::Normal;
        state.status_summary =
            "Protection is inactive. Original DNS settings are restored.".to_owned();
        state.last_message = Some(
            "Sentinel stopped the local DNS runtime and restored the previous DNS."
                .to_owned(),
        );
        state.runtime_pid = None;
        state.runtime_addr = None;
        state.snapshot_id = None;
        state.last_transition_at = chrono::Utc::now();
        state.refresh_bundle(self.blocklist);
        self.state_store.save(&state)?;
        self.event_store.append(EventRecord::new(
            EventKind::Disable,
            Severity::Info,
            "Protection disabled and DNS restored",
        ))?;
        Ok(state)
    }
}
