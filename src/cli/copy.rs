use crate::{
    cli::navigation::DomainEditorMode,
    install::version,
    storage::{events::BlockActivitySummary, state::ProtectionMode},
};

use super::navigation::{ConfirmationAction, LogScope, MenuActionId, Route};

pub fn app_subtitle() -> String {
    format!("Version {}", version::current_version())
}

pub fn route_title(route: Route) -> &'static str {
    match route {
        Route::Home => "Inicio",
        Route::Settings => "Ajustes",
        Route::BlockedDomains => "Dominios bloqueados",
        Route::BlockedDomainEditor(DomainEditorMode::Add) => "Agregar dominio bloqueado",
        Route::BlockedDomainEditor(DomainEditorMode::Edit) => "Editar dominio bloqueado",
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
            "Cada accion principal abre una vista independiente y mantiene el inicio limpio."
        }
        Route::Settings => {
            "Administra el catalogo activo de dominios bloqueados sin salir del flujo guiado."
        }
        Route::BlockedDomains => {
            "Revisa la lista vigente, mueve la seleccion actual y administra sus cambios desde esta vista."
        }
        Route::BlockedDomainEditor(DomainEditorMode::Add) => {
            "Escribe el dominio que quieres agregar al catalogo activo."
        }
        Route::BlockedDomainEditor(DomainEditorMode::Edit) => {
            "Edita el dominio seleccionado y confirma para reemplazarlo en la lista activa."
        }
        Route::Safety => {
            "Revisa si el equipo esta listo para cambiar la proteccion sin comprometer conectividad."
        }
        Route::Status => {
            "Consulta el estado actual de Sentinel y la actividad reciente de bloqueo en una vista mas enfocada."
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
        Route::BlockedDomainEditor(_) => {
            "Escribe el dominio, usa Backspace para borrar, Enter para guardar, Esc para cancelar y q para salir."
        }
        Route::Home | Route::Recovery | Route::Confirm(_) | Route::Result => {
            "Usa ↑/↓ para navegar, Enter para confirmar, Esc para volver y q para salir."
        }
        _ => "Usa ↑/↓ para elegir, Enter para continuar, Esc para volver y q para salir.",
    }
}

pub fn action_label(action: MenuActionId, mode: ProtectionMode) -> String {
    match action {
        MenuActionId::ToggleProtection => {
            if mode == ProtectionMode::Active {
                "Desactivar Sentinel".to_owned()
            } else {
                "Activar Sentinel".to_owned()
            }
        }
        MenuActionId::ViewStatus => "Ver estado de Sentinel".to_owned(),
        MenuActionId::OpenSettings => "Ajustes".to_owned(),
        MenuActionId::ViewBlockedDomains => "Dominios bloqueados".to_owned(),
        MenuActionId::AddBlockedDomain => "Agregar dominio".to_owned(),
        MenuActionId::EditBlockedDomain => "Editar dominio seleccionado".to_owned(),
        MenuActionId::DeleteBlockedDomain => "Eliminar dominio seleccionado".to_owned(),
        MenuActionId::SelectNextBlockedDomain => "Siguiente dominio".to_owned(),
        MenuActionId::SelectPreviousBlockedDomain => "Dominio anterior".to_owned(),
        MenuActionId::ViewLogs => "Logs".to_owned(),
        MenuActionId::RecoverNetwork => "Recuperar red".to_owned(),
        MenuActionId::BackToPrevious => "Volver a la vista anterior".to_owned(),
        MenuActionId::BackSettings => "Volver a Ajustes".to_owned(),
        MenuActionId::BackHome => "Volver al inicio".to_owned(),
        MenuActionId::Exit => "Salir de Sentinel".to_owned(),
        MenuActionId::Confirm => "Confirmar accion".to_owned(),
        MenuActionId::Cancel => "Cancelar y volver".to_owned(),
    }
}

