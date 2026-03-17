use crate::{
    blocking::blocklist::BlocklistBundle,
    storage::{
        events::EventRecord,
        install::InstallationState,
        state::{ProtectionMode, RiskLevel, RuntimeState},
    },
};

use super::{
    copy,
    navigation::{
        ConfirmationAction, LogScope, MenuAction, MenuActionId, ResultTone, Route,
        default_route,
    },
};

#[derive(Debug, Clone)]
pub struct OperationResult {
    pub title: String,
    pub summary: String,
    pub next_step: String,
    pub tone: ResultTone,
}

#[derive(Debug, Clone)]
pub struct MenuSession {
    pub route: Route,
    pub selected_index: usize,
    pub status_summary: String,
    pub risk_level: RiskLevel,
    pub last_message: String,
    pub runtime_state: RuntimeState,
    pub install_state: InstallationState,
    pub recent_events: Vec<EventRecord>,
    pub transcript_mode: bool,
    pub last_result: Option<OperationResult>,
    pub progress_label: Option<String>,
    pub progress_step: usize,
}

impl MenuSession {
    pub fn from_runtime_state(
        mut runtime_state: RuntimeState,
        install_state: InstallationState,
        recent_events: Vec<EventRecord>,
        blocklist: &BlocklistBundle,
        transcript_mode: bool,
    ) -> Self {
        normalize_runtime_copy(&mut runtime_state);
        runtime_state.refresh_bundle(blocklist);
        let route = default_route(runtime_state.mode);
        let last_message = runtime_state.last_message.clone().unwrap_or_else(|| {
            "Sentinel esta listo para revisar seguridad, ver el estado o cambiar la proteccion."
                .to_owned()
        });
        Self {
            route,
            selected_index: 0,
            status_summary: runtime_state.status_summary.clone(),
            risk_level: runtime_state.risk_level,
            last_message,
            runtime_state,
            install_state,
            recent_events,
            transcript_mode,
            last_result: None,
            progress_label: None,
            progress_step: 0,
        }
    }

