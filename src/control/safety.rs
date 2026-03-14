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

        if services.is_empty() {
            issues.push("No active macOS network services were detected.".to_owned());
            status = SafetyStatus::Fail;
        }

        if !self.blocklist.integrity_state {
            issues.push(
                "The bundled blocklist did not pass integrity validation.".to_owned(),
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
                "The local DNS port {} is already in use by another process.",
                addr.port()
            ));
            status = SafetyStatus::Fail;
        }

        if status != SafetyStatus::Fail
            && services
                .iter()
                .any(|service| manager.has_custom_dns(service).unwrap_or(false))
        {
            issues.push(
                "Custom DNS settings were detected and will be preserved with a recovery snapshot."
                    .to_owned(),
            );
            status = SafetyStatus::Warn;
        }

        let connectivity_ready = status != SafetyStatus::Fail;
        let recovery_ready = !services.is_empty();
        let recommended_action = match status {
            SafetyStatus::Pass => {
                "Safety checks passed. You can enable protection safely.".to_owned()
            }
            SafetyStatus::Warn => {
                "Safety checks passed with cautions. Review the note and confirm before enabling."
                    .to_owned()
            }
            SafetyStatus::Fail => {
                "Safety checks failed. Fix the issue or recover before changing network state."
                    .to_owned()
            }
        };

        Ok(SafetyCheckSummary::new(
            status,
            connectivity_ready,
            recovery_ready,
            issues,
            recommended_action,
        ))
    }
}
