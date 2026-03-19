use crate::storage::{
    events::{BlockActivitySummary, EventRecord},
    install::InstallationState,
    state::RuntimeState,
};

use super::{copy, styles, styles::StyleProfile};

pub fn render_summary_table(
    runtime: &RuntimeState,
    next_action: &str,
    terminal_width: usize,
    compact: bool,
    profile: StyleProfile,
) -> String {
    let mut rows = vec![
        ("Proteccion", runtime.mode.label().to_owned()),
        ("Riesgo", runtime.risk_level.label().to_owned()),
        ("Snapshot activo", runtime.snapshot_id.as_deref().unwrap_or("-").to_owned()),
        ("Siguiente accion", next_action.to_owned()),
    ];
    if !compact {
        rows.push(("Dominios bloqueados", runtime.blocklist_domain_count.to_string()));
        rows.push(("Version del bloqueador", runtime.blocklist_version.clone()));
    }
    render_table("Campo", "Valor", &rows, terminal_width, profile)
}

pub fn render_status_table(
    runtime: &RuntimeState,
    install: &InstallationState,
    activity: &BlockActivitySummary,
    terminal_width: usize,
    compact: bool,
    profile: StyleProfile,
) -> String {
    let mut rows = vec![
        ("Proteccion", runtime.mode.label().to_owned()),
        (
            "Runtime local",
            runtime
                .runtime_pid
                .map(|pid| pid.to_string())
                .unwrap_or_else(|| "No activo".to_owned()),
        ),
        ("Snapshot activo", runtime.snapshot_id.as_deref().unwrap_or("-").to_owned()),
        (
            "Ruta instalada",
            install.path_entry.as_deref().unwrap_or("No disponible").to_owned(),
        ),
    ];
    if let Some(verification) = runtime.last_verification_result.as_ref() {
        rows.push(("Verificacion", verification.summary.clone()));
    }
    if !compact {
        rows.push((
            "Version instalada",
            install
                .installed_version
                .as_deref()
                .unwrap_or("No detectada")
                .to_owned(),
        ));
    }

    [
        render_table("Estado", "Valor", &rows, terminal_width, profile),
        String::new(),
        render_block_activity_table(activity, terminal_width, profile),
    ]
    .join("\n")
}

pub fn render_safety_table(
    runtime: &RuntimeState,
    terminal_width: usize,
    compact: bool,
    profile: StyleProfile,
) -> String {
    let mut rows = Vec::new();
    if let Some(summary) = runtime.last_safety_check.as_ref() {
        rows.push(("Estado", summary.status.label().to_owned()));
        rows.push((
            "Conectividad lista",
            if summary.connectivity_ready { "Si" } else { "No" }.to_owned(),
        ));
        rows.push((
            "Recuperacion lista",
            if summary.recovery_ready { "Si" } else { "No" }.to_owned(),
        ));
        rows.push((
            "DNS personalizados",
            if summary.detected_custom_dns { "Si" } else { "No" }.to_owned(),
        ));
        if !compact {
            rows.push(("Accion sugerida", summary.recommended_action.clone()));
        }
    } else {
        rows.push(("Estado", "Pendiente".to_owned()));
        rows.push((
            "Accion sugerida",
            "Activa Sentinel; la validacion de seguridad se ejecuta automaticamente."
                .to_owned(),
        ));
    }
    render_table("Chequeo", "Resultado", &rows, terminal_width, profile)
}

pub fn render_recovery_table(
    runtime: &RuntimeState,
    terminal_width: usize,
    compact: bool,
    profile: StyleProfile,
) -> String {
    let mut rows = vec![
        ("Modo actual", runtime.mode.label().to_owned()),
        ("Snapshot", runtime.snapshot_id.as_deref().unwrap_or("-").to_owned()),
    ];
    if let Some(verification) = runtime.last_verification_result.as_ref() {
        rows.push((
            "Coincide con snapshot",
            if verification.matches_snapshot { "Si" } else { "No" }.to_owned(),
        ));
        if !compact {
            rows.push((
                "Servicios con diferencia",
                join_or_dash(&verification.mismatched_services),
            ));
        }
        rows.push(("Resumen", verification.summary.clone()));
    } else {
        rows.push(("Resumen", runtime.status_summary.clone()));
    }
    render_table("Recuperacion", "Valor", &rows, terminal_width, profile)
}

pub fn render_settings_summary(
    blocked_domain_count: usize,
    terminal_width: usize,
    profile: StyleProfile,
) -> String {
    let rows = vec![
        ("Seccion", "Administracion del bloqueo".to_owned()),
        ("Dominios vigentes", blocked_domain_count.to_string()),
        (
            "Siguiente paso",
            "Entra a Dominios bloqueados para revisar o modificar la lista actual."
                .to_owned(),
        ),
    ];
    render_table("Campo", "Valor", &rows, terminal_width, profile)
}

