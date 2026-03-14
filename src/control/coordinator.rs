use crate::{
    app::{AppPaths, AppResult},
    control::snapshot::{NetworkSnapshot, capture},
    platform::macos::MacOsNetworkManager,
};

/// Coordinates snapshot capture, local DNS redirection, and restoration flows.
pub struct Coordinator {
    paths: AppPaths,
    manager: MacOsNetworkManager,
}

impl Coordinator {
    pub fn new(paths: AppPaths) -> Self {
        Self { manager: MacOsNetworkManager::new(paths.clone()), paths }
    }

    pub fn capture_snapshot(&self) -> AppResult<NetworkSnapshot> {
        capture(&self.paths, &self.manager)
    }

    pub fn apply_local_dns(
        &self,
        snapshot: &NetworkSnapshot,
        local_ip: &str,
    ) -> AppResult<()> {
        let target = vec![local_ip.to_owned()];
        for service in &snapshot.services {
            self.manager.set_dns_servers(&service.service, &target)?;
        }
        Ok(())
    }

    pub fn restore_snapshot(&self, snapshot: &NetworkSnapshot) -> AppResult<()> {
        for service in &snapshot.services {
            self.manager.set_dns_servers(&service.service, &service.dns_servers)?;
        }
        Ok(())
    }
}
