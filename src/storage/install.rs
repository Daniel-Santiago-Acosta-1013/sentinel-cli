use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use chrono::{DateTime, Utc};
use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};

use crate::{
    app::{AppPaths, AppResult, read_file_if_exists},
    install::version,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstallAction {
    Install,
    Update,
    Reinstall,
    None,
}

impl InstallAction {
    pub fn label(self) -> &'static str {
        match self {
            Self::Install => "Install",
            Self::Update => "Update",
            Self::Reinstall => "Reinstall",
            Self::None => "None",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationState {
    pub installed: bool,
    pub path_entry: Option<String>,
    pub installed_version: Option<String>,
    pub target_version: String,
    pub action: InstallAction,
    pub last_install_result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InstallMetadata {
    installed_at: Option<DateTime<Utc>>,
    installed_path: Option<String>,
    installed_version: Option<String>,
    last_action: InstallAction,
    last_install_result: String,
}

impl Default for InstallMetadata {
    fn default() -> Self {
        Self {
            installed_at: None,
            installed_path: None,
            installed_version: None,
            last_action: InstallAction::Install,
            last_install_result: "No installer action recorded yet.".to_owned(),
        }
    }
}

pub struct InstallStore {
    paths: AppPaths,
}

impl InstallStore {
    pub fn new(paths: AppPaths) -> Self {
        Self { paths }
    }

    pub fn inspect_current(&self) -> AppResult<InstallationState> {
        let metadata = self.load_metadata()?;
        let target_version = version::current_version().to_owned();
        let install_path = discover_install_path(metadata.installed_path.as_deref());
        let installed_version =
            install_path.as_ref().and_then(|path| read_binary_version(path));
        let action = version::decide_action(
            installed_version.as_deref(),
            install_path.is_some() && installed_version.is_none(),
        );

        Ok(InstallationState {
            installed: install_path.is_some() && installed_version.is_some(),
            path_entry: install_path.map(|path| path.display().to_string()),
            installed_version,
            target_version,
            action,
            last_install_result: metadata.last_install_result,
        })
    }

    #[allow(dead_code)]
    pub fn note_result(
        &self,
        install_path: Option<&Path>,
        action: InstallAction,
        result: impl Into<String>,
    ) -> AppResult<()> {
        let installed_version = install_path.and_then(read_binary_version);
        let metadata = InstallMetadata {
            installed_at: Some(Utc::now()),
            installed_path: install_path.map(|path| path.display().to_string()),
            installed_version,
            last_action: action,
            last_install_result: result.into(),
        };
        let payload = serde_json::to_string_pretty(&metadata).into_diagnostic()?;
        std::fs::write(&self.paths.install_file, payload).into_diagnostic()?;
        Ok(())
    }

    fn load_metadata(&self) -> AppResult<InstallMetadata> {
        match read_file_if_exists(&self.paths.install_file)? {
            Some(content) => serde_json::from_str(&content).into_diagnostic(),
            None => Ok(InstallMetadata::default()),
        }
    }
}

fn discover_install_path(metadata_path: Option<&str>) -> Option<PathBuf> {
    if let Ok(dir) = env::var("SENTINEL_INSTALL_DIR") {
        let candidate = PathBuf::from(dir).join("sentinel");
        if candidate.exists() {
            return Some(candidate);
        }
    }

    if let Some(path) = metadata_path {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    std::env::current_exe()
        .ok()
        .filter(|path| path.file_name().is_some_and(|name| name == "sentinel"))
        .filter(|path| {
            !path.components().any(|component| {
                component.as_os_str() == "target" || component.as_os_str() == "deps"
            })
        })
}

fn read_binary_version(path: &Path) -> Option<String> {
    if !path.exists() {
        return None;
    }

    let output = Command::new(path)
        .env("SENTINEL_INTERNAL_MODE", "print-version")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let version = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if version.is_empty() { None } else { Some(version) }
}