pub fn render_blocked_domains_table(
    domains: &[String],
    selected_index: usize,
    terminal_width: usize,
    profile: StyleProfile,
) -> String {
    if domains.is_empty() {
        let rows = vec![
            ("Estado", copy::blocked_domains_empty_state().to_owned()),
            ("Seleccion", copy::blocked_domain_selection(None)),
        ];
        return render_table("Campo", "Valor", &rows, terminal_width, profile);
    }

    let window_size = 8usize;
    let selected_index = selected_index.min(domains.len().saturating_sub(1));
    let start = selected_index.saturating_sub(window_size.saturating_sub(1));
    let end = (start + window_size).min(domains.len());
    let mut rows = domains
        .iter()
        .enumerate()
        .skip(start)
        .take(end.saturating_sub(start))
        .map(|(index, domain)| {
            (
                if index == selected_index { "›" } else { " " },
                domain.clone(),
            )
        })
        .collect::<Vec<_>>();
    rows.push((
        "Seleccion",
        copy::blocked_domain_selection(domains.get(selected_index).map(String::as_str)),
    ));
    let selected_domain = domains.get(selected_index).cloned();
    colorize_selected_domain(
        render_table("Sel", "Dominio", &rows, terminal_width, profile),
        selected_domain.as_deref(),
        profile,
    )
}

pub fn render_domain_editor(
    current_value: &str,
    hint: String,
    terminal_width: usize,
    profile: StyleProfile,
) -> String {
    let rows = vec![
        (
            "Valor actual",
            if current_value.is_empty() {
                "-".to_owned()
            } else {
                current_value.to_owned()
            },
        ),
        ("Ayuda", hint),
    ];
    render_table("Campo", "Valor", &rows, terminal_width, profile)
}

