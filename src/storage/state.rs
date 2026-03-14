use miette::IntoDiagnostic;

use crate::{
    app::{AppPaths, AppResult, read_file_if_exists},
    core::state::ProtectionState,
};

pub type RuntimeState = ProtectionState;

pub struct StateStore {
    paths: AppPaths,
}

impl StateStore {
    pub fn new(paths: AppPaths) -> Self {
        Self { paths }
    }

    pub fn load(&self) -> AppResult<RuntimeState> {
        match read_file_if_exists(&self.paths.state_file)? {
            Some(content) => serde_json::from_str(&content).into_diagnostic(),
            None => Ok(RuntimeState::default()),
        }
    }

    pub fn save(&self, state: &RuntimeState) -> AppResult<()> {
        let payload = serde_json::to_string_pretty(state).into_diagnostic()?;
        std::fs::write(&self.paths.state_file, payload).into_diagnostic()?;
        Ok(())
    }
}
