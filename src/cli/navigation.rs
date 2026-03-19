use crate::storage::state::ProtectionMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationAction {
    EnableProtection,
    DisableProtection,
    RecoverNetwork,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogScope {
    Safety,
    Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainEditorMode {
    Add,
    Edit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Home,
    Settings,
    BlockedDomains,
    BlockedDomainEditor(DomainEditorMode),
    Safety,
    Status,
    Logs(LogScope),
    Recovery,
    Confirm(ConfirmationAction),
    Progress,
    Result,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuActionId {
    ToggleProtection,
    ViewStatus,
    OpenSettings,
    ViewBlockedDomains,
    AddBlockedDomain,
    EditBlockedDomain,
    DeleteBlockedDomain,
    SelectNextBlockedDomain,
    SelectPreviousBlockedDomain,
    ViewLogs,
    RecoverNetwork,
    BackToPrevious,
    BackSettings,
    BackHome,
    Exit,
    Confirm,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultTone {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct MenuAction {
    pub id: MenuActionId,
    pub label: String,
    pub description: String,
}

pub fn default_route(mode: ProtectionMode) -> Route {
    if matches!(mode, ProtectionMode::Degraded | ProtectionMode::Recovering) {
        Route::Recovery
    } else {
        Route::Home
    }
}
