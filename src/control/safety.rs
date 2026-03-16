use crate::{
    app::{AppPaths, AppResult},
    blocking::{blocklist::BlocklistBundle, runtime},
    platform::macos::MacOsNetworkManager,
    storage::state::{RuntimeState, SafetyCheckSummary, SafetyStatus},
};

pub struct SafetyController<'a> {
    paths: &'a AppPaths,
    blocklist: &'a BlocklistBundle,
}

impl<'a> SafetyController<'a> {
    pub fn new(paths: &'a AppPaths, blocklist: &'a BlocklistBundle) -> Self {
        Self { paths, blocklist }
    }

    pub fn run_checks(&self, state: &RuntimeState) -> AppResult<SafetyCheckSummary> {
        let manager = MacOsNetworkManager::new(self.paths.clone());
        let services = manager.list_services()?;
        let mut issues = Vec::new();
        let mut status = SafetyStatus::Pass;
        let mut detected_custom_dns = false;

        if services.is_empty() {
            issues.push("No se detectaron servicios de red activos en macOS.".to_owned());
            status = SafetyStatus::Fail;
        }

        if !self.blocklist.integrity_state {
            issues.push(
                "El bloqueador incluido no paso la validacion de integridad.".to_owned(),
            );
            status = SafetyStatus::Fail;
        }

        let addr = self.paths.runtime_addr()?;
        let simulated_busy =
            std::env::var("SENTINEL_SIMULATE_BUSY_PORT").ok().as_deref() == Some("1");
        let runtime_busy = simulated_busy
            || (!manager.is_fake_mode()
                && !runtime::port_available(addr)
                && !state.runtime_pid.is_some_and(runtime::process_alive));
        if runtime_busy {
            issues.push(format!(
                "El puerto DNS local {} ya esta en uso por otro proceso.",
                addr.port()
            ));
            status = SafetyStatus::Fail;
        }

        if status != SafetyStatus::Fail
            && services
                .iter()
                .any(|service| manager.has_custom_dns(service).unwrap_or(false))
        {
            detected_custom_dns = true;
            issues.push(
                "Se detectaron DNS personalizados; Sentinel los preservara con un snapshot recuperable."
                    .to_owned(),
            );
            status = SafetyStatus::Warn;
        }

        let connectivity_ready = status != SafetyStatus::Fail;
        let recovery_ready = !services.is_empty();
        let recommended_action = match status {
            SafetyStatus::Pass => {
                "Los chequeos aprobaron. Puedes activar la proteccion de forma segura."
                    .to_owned()
            }
            SafetyStatus::Warn => {
                "Los chequeos aprobaron con precauciones. Revisa la nota antes de confirmar."
                    .to_owned()
            }
            SafetyStatus::Fail => {
                "Los chequeos fallaron. Corrige el problema o recupera la red antes de cambiarla."
                    .to_owned()
            }
        };

        let mut summary = SafetyCheckSummary::new(
            status,
            connectivity_ready,
            recovery_ready,
            issues,
            recommended_action,
        );
        summary.detected_custom_dns = detected_custom_dns;
        summary.verification_target =
            Some("snapshot original de DNS por servicio".to_owned());
        Ok(summary)
    }
}
