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
        self.event_store.record_safety(&safety)?;
        if !safety.connectivity_ready || !safety.recovery_ready {
            state.mode = ProtectionMode::Degraded;
            state.risk_level = RiskLevel::Critical;
            state.status_summary = safety.recommended_action.clone();
            state.last_message = Some(if safety.issues.is_empty() {
                safety.recommended_action.clone()
            } else {
                safety.issues.join(" | ")
            });
            state.last_safety_check = Some(safety);
            self.state_store.save(&state)?;
            self.event_store.append(EventRecord::new(
                EventKind::Error,
                Severity::Error,
                "La activacion fue bloqueada porque fallaron los chequeos de seguridad",
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
        state.mode = ProtectionMode::Recovering;
        state.risk_level = RiskLevel::Warning;
        state.snapshot_id = Some(snapshot.snapshot_id.clone());
        state.status_summary =
            "Sentinel esta preparando un cambio de red con snapshot recuperable."
                .to_owned();
        state.last_message = Some(
            "Se capturo la configuracion original antes de activar la proteccion."
                .to_owned(),
        );
        state.last_transition_at = chrono::Utc::now();
        state.last_safety_check = Some(safety.clone());
        state.last_verification_result = None;
        self.state_store.save(&state)?;

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
            let verification = crate::control::snapshot::verify_restoration(
                &manager,
                &crate::control::snapshot::NetworkSnapshot {
                    id: snapshot.snapshot_id.clone(),
                    captured_at: snapshot.captured_at,
                    interface_scope: snapshot.affected_services.clone(),
                    services: snapshot
                        .dns_state
                        .iter()
                        .map(|service| crate::control::snapshot::NetworkServiceSnapshot {
                            service: service.service.clone(),
                            dns_servers: service.dns_servers.clone(),
                        })
                        .collect(),
                    restorable: snapshot.restorable,
                },
            )?;
            state.mode = ProtectionMode::Degraded;
            state.risk_level = RiskLevel::Critical;
            state.status_summary = verification.summary.clone();
            state.last_message = Some(format!(
                "La activacion fallo y Sentinel intento restaurar la red: {err}"
            ));
            state.last_verification_result = Some(verification);
            self.state_store.save(&state)?;
            self.event_store.append(EventRecord::new(
                EventKind::Error,
                Severity::Error,
                "La activacion fallo durante el cambio de DNS y la red fue restaurada",
            ))?;
            return Ok(state);
        }

        state.mode = ProtectionMode::Active;
        state.risk_level = safety.risk_level();
        state.status_summary =
            "La proteccion esta activa. Sentinel responde DNS filtrado localmente."
                .to_owned();
        state.last_message = Some(
            "Sentinel guardo un snapshot recuperable antes de cambiar el DNS.".to_owned(),
        );
        state.runtime_pid = Some(runtime_pid);
        state.runtime_addr = Some(self.paths.runtime_addr()?);
        state.snapshot_id = Some(snapshot.snapshot_id);
        state.last_transition_at = chrono::Utc::now();
        state.refresh_bundle(self.blocklist);
        state.last_safety_check = Some(safety);
        state.last_verification_result = None;
        self.state_store.save(&state)?;
        self.event_store.append(EventRecord::new(
            EventKind::Enable,
            Severity::Info,
            "Proteccion activada despues de chequeos de seguridad exitosos",
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
        state.mode = ProtectionMode::Recovering;
        state.risk_level = RiskLevel::Warning;
        state.status_summary =
            "Sentinel esta restaurando la configuracion original de red.".to_owned();
        state.last_message = Some(
            "Se detuvo el runtime local y ahora se restaurara el snapshot original."
                .to_owned(),
        );
        state.runtime_pid = None;
        state.runtime_addr = None;
        state.last_transition_at = chrono::Utc::now();
        self.state_store.save(&state)?;

        if let Some(snapshot_id) = state.snapshot_id.clone()
            && let Some(snapshot) = recovery.load_snapshot(&snapshot_id)?
        {
            recovery.restore_snapshot(&snapshot)?;
            let manager =
                crate::platform::macos::MacOsNetworkManager::new(self.paths.clone());
            let verification = crate::control::snapshot::verify_restoration(
                &manager,
                &crate::control::snapshot::NetworkSnapshot {
                    id: snapshot.snapshot_id.clone(),
                    captured_at: snapshot.captured_at,
                    interface_scope: snapshot.affected_services.clone(),
                    services: snapshot
                        .dns_state
                        .iter()
                        .map(|service| crate::control::snapshot::NetworkServiceSnapshot {
                            service: service.service.clone(),
                            dns_servers: service.dns_servers.clone(),
                        })
                        .collect(),
                    restorable: snapshot.restorable,
                },
            )?;

            if verification.matches_snapshot {
                state.mode = ProtectionMode::Inactive;
                state.risk_level = RiskLevel::Normal;
                state.status_summary =
                    "La proteccion esta inactiva y la red original fue restaurada."
                        .to_owned();
                state.last_message = Some(
                    "Sentinel restauro y verifico correctamente la configuracion original de DNS."
                        .to_owned(),
                );
                state.snapshot_id = None;
            } else {
                state.mode = ProtectionMode::Degraded;
                state.risk_level = RiskLevel::Critical;
                state.status_summary = verification.summary.clone();
                state.last_message = Some(
                    "Sentinel desactivo la proteccion, pero detecto diferencias frente al snapshot original."
                        .to_owned(),
                );
            }
            state.last_verification_result = Some(verification.clone());
            state.last_transition_at = chrono::Utc::now();
            state.refresh_bundle(self.blocklist);
            self.state_store.save(&state)?;
            self.event_store.append(EventRecord::new(
                EventKind::Disable,
                if verification.matches_snapshot {
                    Severity::Info
                } else {
                    Severity::Error
                },
                if verification.matches_snapshot {
                    "Proteccion desactivada y red restaurada con verificacion exitosa"
                } else {
                    "Proteccion desactivada, pero la restauracion no coincidio con el snapshot"
                },
            ))?;
            return Ok(state);
        }

        state.mode = ProtectionMode::Degraded;
        state.risk_level = RiskLevel::Critical;
        state.status_summary =
            "No se encontro un snapshot recuperable para restaurar la red.".to_owned();
        state.last_message =
            Some("Sentinel no pudo completar la desactivacion segura porque falta el snapshot.".to_owned());
        state.last_transition_at = chrono::Utc::now();
        self.state_store.save(&state)?;
        self.event_store.append(EventRecord::new(
            EventKind::Error,
            Severity::Error,
            "No se encontro un snapshot recuperable durante la desactivacion",
        ))?;
        Ok(state)
    }
}