pub fn action_description(action: MenuActionId, mode: ProtectionMode) -> String {
    match action {
        MenuActionId::ToggleProtection if mode == ProtectionMode::Active => {
            "Restaura la configuracion original y detiene Sentinel.".to_owned()
        }
        MenuActionId::ToggleProtection => {
            "Inicia Sentinel y activa el bloqueo DNS local con snapshot recuperable.".to_owned()
        }
        MenuActionId::ViewStatus => {
            "Muestra el estado actual de Sentinel, su runtime, el snapshot y la instalacion.".to_owned()
        }
        MenuActionId::OpenSettings => {
            "Abre opciones de administracion para el catalogo activo de dominios bloqueados."
                .to_owned()
        }
        MenuActionId::ViewBlockedDomains => {
            "Lista y administra los dominios que Sentinel bloquea actualmente.".to_owned()
        }
        MenuActionId::AddBlockedDomain => {
            "Escribe un nuevo dominio para incluirlo en el catalogo activo.".to_owned()
        }
        MenuActionId::EditBlockedDomain => {
            "Edita el dominio actualmente seleccionado en la lista vigente.".to_owned()
        }
        MenuActionId::DeleteBlockedDomain => {
            "Elimina el dominio seleccionado del catalogo activo.".to_owned()
        }
        MenuActionId::SelectNextBlockedDomain => {
            "Mueve la seleccion al siguiente dominio visible.".to_owned()
        }
        MenuActionId::SelectPreviousBlockedDomain => {
            "Mueve la seleccion al dominio anterior visible.".to_owned()
        }
        MenuActionId::ViewLogs => {
            "Abre una vista independiente con logs exactos de eventos y errores.".to_owned()
        }
        MenuActionId::RecoverNetwork => {
            "Intenta restaurar la red y verificar que coincide con el snapshot original."
                .to_owned()
        }
        MenuActionId::BackToPrevious => {
            "Regresa a la vista anterior sin perder el contexto actual.".to_owned()
        }
        MenuActionId::BackSettings => {
            "Vuelve a Ajustes sin perder el catalogo actual.".to_owned()
        }
        MenuActionId::BackHome => "Vuelve al inicio limpio para elegir otra accion.".to_owned(),
        MenuActionId::Exit => "Cierra la sesion interactiva actual.".to_owned(),
        MenuActionId::Confirm => "Ejecuta la accion sensible seleccionada.".to_owned(),
        MenuActionId::Cancel => "Detiene esta accion sensible sin aplicar cambios.".to_owned(),
    }
}

pub fn confirmation_heading(action: ConfirmationAction) -> &'static str {
    match action {
        ConfirmationAction::EnableProtection => "Activar Sentinel",
        ConfirmationAction::DisableProtection => "Desactivar Sentinel",
        ConfirmationAction::RecoverNetwork => "Recuperar red",
    }
}

pub fn blocked_domains_empty_state() -> &'static str {
    "No hay dominios bloqueados configurados todavia."
}

pub fn blocked_domain_deleted(domain: &str) -> String {
    format!("Se elimino `{domain}` del catalogo activo.")
}

pub fn blocked_domain_saved(domain: &str, edited: bool) -> String {
    if edited {
        format!("El dominio seleccionado ahora se guarda como `{domain}`.")
    } else {
        format!("Se agrego `{domain}` al catalogo activo.")
    }
}

pub fn blocked_domain_selection(domain: Option<&str>) -> String {
    match domain {
        Some(domain) => format!("Dominio seleccionado: {domain}"),
        None => "No hay un dominio seleccionable todavia.".to_owned(),
    }
}

pub fn blocked_domain_editor_hint(mode: DomainEditorMode, original: Option<&str>) -> String {
    match (mode, original) {
        (DomainEditorMode::Add, _) => {
            "Escribe un dominio nuevo y pulsa Enter para guardarlo.".to_owned()
        }
        (DomainEditorMode::Edit, Some(domain)) => {
            format!("Edita `{domain}` y pulsa Enter para reemplazarlo.")
        }
        (DomainEditorMode::Edit, None) => {
            "Edita el dominio actual y pulsa Enter para guardarlo.".to_owned()
        }
    }
}

pub fn block_activity_empty_value() -> &'static str {
    "Sin datos"
}

pub fn top_blocked_domains_label(summary: &BlockActivitySummary) -> String {
    if summary.top_blocked_domains.is_empty() {
        return block_activity_empty_value().to_owned();
    }

    summary
        .top_blocked_domains
        .iter()
        .take(3)
        .map(|(domain, count)| format!("{domain} ({count})"))
        .collect::<Vec<_>>()
        .join(", ")
}
