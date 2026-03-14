use comfy_table::{Cell, ContentArrangement, Table, presets::UTF8_BORDERS_ONLY};
use miette::{IntoDiagnostic, Result};
use serde_json::json;

use crate::{
    cli::{GlobalOptions, styles},
    core::{events::OperationEvent, rules::RuleEntry, state::ProtectionState},
    storage::config::AppConfig,
};

pub fn render_status(
    state: &ProtectionState,
    config: &AppConfig,
    options: &GlobalOptions,
) -> Result<serde_json::Value> {
    let mut table = Table::new();
    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new(styles::header("Field", options.no_color)),
            Cell::new(styles::header("Value", options.no_color)),
        ]);
    table.add_row(vec![Cell::new("State"), Cell::new(state.mode.as_str())]);
    table.add_row(vec![
        Cell::new("Active rules"),
        Cell::new(state.active_rule_count.to_string()),
    ]);
    table.add_row(vec![
        Cell::new("Allow rules"),
        Cell::new(state.active_exclusion_count.to_string()),
    ]);
    table.add_row(vec![
        Cell::new("Runtime PID"),
        Cell::new(
            state
                .runtime_pid
                .map(|pid| pid.to_string())
                .unwrap_or_else(|| "-".to_owned()),
        ),
    ]);
    if options.verbose {
        table.add_row(vec![
            Cell::new("Snapshot"),
            Cell::new(state.snapshot_id.clone().unwrap_or_else(|| "-".to_owned())),
        ]);
        table.add_row(vec![
            Cell::new("Last transition"),
            Cell::new(state.last_transition_at.to_rfc3339()),
        ]);
    }

    Ok(json!({
        "rendered": format!(
            "{}\n{}\n{}",
            styles::accent("Sentinel Status", options.no_color),
            table,
            if config.allow_rules.is_empty() {
                "No active allow rules".to_owned()
            } else {
                format!("Active allow rules: {}", config.allow_rules.len())
            }
        ),
        "active_rule_count": state.active_rule_count,
        "active_exclusion_count": state.active_exclusion_count,
        "runtime_pid": state.runtime_pid,
    }))
}

pub fn render_rules(
    block_rules: &[RuleEntry],
    allow_rules: Vec<RuleEntry>,
    options: &GlobalOptions,
) -> Result<serde_json::Value> {
    let mut table = Table::new();
    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new(styles::header("Kind", options.no_color)),
            Cell::new(styles::header("Match", options.no_color)),
            Cell::new(styles::header("Value", options.no_color)),
            Cell::new(styles::header("Source", options.no_color)),
        ]);

    for rule in block_rules.iter().chain(allow_rules.iter()) {
        table.add_row(vec![
            Cell::new(rule.kind.as_str()),
            Cell::new(rule.match_type.as_str()),
            Cell::new(rule.value.clone()),
            Cell::new(rule.source.as_str()),
        ]);
    }

    Ok(json!({
        "rendered": format!(
            "{}\n{}",
            styles::accent("Sentinel Rules", options.no_color),
            table
        ),
        "block_rules": block_rules.len(),
        "allow_rules": allow_rules.len(),
    }))
}

pub fn render_events(
    events: &[OperationEvent],
    options: &GlobalOptions,
) -> Result<serde_json::Value> {
    let mut table = Table::new();
    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new(styles::header("Time", options.no_color)),
            Cell::new(styles::header("Type", options.no_color)),
            Cell::new(styles::header("Severity", options.no_color)),
            Cell::new(styles::header("Message", options.no_color)),
        ]);

    for event in events {
        table.add_row(vec![
            Cell::new(event.timestamp.to_rfc3339()),
            Cell::new(event.kind.as_str()),
            Cell::new(event.severity.as_str()),
            Cell::new(event.message.clone()),
        ]);
    }

    Ok(json!({
        "rendered": format!(
            "{}\n{}",
            styles::accent("Recent Events", options.no_color),
            table
        ),
        "items": serde_json::to_value(events).into_diagnostic()?,
    }))
}
