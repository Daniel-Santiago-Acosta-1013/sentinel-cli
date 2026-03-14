use crate::{
    blocking::blocklist::BlocklistBundle,
    storage::{
        install::InstallationState,
        state::{ProtectionMode, RiskLevel, RuntimeState},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    Safety,
    Status,
    Install,
    Recovery,
    Confirm,
    Exit,
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
pub struct InteractiveSession {
    pub screen: Screen,
    pub selected_action: usize,
    pub status_summary: String,
    pub risk_level: RiskLevel,
    pub pending_confirmation: Option<ConfirmationAction>,
    pub last_message: String,
    pub runtime_state: RuntimeState,
    pub install_state: InstallationState,
}

impl InteractiveSession {
    pub fn from_runtime_state(
        runtime_state: RuntimeState,
        install_state: InstallationState,
        blocklist: &BlocklistBundle,
    ) -> Self {
        let mut runtime_state = runtime_state;
        runtime_state.refresh_bundle(blocklist);
        let last_message = runtime_state
            .last_message
            .clone()
            .unwrap_or_else(|| "Sentinel is ready.".to_owned());
        Self {
            screen: Screen::Home,
            selected_action: 0,
            status_summary: runtime_state.status_summary.clone(),
            risk_level: runtime_state.risk_level,
            pending_confirmation: None,
            last_message,
            runtime_state,
            install_state,
        }
    }

    pub fn actions(&self) -> Vec<ActionItem> {
        let toggle_label = if self.runtime_state.mode == ProtectionMode::Active {
            "Disable protection"
        } else {
            "Enable protection"
        };

        vec![
            ActionItem {
                id: ActionId::RunSafetyChecks,
                label: "Run safety checks".to_owned(),
            },
            ActionItem { id: ActionId::ToggleProtection, label: toggle_label.to_owned() },
            ActionItem {
                id: ActionId::ViewStatus,
                label: "View status and health".to_owned(),
            },
            ActionItem {
                id: ActionId::ViewInstallState,
                label: "View install state".to_owned(),
            },
            ActionItem {
                id: ActionId::RecoverNetwork,
                label: "Recover network".to_owned(),
            },
            ActionItem { id: ActionId::Exit, label: "Exit Sentinel".to_owned() },
        ]
    }

    pub fn selected_action_id(&self) -> ActionId {
        self.actions()[self.selected_action].id
    }

    pub fn select_next(&mut self) {
        self.selected_action = (self.selected_action + 1) % self.actions().len();
    }

    pub fn select_previous(&mut self) {
        self.selected_action = if self.selected_action == 0 {
            self.actions().len() - 1
        } else {
            self.selected_action - 1
        };
    }

    pub fn toggle_confirmation_action(&self) -> ConfirmationAction {
        if self.runtime_state.mode == ProtectionMode::Active {
            ConfirmationAction::DisableProtection
        } else {
            ConfirmationAction::EnableProtection
        }
    }

    pub fn sync_runtime_state(&mut self, runtime_state: RuntimeState) {
        self.status_summary = runtime_state.status_summary.clone();
        self.risk_level = runtime_state.risk_level;
        self.last_message = runtime_state
            .last_message
            .clone()
            .unwrap_or_else(|| self.last_message.clone());
        self.runtime_state = runtime_state;
    }
}
