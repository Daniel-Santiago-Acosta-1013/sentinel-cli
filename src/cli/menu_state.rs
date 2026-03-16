use crate::{
    blocking::blocklist::BlocklistBundle,
    storage::{
        install::InstallationState,
        state::{ProtectionMode, RiskLevel, RuntimeState},
    },
};

use super::copy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewId {
    Inicio,
    Seguridad,
    Estado,
    Instalacion,
    Recuperacion,
    Confirmacion,
    Salida,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionId {
    RunSafetyChecks,
    ToggleProtection,
    ViewStatus,
    ViewInstallState,
    RecoverNetwork,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationAction {
    EnableProtection,
    DisableProtection,
    RecoverNetwork,
}

#[derive(Debug, Clone)]
pub struct ActionItem {
    pub id: ActionId,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct MenuSession {
    pub view: ViewId,
    pub selected_index: usize,
    pub status_summary: String,
    pub risk_level: RiskLevel,
    pub pending_confirmation: Option<ConfirmationAction>,
    pub last_message: String,
    pub runtime_state: RuntimeState,
    pub install_state: InstallationState,
}

impl MenuSession {
    pub fn from_runtime_state(
        mut runtime_state: RuntimeState,
        install_state: InstallationState,
        blocklist: &BlocklistBundle,
        _transcript_mode: bool,
    ) -> Self {
        normalize_runtime_copy(&mut runtime_state);
        runtime_state.refresh_bundle(blocklist);
        let view = if matches!(
            runtime_state.mode,
            ProtectionMode::Degraded | ProtectionMode::Recovering
        ) {
            ViewId::Recuperacion
        } else {
            ViewId::Inicio
        };
        let selected_index = if view == ViewId::Recuperacion { 1 } else { 0 };
        let last_message = runtime_state.last_message.clone().unwrap_or_else(|| {
            "Sentinel esta listo para revisar seguridad o cambiar proteccion.".to_owned()
        });
        Self {
            view,
            selected_index,
            status_summary: runtime_state.status_summary.clone(),
            risk_level: runtime_state.risk_level,
            pending_confirmation: None,
            last_message,
            runtime_state,
            install_state,
        }
    }

    pub fn actions(&self) -> Vec<ActionItem> {
        if matches!(
            self.runtime_state.mode,
            ProtectionMode::Degraded | ProtectionMode::Recovering
        ) {
            return vec![
                ActionItem {
                    id: ActionId::ViewStatus,
                    label: copy::action_label(
                        ActionId::ViewStatus,
                        self.runtime_state.mode,
                    ),
                },
                ActionItem {
                    id: ActionId::RecoverNetwork,
                    label: copy::action_label(
                        ActionId::RecoverNetwork,
                        self.runtime_state.mode,
                    ),
                },
                ActionItem {
                    id: ActionId::Exit,
                    label: copy::action_label(ActionId::Exit, self.runtime_state.mode),
                },
            ];
        }

        vec![
            ActionItem {
                id: ActionId::RunSafetyChecks,
                label: copy::action_label(
                    ActionId::RunSafetyChecks,
                    self.runtime_state.mode,
                ),
            },
            ActionItem {
                id: ActionId::ToggleProtection,
                label: copy::action_label(
                    ActionId::ToggleProtection,
                    self.runtime_state.mode,
                ),
            },
            ActionItem {
                id: ActionId::ViewStatus,
                label: copy::action_label(ActionId::ViewStatus, self.runtime_state.mode),
            },
            ActionItem {
                id: ActionId::ViewInstallState,
                label: copy::action_label(
                    ActionId::ViewInstallState,
                    self.runtime_state.mode,
                ),
            },
            ActionItem {
                id: ActionId::RecoverNetwork,
                label: copy::action_label(
                    ActionId::RecoverNetwork,
                    self.runtime_state.mode,
                ),
            },
            ActionItem {
                id: ActionId::Exit,
                label: copy::action_label(ActionId::Exit, self.runtime_state.mode),
            },
        ]
    }

    pub fn selected_action_id(&self) -> ActionId {
        self.actions()[self.selected_index].id
    }

    pub fn select_next(&mut self) {
        let len = self.actions().len();
        self.selected_index = (self.selected_index + 1) % len;
    }

    pub fn select_previous(&mut self) {
        let len = self.actions().len();
        self.selected_index =
            if self.selected_index == 0 { len - 1 } else { self.selected_index - 1 };
    }

    pub fn toggle_confirmation_action(&self) -> ConfirmationAction {
        if self.runtime_state.mode == ProtectionMode::Active {
            ConfirmationAction::DisableProtection
        } else {
            ConfirmationAction::EnableProtection
        }
    }

    pub fn sync_runtime_state(&mut self, runtime_state: RuntimeState) {
        let mut runtime_state = runtime_state;
        normalize_runtime_copy(&mut runtime_state);
        self.status_summary = runtime_state.status_summary.clone();
        self.risk_level = runtime_state.risk_level;
        self.last_message = runtime_state
            .last_message
            .clone()
            .unwrap_or_else(|| self.last_message.clone());
        self.runtime_state = runtime_state;
        let len = self.actions().len();
        if self.selected_index >= len {
            self.selected_index = len.saturating_sub(1);
        }
        if matches!(
            self.runtime_state.mode,
            ProtectionMode::Degraded | ProtectionMode::Recovering
        ) {
            self.view = ViewId::Recuperacion;
            self.selected_index = self
                .actions()
                .iter()
                .position(|item| item.id == ActionId::RecoverNetwork)
                .unwrap_or(0);
        }
    }
}

fn normalize_runtime_copy(runtime_state: &mut RuntimeState) {
    runtime_state.status_summary = normalize_copy(&runtime_state.status_summary);
    runtime_state.last_message =
        runtime_state.last_message.take().map(|message| normalize_copy(&message));

    if let Some(check) = runtime_state.last_safety_check.as_mut() {
        check.recommended_action = normalize_copy(&check.recommended_action);
        check.issues =
            check.issues.iter().map(|issue| normalize_copy(issue)).collect();
    }

    if let Some(verification) = runtime_state.last_verification_result.as_mut() {
        verification.summary = normalize_copy(&verification.summary);
    }
}

fn normalize_copy(input: &str) -> String {
    match input {
        "Sentinel restored the latest valid snapshot and stopped the local runtime." => {
            "Sentinel restauro el ultimo snapshot valido y detuvo el runtime local."
                .to_owned()
        }
        "Sentinel is ready to inspect safety and activate protection." => {
            "Sentinel esta listo para inspeccionar seguridad y activar proteccion."
                .to_owned()
        }
        "Protection is inactive. Run checks before changing the network." => {
            "La proteccion esta inactiva. Ejecuta chequeos antes de cambiar la red."
                .to_owned()
        }
        other => other.to_owned(),
    }
}