    pub fn actions(&self) -> Vec<MenuAction> {
        match self.route {
            Route::Home => vec![
                action(
                    MenuActionId::RunSafetyChecks,
                    copy::action_label(
                        MenuActionId::RunSafetyChecks,
                        self.runtime_state.mode,
                    ),
                    copy::action_description(
                        MenuActionId::RunSafetyChecks,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::ToggleProtection,
                    copy::action_label(
                        MenuActionId::ToggleProtection,
                        self.runtime_state.mode,
                    ),
                    copy::action_description(
                        MenuActionId::ToggleProtection,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::ViewStatus,
                    copy::action_label(MenuActionId::ViewStatus, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::ViewStatus,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::RecoverNetwork,
                    copy::action_label(
                        MenuActionId::RecoverNetwork,
                        self.runtime_state.mode,
                    ),
                    copy::action_description(
                        MenuActionId::RecoverNetwork,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::Exit,
                    copy::action_label(MenuActionId::Exit, self.runtime_state.mode),
                    copy::action_description(MenuActionId::Exit, self.runtime_state.mode),
                ),
            ],
            Route::Safety => vec![
                action(
                    MenuActionId::ViewLogs,
                    copy::action_label(MenuActionId::ViewLogs, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::ViewLogs,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::BackHome,
                    copy::action_label(MenuActionId::BackHome, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::BackHome,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::Exit,
                    copy::action_label(MenuActionId::Exit, self.runtime_state.mode),
                    copy::action_description(MenuActionId::Exit, self.runtime_state.mode),
                ),
            ],
            Route::Status => vec![
                action(
                    MenuActionId::ViewLogs,
                    copy::action_label(MenuActionId::ViewLogs, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::ViewLogs,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::BackHome,
                    copy::action_label(MenuActionId::BackHome, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::BackHome,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::Exit,
                    copy::action_label(MenuActionId::Exit, self.runtime_state.mode),
                    copy::action_description(MenuActionId::Exit, self.runtime_state.mode),
                ),
            ],
            Route::Logs(_) => vec![
                action(
                    MenuActionId::BackToPrevious,
                    copy::action_label(
                        MenuActionId::BackToPrevious,
                        self.runtime_state.mode,
                    ),
                    copy::action_description(
                        MenuActionId::BackToPrevious,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::Exit,
                    copy::action_label(MenuActionId::Exit, self.runtime_state.mode),
                    copy::action_description(MenuActionId::Exit, self.runtime_state.mode),
                ),
            ],
            Route::Recovery => vec![
                action(
                    MenuActionId::RecoverNetwork,
                    copy::action_label(
                        MenuActionId::RecoverNetwork,
                        self.runtime_state.mode,
                    ),
                    copy::action_description(
                        MenuActionId::RecoverNetwork,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::ViewStatus,
                    copy::action_label(MenuActionId::ViewStatus, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::ViewStatus,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::BackHome,
                    copy::action_label(MenuActionId::BackHome, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::BackHome,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::Exit,
                    copy::action_label(MenuActionId::Exit, self.runtime_state.mode),
                    copy::action_description(MenuActionId::Exit, self.runtime_state.mode),
                ),
            ],
            Route::Confirm(_) => vec![
                action(
                    MenuActionId::Confirm,
                    copy::action_label(MenuActionId::Confirm, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::Confirm,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::Cancel,
                    copy::action_label(MenuActionId::Cancel, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::Cancel,
                        self.runtime_state.mode,
                    ),
                ),
            ],
            Route::Progress => Vec::new(),
            Route::Exit => Vec::new(),
            _ => vec![
                action(
                    MenuActionId::BackHome,
                    copy::action_label(MenuActionId::BackHome, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::BackHome,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::Exit,
                    copy::action_label(MenuActionId::Exit, self.runtime_state.mode),
                    copy::action_description(MenuActionId::Exit, self.runtime_state.mode),
                ),
            ],
        }
    }

    pub fn selected_action_id(&self) -> Option<MenuActionId> {
        self.actions().get(self.selected_index).map(|item| item.id)
    }

    pub fn select_next(&mut self) {
        let len = self.actions().len();
        if len != 0 {
            self.selected_index = (self.selected_index + 1) % len;
        }
    }

    pub fn select_previous(&mut self) {
        let len = self.actions().len();
        if len != 0 {
            self.selected_index =
                if self.selected_index == 0 { len - 1 } else { self.selected_index - 1 };
        }
    }

    pub fn pending_confirmation(&self) -> ConfirmationAction {
        match self.route {
            Route::Confirm(action) => action,
            other => panic!("route {other:?} is not a confirmation route"),
        }
    }

    pub fn toggle_confirmation_action(&self) -> ConfirmationAction {
        if self.runtime_state.mode == ProtectionMode::Active {
            ConfirmationAction::DisableProtection
        } else {
            ConfirmationAction::EnableProtection
        }
    }

    pub fn progress_label(&self) -> Option<&str> {
        self.progress_label.as_deref()
    }

    pub fn show_result(
        &mut self,
        title: impl Into<String>,
        summary: impl Into<String>,
        next_step: impl Into<String>,
        tone: ResultTone,
    ) {
        self.last_result = Some(OperationResult {
            title: title.into(),
            summary: summary.into(),
            next_step: next_step.into(),
            tone,
        });
        self.route = Route::Result;
        self.selected_index = 0;
        self.progress_label = None;
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
    }

    pub fn sync_recent_events(&mut self, recent_events: Vec<EventRecord>) {
        self.recent_events = recent_events;
    }

    pub fn log_scope(&self) -> Option<LogScope> {
        match self.route {
            Route::Logs(scope) => Some(scope),
            Route::Safety => Some(LogScope::Safety),
            Route::Status => Some(LogScope::Status),
            _ => None,
        }
    }
}

fn action(id: MenuActionId, label: String, description: String) -> MenuAction {
    MenuAction { id, label, description }
}

fn normalize_runtime_copy(runtime_state: &mut RuntimeState) {
    runtime_state.status_summary = normalize_copy(&runtime_state.status_summary);
    runtime_state.last_message =
        runtime_state.last_message.take().map(|message| normalize_copy(&message));

    if let Some(check) = runtime_state.last_safety_check.as_mut() {
        check.recommended_action = normalize_copy(&check.recommended_action);
        check.issues = check.issues.iter().map(|issue| normalize_copy(issue)).collect();
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
