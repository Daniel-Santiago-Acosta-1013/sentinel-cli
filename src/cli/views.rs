use crate::cli::{
    copy,
    logo::SENTINEL_ASCII_LOGO,
    menu_state::MenuSession,
    navigation::{MenuAction, MenuActionId, Route},
    output,
    spinner,
    styles::{self, StyleProfile},
};

pub fn render(session: &MenuSession, terminal_width: usize, profile: StyleProfile) -> String {
    match session.route {
        Route::Home => render_home(session, terminal_width, profile),
        Route::Safety => render_detail(session, terminal_width, profile),
        Route::Status => render_detail(session, terminal_width, profile),
        Route::Installation => render_detail(session, terminal_width, profile),
        Route::Recovery => render_recovery(session, terminal_width, profile),
        Route::Confirm(_) => render_confirmation(session, profile),
        Route::Progress => render_progress(session, terminal_width, profile),
        Route::Result => render_result(session, terminal_width, profile),
        Route::Exit => render_exit(session, profile),
    }
}

fn render_home(
    session: &MenuSession,
    terminal_width: usize,
    profile: StyleProfile,
) -> String {
    let mut lines = vec![
        styles::title(SENTINEL_ASCII_LOGO, profile),
        styles::muted(copy::app_subtitle(), profile),
        String::new(),
        styles::section_title("Inicio", profile),
        copy::intro_text(session.route, session.runtime_state.mode).to_owned(),
        String::new(),
        styles::inline_badges(&[
            styles::status_badge(
                "Proteccion",
                session.runtime_state.mode.label(),
                styles::tone_for_mode(session.runtime_state.mode),
                profile,
            ),
            styles::status_badge(
                "Riesgo",
                session.runtime_state.risk_level.label(),
                styles::tone_for_risk(session.runtime_state.risk_level),
                profile,
            ),
        ]),
        String::new(),
        output::render_summary_table(
            &session.runtime_state,
            &copy::home_summary_hint(session.runtime_state.mode),
            terminal_width,
            terminal_width < 84,
            profile,
        ),
        String::new(),
        styles::muted(&session.last_message, profile),
        String::new(),
        styles::section_title("Acciones principales", profile),
    ];

    lines.extend(render_menu(&session.actions(), session.selected_index, profile, true));
    lines.push(String::new());
    lines.push(styles::muted(copy::footer_hint(session.route), profile));
    lines.join("\n")
}

fn render_detail(
    session: &MenuSession,
    terminal_width: usize,
    profile: StyleProfile,
) -> String {
    let compact = terminal_width < 84;
    let body = match session.route {
        Route::Safety => {
            output::render_safety_table(&session.runtime_state, terminal_width, compact, profile)
        }
        Route::Status => output::render_status_table(
            &session.runtime_state,
            &session.install_state,
            terminal_width,
            compact,
            profile,
        ),
        Route::Installation => {
            output::render_install_table(&session.install_state, terminal_width, profile)
        }
        _ => String::new(),
    };

    render_standard_view(
        session,
        profile,
        body,
        copy::intro_text(session.route, session.runtime_state.mode),
    )
}

fn render_recovery(
    session: &MenuSession,
    terminal_width: usize,
    profile: StyleProfile,
) -> String {
    let compact = terminal_width < 84;
    let table =
        output::render_recovery_table(&session.runtime_state, terminal_width, compact, profile);
    render_standard_view(
        session,
        profile,
        table,
        copy::intro_text(session.route, session.runtime_state.mode),
    )
}

