use miette::miette;

use crate::{
    app::{AppPaths, AppResult},
    control::{
        coordinator::Coordinator,
        snapshot::{latest_snapshot, load_snapshot},
    },
};

pub struct RecoveryManager {
    paths: AppPaths,
}

impl RecoveryManager {
    pub fn new(paths: AppPaths) -> Self {
        Self { paths }
    }

    pub fn restore_latest_or_active(&self, snapshot_id: Option<String>) -> AppResult<()> {
        let coordinator = Coordinator::new(self.paths.clone());
        if let Some(snapshot_id) = snapshot_id {
            if let Some(snapshot) = load_snapshot(&self.paths, &snapshot_id)? {
                return coordinator.restore_snapshot(&snapshot);
            }
        }

        if let Some(snapshot) = latest_snapshot(&self.paths)? {
            return coordinator.restore_snapshot(&snapshot);
        }

        Err(miette!("no recoverable network snapshot was found"))
    }

    pub fn restore_if_available(&self, snapshot_id: Option<String>) -> AppResult<bool> {
        let coordinator = Coordinator::new(self.paths.clone());
        if let Some(snapshot_id) = snapshot_id {
            if let Some(snapshot) = load_snapshot(&self.paths, &snapshot_id)? {
                coordinator.restore_snapshot(&snapshot)?;
                return Ok(true);
            }
        }

        if let Some(snapshot) = latest_snapshot(&self.paths)? {
            coordinator.restore_snapshot(&snapshot)?;
            return Ok(true);
        }

        Ok(false)
    }
}
