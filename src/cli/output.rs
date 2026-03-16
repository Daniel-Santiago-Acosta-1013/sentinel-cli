use crate::storage::{install::InstallationState, state::RuntimeState};

pub fn render_summary_table(
    runtime: &RuntimeState,
    next_action: &str,
    terminal_width: usize,
    compact: bool,
) -> String {
    let mut rows = vec![
        ("Proteccion", runtime.mode.label().to_owned()),
        ("Riesgo", runtime.risk_level.label().to_owned()),
        (
            "Snapshot activo",
            runtime.snapshot_id.as_deref().unwrap_or("-").to_owned(),
        ),
        ("Siguiente accion", next_action.to_owned()),
    ];
    if !compact {
        rows.push((
            "Dominios bloqueados",
            runtime.blocklist_domain_count.to_string(),
        ));
        rows.push((
            "Version del bloqueador",
            runtime.blocklist_version.clone(),
        ));
    }
    render_table("Campo", "Valor", &rows, terminal_width)
}

pub fn render_status_table(
    runtime: &RuntimeState,
    install: &InstallationState,
    terminal_width: usize,
    compact: bool,
) -> String {
    let mut rows = vec![
        ("Proteccion", runtime.mode.label().to_owned()),
        ("Riesgo", runtime.risk_level.label().to_owned()),
        (
            "Runtime local",
            runtime
                .runtime_pid
                .map(|pid| pid.to_string())
                .unwrap_or_else(|| "No activo".to_owned()),
        ),
        (
            "Snapshot activo",
            runtime.snapshot_id.as_deref().unwrap_or("-").to_owned(),
        ),
        (
            "Ruta instalada",
            install
                .path_entry
                .as_deref()
                .unwrap_or("No disponible")
                .to_owned(),
        ),
        ("Resumen", runtime.status_summary.clone()),
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
        rows.push(("Accion sugerida", install.action.label().to_owned()));
    }
    render_table("Estado", "Valor", &rows, terminal_width)
}

pub fn render_safety_table(
    runtime: &RuntimeState,
    terminal_width: usize,
    compact: bool,
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
        rows.push(("Estado", "Sin ejecutar".to_owned()));
        rows.push((
            "Accion sugerida",
            "Ejecuta chequeos antes de cambiar la red".to_owned(),
        ));
    }
    render_table("Chequeo", "Resultado", &rows, terminal_width)
}

pub fn render_recovery_table(
    runtime: &RuntimeState,
    terminal_width: usize,
    compact: bool,
) -> String {
    let mut rows = vec![
        ("Modo actual", runtime.mode.label().to_owned()),
        (
            "Snapshot",
            runtime.snapshot_id.as_deref().unwrap_or("-").to_owned(),
        ),
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
    render_table("Recuperacion", "Valor", &rows, terminal_width)
}

pub fn render_install_table(
    install: &InstallationState,
    terminal_width: usize,
) -> String {
    let rows = vec![
        (
            "Instalado",
            if install.installed { "Si" } else { "No" }.to_owned(),
        ),
        ("Version objetivo", install.target_version.clone()),
        (
            "Version instalada",
            install
                .installed_version
                .as_deref()
                .unwrap_or("No detectada")
                .to_owned(),
        ),
        ("Accion sugerida", install.action.label().to_owned()),
        ("Resultado previo", install.last_install_result.clone()),
    ];
    render_table("Instalacion", "Valor", &rows, terminal_width)
}

fn render_table(
    left_header: &str,
    right_header: &str,
    rows: &[(&str, String)],
    terminal_width: usize,
) -> String {
    let width = terminal_width.saturating_sub(4).clamp(48, 92);
    let inner_width = width.saturating_sub(4);
    let max_label_width = rows
        .iter()
        .map(|(label, _)| label.chars().count())
        .chain(std::iter::once(left_header.chars().count()))
        .max()
        .unwrap_or(12);
    let left_width = max_label_width.clamp(12, inner_width.saturating_sub(16).min(22));
    let right_width = inner_width.saturating_sub(left_width + 3);

    let mut lines = Vec::new();
    lines.push(border('┌', '┬', '┐', left_width, right_width));
    lines.push(row(left_header, right_header, left_width, right_width));
    lines.push(border('├', '┼', '┤', left_width, right_width));
    for (label, value) in rows {
        lines.push(row(label, value, left_width, right_width));
    }
    lines.push(border('└', '┴', '┘', left_width, right_width));
    lines.join("\n")
}

fn row(left: &str, right: &str, left_width: usize, right_width: usize) -> String {
    format!(
        "│ {:left$} │ {:right$} │",
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
    left_width: usize,
    right_width: usize,
) -> String {
    format!(
        "{left}{}{mid}{}{right}",
        "─".repeat(left_width + 2),
        "─".repeat(right_width + 2),
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
        return "…".to_owned();
    }

    let mut truncated = value.chars().take(width - 1).collect::<String>();
    truncated.push('…');
    truncated
}

fn join_or_dash(items: &[String]) -> String {
    if items.is_empty() {
        "-".to_owned()
    } else {
        items.join(", ")
    }
}
