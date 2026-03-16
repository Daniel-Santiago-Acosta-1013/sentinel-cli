use std::{collections::BTreeMap, path::PathBuf, process::Command};

use miette::{IntoDiagnostic, miette};
use serde::{Deserialize, Serialize};

use crate::app::{AppPaths, AppResult, read_file_if_exists};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct FakeNetworkState {
    services: BTreeMap<String, Vec<String>>,
}

pub struct MacOsNetworkManager {
    paths: AppPaths,
}

impl MacOsNetworkManager {
    pub fn new(paths: AppPaths) -> Self {
        Self { paths }
    }

    pub fn is_fake_mode(&self) -> bool {
        std::env::var("SENTINEL_FAKE_PLATFORM").ok().as_deref() == Some("1")
    }

    pub fn list_services(&self) -> AppResult<Vec<String>> {
        if self.is_fake_mode() {
            let mut state = self.load_fake_state()?;
            if state.services.is_empty() {
                if let Some(seed) = self.seed_fake_state_from_env()? {
                    state = seed;
                } else {
                    state.services.insert("Wi-Fi".to_owned(), vec!["1.1.1.1".to_owned()]);
                }
                self.save_fake_state(&state)?;
            }
            return Ok(state.services.keys().cloned().collect());
        }

        let output = Command::new("networksetup")
            .arg("-listallnetworkservices")
            .output()
            .into_diagnostic()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout
            .lines()
            .skip(1)
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .filter(|line| !line.starts_with('*'))
            .map(ToOwned::to_owned)
            .collect())
    }

    pub fn dns_servers(&self, service: &str) -> AppResult<Vec<String>> {
        if self.is_fake_mode() {
            return Ok(self
                .load_fake_state()?
                .services
                .get(service)
                .cloned()
                .unwrap_or_default());
        }

        let output = Command::new("networksetup")
            .args(["-getdnsservers", service])
            .output()
            .into_diagnostic()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("There aren't any DNS Servers set") {
            return Ok(Vec::new());
        }
        Ok(stdout
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToOwned::to_owned)
            .collect())
    }

    pub fn set_dns_servers(&self, service: &str, servers: &[String]) -> AppResult<()> {
        if self.is_fake_mode() {
            let mut state = self.load_fake_state()?;
            let should_corrupt_restore =
                std::env::var("SENTINEL_SIMULATE_RESTORE_MISMATCH").ok().as_deref()
                    == Some("1")
                    && servers != ["127.0.0.1".to_owned()];
            let applied = if should_corrupt_restore {
                vec!["203.0.113.53".to_owned()]
            } else {
                servers.to_vec()
            };
            state.services.insert(service.to_owned(), applied);
            self.save_fake_state(&state)?;
            return Ok(());
        }

        let mut command = Command::new("networksetup");
        command.arg("-setdnsservers").arg(service);
        if servers.is_empty() {
            command.arg("Empty");
        } else {
            for server in servers {
                command.arg(server);
            }
        }

        let status = command.status().into_diagnostic()?;
        if status.success() {
            Ok(())
        } else {
            Err(miette!("failed to update DNS for service {service}"))
        }
    }

    pub fn has_custom_dns(&self, service: &str) -> AppResult<bool> {
        Ok(!self.dns_servers(service)?.is_empty())
    }

    fn fake_file(&self) -> PathBuf {
        self.paths.state_dir.join("fake-network.json")
    }

    fn load_fake_state(&self) -> AppResult<FakeNetworkState> {
        match read_file_if_exists(&self.fake_file())? {
            Some(content) => serde_json::from_str(&content).into_diagnostic(),
            None => Ok(FakeNetworkState::default()),
        }
    }

    fn save_fake_state(&self, state: &FakeNetworkState) -> AppResult<()> {
        let payload = serde_json::to_string_pretty(state).into_diagnostic()?;
        std::fs::write(self.fake_file(), payload).into_diagnostic()?;
        Ok(())
    }

    fn seed_fake_state_from_env(&self) -> AppResult<Option<FakeNetworkState>> {
        let Some(seed) = std::env::var("SENTINEL_FAKE_NETWORK_TEMPLATE").ok() else {
            return Ok(None);
        };
        serde_json::from_str(&seed).into_diagnostic().map(Some)
    }
}
