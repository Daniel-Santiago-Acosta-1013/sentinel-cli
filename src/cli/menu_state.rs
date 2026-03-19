use crate::{
    blocking::blocklist::BlocklistBundle,
    storage::{
        events::{BlockActivitySummary, EventRecord},
        install::InstallationState,
        state::{ProtectionMode, RiskLevel, RuntimeState},
    },
};

use super::{
    copy,
    navigation::{
        ConfirmationAction, DomainEditorMode, LogScope, MenuAction, MenuActionId,
        ResultTone, Route, default_route,
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
    pub blocked_domains: Vec<String>,
    pub selected_domain_index: usize,
    pub domain_input: String,
    pub domain_original: Option<String>,
    pub block_activity: BlockActivitySummary,
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
        blocked_domains: Vec<String>,
        block_activity: BlockActivitySummary,
        blocklist: &BlocklistBundle,
        transcript_mode: bool,
    ) -> Self {
        normalize_runtime_copy(&mut runtime_state);
        runtime_state.refresh_bundle(blocklist);
        let route = default_route(runtime_state.mode);
        let last_message = runtime_state.last_message.clone().unwrap_or_else(|| {
            "Sentinel esta listo para activarse, ver su estado o recuperar la red."
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
            blocked_domains,
            selected_domain_index: 0,
            domain_input: String::new(),
            domain_original: None,
            block_activity,
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
                    MenuActionId::OpenSettings,
                    copy::action_label(MenuActionId::OpenSettings, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::OpenSettings,
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
            Route::Settings => vec![
                action(
                    MenuActionId::ViewBlockedDomains,
                    copy::action_label(MenuActionId::ViewBlockedDomains, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::ViewBlockedDomains,
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
            Route::BlockedDomains => vec![
                action(
                    MenuActionId::AddBlockedDomain,
                    copy::action_label(MenuActionId::AddBlockedDomain, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::AddBlockedDomain,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::EditBlockedDomain,
                    copy::action_label(MenuActionId::EditBlockedDomain, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::EditBlockedDomain,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::DeleteBlockedDomain,
                    copy::action_label(
                        MenuActionId::DeleteBlockedDomain,
                        self.runtime_state.mode,
                    ),
                    copy::action_description(
                        MenuActionId::DeleteBlockedDomain,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::SelectNextBlockedDomain,
                    copy::action_label(
                        MenuActionId::SelectNextBlockedDomain,
                        self.runtime_state.mode,
                    ),
                    copy::action_description(
                        MenuActionId::SelectNextBlockedDomain,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::SelectPreviousBlockedDomain,
                    copy::action_label(
                        MenuActionId::SelectPreviousBlockedDomain,
                        self.runtime_state.mode,
                    ),
                    copy::action_description(
                        MenuActionId::SelectPreviousBlockedDomain,
                        self.runtime_state.mode,
                    ),
                ),
                action(
                    MenuActionId::BackSettings,
                    copy::action_label(MenuActionId::BackSettings, self.runtime_state.mode),
                    copy::action_description(
                        MenuActionId::BackSettings,
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
            Route::BlockedDomainEditor(_) | Route::Progress | Route::Exit => Vec::new(),
            Route::Result => vec![
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

    pub fn selected_blocked_domain(&self) -> Option<&str> {
        self.blocked_domains
            .get(self.selected_domain_index)
            .map(String::as_str)
    }

    pub fn select_next_domain(&mut self) {
        if !self.blocked_domains.is_empty() {
            self.selected_domain_index =
                (self.selected_domain_index + 1) % self.blocked_domains.len();
        }
    }

    pub fn select_previous_domain(&mut self) {
        if !self.blocked_domains.is_empty() {
            self.selected_domain_index = if self.selected_domain_index == 0 {
                self.blocked_domains.len() - 1
            } else {
                self.selected_domain_index - 1
            };
        }
    }

    pub fn start_domain_editor(
        &mut self,
        mode: DomainEditorMode,
        initial_value: Option<String>,
    ) {
        self.route = Route::BlockedDomainEditor(mode);
        self.selected_index = 0;
        self.domain_original = initial_value.clone();
        self.domain_input = initial_value.unwrap_or_default();
    }

    pub fn append_domain_input(&mut self, value: &str) {
        self.domain_input.push_str(value);
    }

    pub fn replace_domain_input(&mut self, value: String) {
        self.domain_input = value;
    }

    pub fn pop_domain_input(&mut self) {
        self.domain_input.pop();
    }

    pub fn clear_domain_editor(&mut self) {
        self.domain_input.clear();
        self.domain_original = None;
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

    pub fn sync_blocked_domains(&mut self, blocked_domains: Vec<String>) {
        self.blocked_domains = blocked_domains;
        if self.blocked_domains.is_empty() {
            self.selected_domain_index = 0;
        } else if self.selected_domain_index >= self.blocked_domains.len() {
            self.selected_domain_index = self.blocked_domains.len() - 1;
        }
    }

    pub fn sync_block_activity(&mut self, block_activity: BlockActivitySummary) {
        self.block_activity = block_activity;
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
            "Sentinel esta listo para activarse, ver su estado o recuperar la red."
                .to_owned()
        }
        "Protection is inactive. Run checks before changing the network." => {
            "La proteccion esta inactiva. Puedes activar Sentinel cuando lo necesites."
                .to_owned()
        }
        other => other.to_owned(),
    }
}
