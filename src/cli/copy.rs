use crate::storage::state::ProtectionMode;

use super::navigation::{ConfirmationAction, MenuActionId, Route};

pub fn app_subtitle() -> &'static str {
    "CLI guiada para proteger tu DNS con un flujo limpio, claro y seguro."
}

pub fn route_title(route: Route) -> &'static str {
    match route {
        Route::Home => "Inicio",
        Route::Safety => "Chequeos de seguridad",
        Route::Status => "Estado actual",
        Route::Installation => "Estado de instalacion",
        Route::Recovery => "Recuperacion",
        Route::Confirm(_) => "Confirmacion",
        Route::Progress => "Procesando",
        Route::Result => "Resultado",
        Route::Exit => "Salida",
    }
}

pub fn intro_text(route: Route, mode: ProtectionMode) -> &'static str {
    match route {
        Route::Home if matches!(mode, ProtectionMode::Degraded | ProtectionMode::Recovering) => {
            "Sentinel detecto un estado sensible. Puedes priorizar la recuperacion o revisar el estado antes de hacer nuevos cambios."
        }
        Route::Home => {
            "Elige una accion principal. Cada seleccion abre una vista independiente y limpia."
        }
        Route::Safety => {
            "Revisa si el equipo esta listo para cambiar la proteccion sin comprometer conectividad."
        }
        Route::Status => {
            "Consulta el estado actual de proteccion, runtime, snapshot y siguientes pasos."
        }
        Route::Installation => {
            "Consulta el estado de instalacion actual. Las acciones de instalacion siguen pasando por el script oficial."
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

pub fn home_summary_hint(mode: ProtectionMode) -> String {
    match mode {
        ProtectionMode::Inactive => {
            "Puedes ejecutar chequeos o activar la proteccion desde este inicio."
                .to_owned()
        }
        ProtectionMode::Active => {
            "La proteccion esta activa. Puedes revisar el estado o desactivarla desde el inicio."
                .to_owned()
        }
        ProtectionMode::Degraded | ProtectionMode::Recovering => {
            "Prioriza revisar el estado o recuperar la red antes de otros cambios."
                .to_owned()
        }
    }
}

pub fn action_label(action: MenuActionId, mode: ProtectionMode) -> String {
    match action {
        MenuActionId::RunSafetyChecks => "Ejecutar chequeos de seguridad".to_owned(),
        MenuActionId::ToggleProtection => {
            if mode == ProtectionMode::Active {
                "Desactivar proteccion".to_owned()
            } else {
                "Activar proteccion".to_owned()
            }
        }
        MenuActionId::ViewStatus => "Ver estado actual".to_owned(),
        MenuActionId::ViewInstallState => "Ver estado de instalacion".to_owned(),
        MenuActionId::RecoverNetwork => "Recuperar red".to_owned(),
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
            "Restaura la configuracion original y detiene la proteccion actual."
                .to_owned()
        }
        MenuActionId::ToggleProtection => {
            "Activa la proteccion con snapshot recuperable y runtime DNS local."
                .to_owned()
        }
        MenuActionId::ViewStatus => {
            "Muestra un resumen independiente de proteccion, runtime, snapshot e instalacion."
                .to_owned()
        }
        MenuActionId::ViewInstallState => {
            "Consulta version instalada, ruta detectada y accion sugerida."
                .to_owned()
        }
        MenuActionId::RecoverNetwork => {
            "Intenta restaurar la red y verificar que coincide con el snapshot original."
                .to_owned()
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
        ConfirmationAction::EnableProtection => "Activar proteccion",
        ConfirmationAction::DisableProtection => "Desactivar proteccion",
        ConfirmationAction::RecoverNetwork => "Recuperar red",
    }
}
