use std::{collections::BTreeMap, process::Command};

use miette::{IntoDiagnostic, miette};
use serde::{Deserialize, Serialize};

use crate::app::{AppPaths, AppResult, read_file_if_exists};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FakeNetworkState {
    pub services: BTreeMap<String, Vec<String>>,
}

pub struct MacOsNetworkManager {
    paths: AppPaths,
}

impl MacOsNetworkManager {
    pub fn new(paths: AppPaths) -> Self {
        Self { paths }
    }

    fn fake_mode(&self) -> bool {
        std::env::var("SENTINEL_FAKE_PLATFORM").ok().as_deref() == Some("1")
    }

    fn fake_file(&self) -> std::path::PathBuf {
        self.paths.state_dir.join("fake-network.json")
    }

    pub fn list_services(&self) -> AppResult<Vec<String>> {
        if self.fake_mode() {
            let mut state = self.load_fake_state()?;
            if state.services.is_empty() {
                state.services.insert("Wi-Fi".to_owned(), vec!["1.1.1.1".to_owned()]);
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
            .filter(|line| !line.trim().is_empty())
            .filter(|line| !line.starts_with('*'))
            .map(|line| line.trim().to_owned())
            .collect())
    }

    pub fn dns_servers(&self, service: &str) -> AppResult<Vec<String>> {
        if self.fake_mode() {
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
            .map(|line| line.trim().to_owned())
            .filter(|line| !line.is_empty())
            .collect())
    }

    pub fn set_dns_servers(&self, service: &str, servers: &[String]) -> AppResult<()> {
        if self.fake_mode() {
            let mut state = self.load_fake_state()?;
            state.services.insert(service.to_owned(), servers.to_vec());
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
}
