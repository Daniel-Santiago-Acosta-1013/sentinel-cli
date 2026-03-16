use crate::storage::{install::InstallAction, state::ProtectionMode};

use super::menu_state::{ActionId, ConfirmationAction, ViewId};

pub fn app_title() -> &'static str {
    "Sentinel"
}

pub fn app_subtitle() -> &'static str {
    "CLI guiada para proteger tu DNS sin comprometer la red del equipo."
}

pub fn footer_hint() -> &'static str {
    "Usa ↑/↓ para navegar, Enter para confirmar, Esc para volver y q para salir."
}

pub fn view_title(view: ViewId) -> &'static str {
    match view {
        ViewId::Inicio => "Inicio",
        ViewId::Seguridad => "Seguridad",
        ViewId::Estado => "Estado",
        ViewId::Instalacion => "Instalacion",
        ViewId::Recuperacion => "Recuperacion",
        ViewId::Confirmacion => "Confirmacion",
        ViewId::Salida => "Salida",
    }
}

pub fn intro_text(view: ViewId, mode: ProtectionMode) -> &'static str {
    match view {
        ViewId::Inicio
            if matches!(mode, ProtectionMode::Degraded | ProtectionMode::Recovering) =>
        {
            "Sentinel detecto un estado que requiere recuperacion antes de permitir nuevos cambios."
        }
        ViewId::Inicio => {
            "Elige una accion principal. Sentinel mantiene el flujo corto, guiado y seguro."
        }
        ViewId::Seguridad => {
            "Revisa si el equipo esta listo para activar proteccion sin comprometer conectividad."
        }
        ViewId::Estado => {
            "Consulta el estado actual de proteccion, runtime, snapshot y siguiente accion recomendada."
        }
        ViewId::Instalacion => {
            "Consulta el estado de instalacion. Los cambios de instalacion se hacen con el script oficial."
        }
        ViewId::Recuperacion => {
            "Recupera la configuracion original de red y valida que el equipo quedo en un estado seguro."
        }
        ViewId::Confirmacion => {
            "La accion seleccionada cambia o restaura la red. Confirma solo si deseas continuar."
        }
        ViewId::Salida => "La sesion termino sin dejar texto residual en la terminal.",
    }
}

pub fn action_label(action: ActionId, mode: ProtectionMode) -> String {
    match action {
        ActionId::RunSafetyChecks => "Ejecutar chequeos de seguridad".to_owned(),
        ActionId::ToggleProtection => {
            if mode == ProtectionMode::Active {
                "Desactivar proteccion".to_owned()
            } else {
                "Activar proteccion".to_owned()
            }
        }
        ActionId::ViewStatus => "Ver estado actual".to_owned(),
        ActionId::ViewInstallState => "Ver estado de instalacion".to_owned(),
        ActionId::RecoverNetwork => "Recuperar red".to_owned(),
        ActionId::Exit => "Salir de Sentinel".to_owned(),
    }
}

pub fn confirmation_label(action: ConfirmationAction) -> &'static str {
    match action {
        ConfirmationAction::EnableProtection => "Activar proteccion",
        ConfirmationAction::DisableProtection => "Desactivar proteccion",
        ConfirmationAction::RecoverNetwork => "Recuperar red",
    }
}

pub fn install_action_label(action: InstallAction) -> &'static str {
    match action {
        InstallAction::Install => "Instalar",
        InstallAction::Update => "Actualizar",
        InstallAction::Reinstall => "Reinstalar",
        InstallAction::None => "Sin accion",
    }
}
