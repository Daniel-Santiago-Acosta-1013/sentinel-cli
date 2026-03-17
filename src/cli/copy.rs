use crate::{install::version, storage::state::ProtectionMode};

use super::navigation::{ConfirmationAction, LogScope, MenuActionId, Route};

pub fn app_subtitle() -> String {
    format!("Version {}", version::current_version())
}

pub fn route_title(route: Route) -> &'static str {
    match route {
        Route::Home => "Inicio",
        Route::Safety => "Chequeos de seguridad",
        Route::Status => "Estado de Sentinel",
        Route::Logs(LogScope::Safety) => "Logs del chequeo",
        Route::Logs(LogScope::Status) => "Logs de Sentinel",
        Route::Recovery => "Recuperacion",
        Route::Confirm(_) => "Confirmacion",
        Route::Progress => "Procesando",
        Route::Result => "Resultado",
        Route::Exit => "Salida",
    }
}

pub fn intro_text(route: Route, mode: ProtectionMode) -> &'static str {
    match route {
        Route::Home
            if matches!(mode, ProtectionMode::Degraded | ProtectionMode::Recovering) =>
        {
            "Sentinel detecto un estado sensible. Puedes priorizar la recuperacion o revisar el estado antes de hacer nuevos cambios."
        }
        Route::Home => {
            "Elige una accion principal. Cada seleccion abre una vista independiente y limpia."
        }
        Route::Safety => {
            "Revisa si el equipo esta listo para cambiar la proteccion sin comprometer conectividad."
        }
        Route::Status => {
            "Consulta el estado actual de Sentinel, su runtime, el snapshot y los siguientes pasos."
        }
        Route::Logs(LogScope::Safety) => {
            "Revisa los eventos exactos del ultimo chequeo para diagnosticar fallos y decidir el siguiente paso."
        }
        Route::Logs(LogScope::Status) => {
            "Revisa los eventos recientes de Sentinel con su severidad, momento y mensaje exacto."
        }
        Route::Recovery => {
            "Usa esta vista para recuperar la red o revisar el estado cuando Sentinel detecta un problema."
        }
        Route::Confirm(_) => {
            "Esta accion puede modificar o restaurar la red. Confirma solo si deseas continuar."
        }
        Route::Progress => "Sentinel esta ejecutando la accion solicitada.",
        Route::Result => "Revisa el resultado antes de volver al inicio o salir.",
        Route::Exit => "La sesion interactiva termino sin dejar texto residual.",
    }
}

pub fn footer_hint(route: Route) -> &'static str {
    match route {
        Route::Progress => "Espera a que termine la operacion.",
        Route::Home | Route::Recovery | Route::Confirm(_) | Route::Result => {
            "Usa ↑/↓ para navegar, Enter para confirmar, Esc para volver y q para salir."
        }
        _ => "Usa ↑/↓ para elegir, Enter para continuar, Esc para volver y q para salir.",
    }
}

pub fn action_label(action: MenuActionId, mode: ProtectionMode) -> String {
    match action {
        MenuActionId::RunSafetyChecks => "Ejecutar chequeos de seguridad".to_owned(),
        MenuActionId::ToggleProtection => {
            if mode == ProtectionMode::Active {
                "Desactivar Sentinel".to_owned()
            } else {
                "Activar Sentinel".to_owned()
            }
        }
        MenuActionId::ViewStatus => "Ver estado de Sentinel".to_owned(),
        MenuActionId::ViewLogs => "Logs".to_owned(),
        MenuActionId::RecoverNetwork => "Recuperar red".to_owned(),
        MenuActionId::BackToPrevious => "Volver a la vista anterior".to_owned(),
        MenuActionId::BackHome => "Volver al inicio".to_owned(),
        MenuActionId::Exit => "Salir de Sentinel".to_owned(),
        MenuActionId::Confirm => "Confirmar accion".to_owned(),
        MenuActionId::Cancel => "Cancelar y volver".to_owned(),
    }
}

pub fn action_description(action: MenuActionId, mode: ProtectionMode) -> String {
    match action {
        MenuActionId::RunSafetyChecks => {
            "Valida conectividad, snapshot recuperable y condiciones previas antes de tocar la red."
                .to_owned()
        }
        MenuActionId::ToggleProtection if mode == ProtectionMode::Active => {
            "Restaura la configuracion original y detiene Sentinel."
                .to_owned()
        }
        MenuActionId::ToggleProtection => {
            "Inicia Sentinel y activa el bloqueo DNS local con snapshot recuperable."
                .to_owned()
        }
        MenuActionId::ViewStatus => {
            "Muestra el estado actual de Sentinel, su runtime, el snapshot y la instalacion."
                .to_owned()
        }
        MenuActionId::ViewLogs => {
            "Abre una vista independiente con logs exactos de eventos y errores."
                .to_owned()
        }
        MenuActionId::RecoverNetwork => {
            "Intenta restaurar la red y verificar que coincide con el snapshot original."
                .to_owned()
        }
        MenuActionId::BackToPrevious => {
            "Regresa a la vista anterior sin perder el contexto actual.".to_owned()
        }
        MenuActionId::BackHome => {
            "Vuelve al inicio limpio para elegir otra accion.".to_owned()
        }
        MenuActionId::Exit => "Cierra la sesion interactiva actual.".to_owned(),
        MenuActionId::Confirm => {
            "Ejecuta la accion sensible seleccionada.".to_owned()
        }
        MenuActionId::Cancel => {
            "Detiene esta accion sensible sin aplicar cambios.".to_owned()
        }
    }
}

pub fn confirmation_heading(action: ConfirmationAction) -> &'static str {
    match action {
        ConfirmationAction::EnableProtection => "Activar Sentinel",
        ConfirmationAction::DisableProtection => "Desactivar Sentinel",
        ConfirmationAction::RecoverNetwork => "Recuperar red",
    }
}
