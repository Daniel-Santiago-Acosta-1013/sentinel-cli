use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    app::AppResult, platform::macos::MacOsNetworkManager,
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
