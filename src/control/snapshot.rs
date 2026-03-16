use std::path::PathBuf;

use chrono::{DateTime, Utc};
use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::{AppPaths, AppResult, read_file_if_exists},
    platform::macos::MacOsNetworkManager,
    storage::state::RestoreVerification,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkServiceSnapshot {
    pub service: String,
    pub dns_servers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSnapshot {
    pub id: String,
    pub captured_at: DateTime<Utc>,
    pub interface_scope: Vec<String>,
    pub services: Vec<NetworkServiceSnapshot>,
    pub restorable: bool,
}

impl NetworkSnapshot {
    pub fn file_path(&self, paths: &AppPaths) -> PathBuf {
        paths.snapshots_dir.join(format!("{}.json", self.id))
    }
}

pub fn capture(
    paths: &AppPaths,
    manager: &MacOsNetworkManager,
) -> AppResult<NetworkSnapshot> {
    let services = manager.list_services()?;
    let snapshots = services
        .iter()
        .map(|service| {
            Ok(NetworkServiceSnapshot {
                service: service.clone(),
                dns_servers: manager.dns_servers(service)?,
            })
        })
        .collect::<AppResult<Vec<_>>>()?;

    let snapshot = NetworkSnapshot {
        id: Uuid::new_v4().to_string(),
        captured_at: Utc::now(),
        interface_scope: services,
        services: snapshots,
        restorable: true,
    };

    let payload = serde_json::to_string_pretty(&snapshot).into_diagnostic()?;
    std::fs::write(snapshot.file_path(paths), payload).into_diagnostic()?;
    Ok(snapshot)
}

pub fn load_snapshot(
    paths: &AppPaths,
    snapshot_id: &str,
) -> AppResult<Option<NetworkSnapshot>> {
    let path = paths.snapshots_dir.join(format!("{}.json", snapshot_id));
    match read_file_if_exists(&path)? {
        Some(content) => serde_json::from_str(&content).into_diagnostic().map(Some),
        None => Ok(None),
    }
}

pub fn latest_snapshot(paths: &AppPaths) -> AppResult<Option<NetworkSnapshot>> {
    let mut entries = std::fs::read_dir(&paths.snapshots_dir)
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

pub fn inspect_current(
    manager: &MacOsNetworkManager,
) -> AppResult<Vec<NetworkServiceSnapshot>> {
    let services = manager.list_services()?;
    services
        .iter()
        .map(|service| {
            Ok(NetworkServiceSnapshot {
                service: service.clone(),
                dns_servers: manager.dns_servers(service)?,
            })
        })
        .collect()
}

pub fn verify_restoration(
    manager: &MacOsNetworkManager,
    expected: &NetworkSnapshot,
) -> AppResult<RestoreVerification> {
    let current = inspect_current(manager)?;
    let mut mismatched_services = Vec::new();

    for expected_service in &expected.services {
        let Some(found) =
            current.iter().find(|item| item.service == expected_service.service)
        else {
            mismatched_services.push(expected_service.service.clone());
            continue;
        };

        if found.dns_servers != expected_service.dns_servers {
            mismatched_services.push(expected_service.service.clone());
        }
    }

    if mismatched_services.is_empty() {
        Ok(RestoreVerification::success(
            "La red coincide con el snapshot original capturado por Sentinel.",
        ))
    } else {
        Ok(RestoreVerification::failure(
            mismatched_services.clone(),
            format!(
                "La restauracion no coincide con el snapshot en: {}.",
                mismatched_services.join(", ")
            ),
        ))
    }
}