fn render_confirmation(session: &MenuSession, profile: StyleProfile) -> String {
    let route_title = copy::route_title(session.route);
    let mut lines = vec![
        styles::section_title(route_title, profile),
        styles::warning(copy::intro_text(session.route, session.runtime_state.mode), profile),
        String::new(),
        styles::status_badge(
            "Accion sensible",
            copy::confirmation_heading(session.pending_confirmation()),
            styles::Tone::Warning,
            profile,
        ),
        String::new(),
        styles::muted(&session.last_message, profile),
        String::new(),
        styles::section_title("Elegir siguiente paso", profile),
    ];
    lines.extend(render_menu(&session.actions(), session.selected_index, profile, false));
    lines.push(String::new());
    lines.push(styles::muted(copy::footer_hint(session.route), profile));
    lines.join("\n")
}

fn render_progress(
    session: &MenuSession,
    terminal_width: usize,
    profile: StyleProfile,
) -> String {
    let width = terminal_width.min(88);
    let progress = session.progress_label().unwrap_or("Procesando...");
    let mut lines = vec![
        styles::section_title("Procesando", profile),
        styles::muted("Sentinel esta ejecutando la accion solicitada.", profile),
        String::new(),
        styles::emphasis(&spinner::line(progress, session.progress_step, profile), profile),
        String::new(),
        styles::muted(
            "La vista final aparecera al terminar la operacion.",
            profile,
        ),
    ];
    if width > 60 {
        lines.push(String::new());
        lines.push(styles::muted(
            &format!("Estado actual: {}", session.runtime_state.mode.label()),
            profile,
        ));
    }
    lines.join("\n")
}

fn render_result(
    session: &MenuSession,
    terminal_width: usize,
    profile: StyleProfile,
) -> String {
    let result = session
        .last_result
        .as_ref()
        .expect("result route requires last_result");
    let compact = terminal_width < 84;
    let summary = output::render_summary_table(
        &session.runtime_state,
        &result.next_step,
        terminal_width,
        compact,
        profile,
    );

    let mut lines = vec![
        styles::section_title(&result.title, profile),
        styles::tone_text(&result.summary, result.tone, profile),
        String::new(),
        styles::inline_badges(&[
            styles::status_badge(
                "Proteccion",
                session.runtime_state.mode.label(),
                styles::tone_for_mode(session.runtime_state.mode),
                profile,
            ),
            styles::status_badge(
                "Siguiente paso",
                &result.next_step,
                styles::tone_from_result(result.tone),
                profile,
            ),
        ]),
        String::new(),
        summary,
        String::new(),
        styles::section_title("Continuar", profile),
    ];
    lines.extend(render_menu(&session.actions(), session.selected_index, profile, false));
    lines.push(String::new());
    lines.push(styles::muted(copy::footer_hint(session.route), profile));
    lines.join("\n")
}

fn render_exit(session: &MenuSession, profile: StyleProfile) -> String {
    [
        styles::section_title("Salida", profile),
        styles::muted("Sentinel cerro la sesion interactiva.", profile),
        String::new(),
        session.last_message.clone(),
    ]
    .join("\n")
}

fn render_standard_view(
    session: &MenuSession,
    profile: StyleProfile,
    body: String,
    intro: &str,
) -> String {
    let mut lines = vec![
        styles::section_title(copy::route_title(session.route), profile),
        intro.to_owned(),
        String::new(),
        body,
        String::new(),
        styles::muted(&session.last_message, profile),
        String::new(),
        styles::section_title("Continuar", profile),
    ];
    lines.extend(render_menu(&session.actions(), session.selected_index, profile, false));
    lines.push(String::new());
    lines.push(styles::muted(copy::footer_hint(session.route), profile));
    lines.join("\n")
}

fn render_menu(
    actions: &[MenuAction],
    selected_index: usize,
    profile: StyleProfile,
    show_description: bool,
) -> Vec<String> {
    let mut lines = Vec::new();
    for (index, action) in actions.iter().enumerate() {
        let selected = index == selected_index;
        lines.push(styles::menu_line(&action.label, selected, profile));
        if show_description || (selected && !action.description.is_empty() && action.id != MenuActionId::Exit)
        {
            lines.push(styles::menu_description(&action.description, selected, profile));
        }
    }
    lines
}
