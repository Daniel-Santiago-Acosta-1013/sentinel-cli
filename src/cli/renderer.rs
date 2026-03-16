use crate::cli::{
    copy,
    menu_state::{ActionId, MenuSession, ViewId},
    output, styles,
};

pub fn render(session: &MenuSession, terminal_width: u16) -> String {
    let terminal_width = terminal_width as usize;
    let compact = terminal_width < 84;
    let mut lines = Vec::new();
    lines.push(styles::title(copy::app_title()));
    lines.push(copy::app_subtitle().to_owned());
    lines.push(String::new());
    lines.push(format!("Pantalla: {}", copy::view_title(session.view)));
    lines.push(format!("Proteccion: {}", session.runtime_state.mode.label()));
    lines.push(format!("Riesgo: {}", session.runtime_state.risk_level.label()));
    lines.push(format!("Siguiente accion: {}", next_action_label(session)));
    lines.push(String::new());
    lines.push(copy::intro_text(session.view, session.runtime_state.mode).to_owned());
    lines.push(String::new());
    lines.push(match session.view {
        ViewId::Inicio | ViewId::Confirmacion | ViewId::Salida => {
            output::render_summary_table(
                &session.runtime_state,
                &next_action_label(session),
                terminal_width,
                compact,
            )
        }
        ViewId::Seguridad => {
            output::render_safety_table(&session.runtime_state, terminal_width, compact)
        }
        ViewId::Estado => output::render_status_table(
            &session.runtime_state,
            &session.install_state,
            terminal_width,
            compact,
        ),
        ViewId::Instalacion => {
            output::render_install_table(&session.install_state, terminal_width)
        }
        ViewId::Recuperacion => {
            output::render_recovery_table(&session.runtime_state, terminal_width, compact)
        }
    });
    lines.push(String::new());
    if let Some(summary) = view_summary(session) {
        lines.push(format!("Resumen clave: {summary}"));
        lines.push(String::new());
    }

    if let Some(pending) = session.pending_confirmation {
        lines.push(format!(
            "Confirmacion pendiente: {}",
            copy::confirmation_label(pending)
        ));
        lines.push(
            "Presiona Enter para continuar, Esc para cancelar o q para salir."
                .to_owned(),
        );
        lines.push(String::new());
    }

    lines.push("Acciones:".to_owned());
    for (index, action) in session.actions().iter().enumerate() {
        let prefix = if index == session.selected_index { ">" } else { " " };
        lines.push(format!("{prefix} {}", action.label));
    }

    lines.push(String::new());
    lines.push(format!("Mensaje: {}", session.last_message));
    lines.push(copy::footer_hint().to_owned());
    lines.join("\n")
}

pub fn render_snapshot(session: &MenuSession) -> String {
    render(session, 100)
}

fn next_action_label(session: &MenuSession) -> String {
    if let Some(pending) = session.pending_confirmation {
        return copy::confirmation_label(pending).to_owned();
    }
    match session.view {
        ViewId::Confirmacion => "Confirmar accion sensible".to_owned(),
        _ => {
            let id = session.selected_action_id();
            if id == ActionId::ViewInstallState {
                format!(
                    "{} ({})",
                    copy::action_label(id, session.runtime_state.mode),
                    copy::install_action_label(session.install_state.action)
                )
            } else {
                copy::action_label(id, session.runtime_state.mode)
            }
        }
    }
}

fn view_summary(session: &MenuSession) -> Option<String> {
    match session.view {
        ViewId::Seguridad => session
            .runtime_state
            .last_safety_check
            .as_ref()
            .map(|summary| summary.recommended_action.clone()),
        ViewId::Estado => Some(session.runtime_state.status_summary.clone()),
        ViewId::Recuperacion => session
            .runtime_state
            .last_verification_result
            .as_ref()
            .map(|verification| verification.summary.clone())
            .or_else(|| Some(session.runtime_state.status_summary.clone())),
        _ => None,
    }
}