pub fn render_block_activity_table(
    activity: &BlockActivitySummary,
    terminal_width: usize,
    profile: StyleProfile,
) -> String {
    let rows = vec![
        (
            "Bloqueos desde la activacion",
            activity.blocked_since_activation.to_string(),
        ),
        (
            "Dominios unicos bloqueados",
            activity.unique_blocked_domains.to_string(),
        ),
        (
            "Ultimo bloqueo",
            activity
                .last_blocked_at
                .map(|value| value.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                .unwrap_or_else(|| copy::block_activity_empty_value().to_owned()),
        ),
        (
            "Top dominios bloqueados",
            copy::top_blocked_domains_label(activity),
        ),
    ];
    render_table("Actividad de bloqueo", "Valor", &rows, terminal_width, profile)
}

pub fn render_log_panel_stream(
    events: &[EventRecord],
    empty_copy: &str,
    terminal_width: usize,
    profile: StyleProfile,
) -> String {
    let width = terminal_width.saturating_sub(10).clamp(40, 84);
    let mut body_lines = Vec::new();
    let display_events = filter_display_events(events);
    if display_events.is_empty() {
        body_lines.push(empty_copy.to_owned());
    } else {
        for (index, event) in display_events.iter().take(8).enumerate() {
            if index > 0 {
                body_lines.push(String::new());
            }
            body_lines.extend(render_event_entry(event, width));
        }
    }
    render_log_panel(&body_lines, width, profile)
}

fn render_table(
    left_header: &str,
    right_header: &str,
    rows: &[(&str, String)],
    terminal_width: usize,
    profile: StyleProfile,
) -> String {
    let width = terminal_width.saturating_sub(4).clamp(48, 92);
    let inner_width = width.saturating_sub(4);
    let max_label_width = rows
        .iter()
        .map(|(label, _)| label.chars().count())
        .chain(std::iter::once(left_header.chars().count()))
        .max()
        .unwrap_or(12);
    let left_width = max_label_width.clamp(12, inner_width.saturating_sub(16).min(26));
    let right_width = inner_width.saturating_sub(left_width + 3);

    let (
        top_left,
        top_mid,
        top_right,
        mid_left,
        mid_mid,
        mid_right,
        bottom_left,
        bottom_mid,
        bottom_right,
        horiz,
        vert,
    ) = if profile.unicode {
        ('┌', '┬', '┐', '├', '┼', '┤', '└', '┴', '┘', '─', '│')
    } else {
        ('+', '+', '+', '+', '+', '+', '+', '+', '+', '-', '|')
    };

    let mut lines = Vec::new();
    lines.push(border(top_left, top_mid, top_right, horiz, left_width, right_width));
    lines.push(row(left_header, right_header, left_width, right_width, vert));
    lines.push(border(mid_left, mid_mid, mid_right, horiz, left_width, right_width));
    for (label, value) in rows {
        lines.push(row(label, value, left_width, right_width, vert));
    }
    lines.push(border(
        bottom_left,
        bottom_mid,
        bottom_right,
        horiz,
        left_width,
        right_width,
    ));
    lines.join("\n")
}

fn row(
    left: &str,
    right: &str,
    left_width: usize,
    right_width: usize,
    vert: char,
) -> String {
    format!(
        "{vert} {:left$} {vert} {:right$} {vert}",
        truncate(left, left_width),
        truncate(right, right_width),
        left = left_width,
        right = right_width
    )
}

fn border(
    left_edge: char,
    middle_edge: char,
    right_edge: char,
    horiz: char,
    left_width: usize,
    right_width: usize,
) -> String {
    format!(
        "{left}{}{mid}{}{right}",
        horiz.to_string().repeat(left_width + 2),
        horiz.to_string().repeat(right_width + 2),
        left = left_edge,
        mid = middle_edge,
        right = right_edge
    )
}

fn truncate(value: &str, width: usize) -> String {
    let count = value.chars().count();
    if count <= width {
        return value.to_owned();
    }
    if width <= 1 {
        return "...".to_owned();
    }

    let mut truncated = value.chars().take(width - 1).collect::<String>();
    truncated.push(if width > 2 { '…' } else { '.' });
    truncated
}

fn join_or_dash(items: &[String]) -> String {
    if items.is_empty() { "-".to_owned() } else { items.join(", ") }
}

fn colorize_selected_domain(
    table: String,
    selected_domain: Option<&str>,
    profile: StyleProfile,
) -> String {
    if !profile.color {
        return table;
    }

    let Some(domain) = selected_domain else {
        return table;
    };

    table.replace(domain, &styles::accent_blue(domain, profile))
}

fn render_event_entry(event: &EventRecord, width: usize) -> Vec<String> {
    let mut lines = vec![format!(
        "[{}] {:<11} {}",
        event.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
        event.severity.label().to_uppercase(),
        event.kind.label()
    )];
    for line in wrap_text(&sanitize_event_message(event), width) {
        lines.push(format!("  {line}"));
    }
    lines
}

fn wrap_text(value: &str, width: usize) -> Vec<String> {
    if value.is_empty() || width < 12 {
        return vec![value.to_owned()];
    }

    let mut lines = Vec::new();
    let mut current = String::new();
    for word in value.split_whitespace() {
        let candidate_len = current.chars().count()
            + if current.is_empty() { 0 } else { 1 }
            + word.chars().count();
        if candidate_len > width && !current.is_empty() {
            lines.push(current);
            current = word.to_owned();
        } else if current.is_empty() {
            current = word.to_owned();
        } else {
            current.push(' ');
            current.push_str(word);
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn render_log_panel(lines: &[String], width: usize, profile: StyleProfile) -> String {
    let (top_left, top_right, bottom_left, bottom_right, horiz, vert) = if profile.unicode
    {
        ('┌', '┐', '└', '┘', '─', '│')
    } else {
        ('+', '+', '+', '+', '-', '|')
    };

    let mut rendered = Vec::new();
    rendered
        .push(format!("{top_left}{}{top_right}", horiz.to_string().repeat(width + 2)));
    for line in lines {
        if line.is_empty() {
            rendered.push(format!("{vert} {:width$} {vert}", "", width = width));
            continue;
        }
        for wrapped in wrap_text(line, width) {
            rendered.push(format!("{vert} {:width$} {vert}", wrapped, width = width));
        }
    }
    rendered.push(format!(
        "{bottom_left}{}{bottom_right}",
        horiz.to_string().repeat(width + 2)
    ));
    rendered.join("\n")
}

fn filter_display_events(events: &[EventRecord]) -> Vec<EventRecord> {
    events
        .iter()
        .filter_map(|event| {
            let message = sanitize_event_message(event);
            if message.is_empty() {
                None
            } else {
                let mut cloned = event.clone();
                cloned.message = message;
                Some(cloned)
            }
        })
        .collect()
}

fn sanitize_event_message(event: &EventRecord) -> String {
    let trimmed = event.message.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let exact = trimmed.split(" | Siguiente paso:").next().unwrap_or(trimmed).trim();

    if matches!(event.kind, crate::storage::events::EventKind::SafetyCheck)
        && is_generic_safety_message(exact)
    {
        return String::new();
    }

    exact.to_owned()
}

fn is_generic_safety_message(message: &str) -> bool {
    matches!(
        message,
        "Los chequeos fallaron. Corrige el problema o recupera la red antes de cambiarla."
            | "Los chequeos aprobaron. Puedes activar la proteccion de forma segura."
            | "Los chequeos aprobaron con precauciones. Revisa la nota antes de confirmar."
    )
}
