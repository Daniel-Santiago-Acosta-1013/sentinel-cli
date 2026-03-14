use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::tui::{
    app_state::{ConfirmationAction, InteractiveSession, Screen},
    theme,
};

pub fn draw(frame: &mut Frame<'_>, session: &InteractiveSession) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Min(10),
            Constraint::Length(9),
        ])
        .split(area);

    let header = Paragraph::new(vec![
        Line::from(Span::styled("Sentinel", theme::title())),
        Line::from(Span::styled(
            "Interactive DNS protection with recovery-first safety.",
            theme::muted(),
        )),
    ])
    .block(Block::default().borders(Borders::ALL).style(theme::frame()));
    frame.render_widget(header, chunks[0]);

    let overview = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Protection: ", theme::muted()),
            Span::styled(session.runtime_state.mode.label(), highlight_risk(session)),
        ]),
        Line::from(vec![
            Span::styled("Risk: ", theme::muted()),
            Span::styled(session.risk_level.label(), highlight_risk(session)),
            Span::raw("  "),
            Span::styled("Blocklist: ", theme::muted()),
            Span::styled(
                format!("{} domains", session.runtime_state.blocklist_domain_count),
                theme::accent(),
            ),
        ]),
    ])
    .wrap(Wrap { trim: true })
    .block(Block::default().borders(Borders::ALL).style(theme::frame()));
    frame.render_widget(overview, chunks[1]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(chunks[2]);

    let narrative =
        Paragraph::new(screen_lines(session)).wrap(Wrap { trim: true }).block(
            Block::default()
                .title(Span::styled(screen_title(session), theme::accent()))
                .borders(Borders::ALL)
                .style(theme::frame()),
        );
    frame.render_widget(narrative, main_chunks[0]);

    let actions = session.actions();
    let items = actions
        .iter()
        .map(|item| ListItem::new(Line::from(item.label.clone())))
        .collect::<Vec<_>>();
    let list = List::new(items).highlight_style(theme::selected()).block(
        Block::default()
            .title(Span::styled("Core Actions", theme::accent()))
            .borders(Borders::ALL)
            .style(theme::frame()),
    );
    let mut list_state = ListState::default();
    list_state.select(Some(session.selected_action));
    frame.render_stateful_widget(list, main_chunks[1], &mut list_state);

    let footer = Paragraph::new(vec![
        Line::from(Span::styled(&session.last_message, theme::frame())),
        Line::from(Span::styled(
            "Use ↑/↓ to move, Enter to select, Esc to go back, q to exit.",
            theme::muted(),
        )),
    ])
    .wrap(Wrap { trim: true })
    .block(Block::default().borders(Borders::ALL).style(theme::frame()));
    frame.render_widget(footer, chunks[3]);

    if let Some(pending) = session.pending_confirmation {
        draw_confirmation(frame, pending);
    }
}

pub fn render_snapshot(session: &InteractiveSession) -> String {
    let actions = session
        .actions()
        .iter()
        .enumerate()
        .map(|(index, action)| {
            if index == session.selected_action {
                format!("> {}", action.label)
            } else {
                format!("  {}", action.label)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "Sentinel\nProtection: {}\nRisk: {}\nBlocklist domains: {}\nScreen: {}\nSummary: {}\nMessage: {}\nInstall action: {}\nActions:\n{}",
        session.runtime_state.mode.label(),
        session.risk_level.label(),
        session.runtime_state.blocklist_domain_count,
        screen_title(session),
        session.status_summary,
        session.last_message,
        session.install_state.action.label(),
        actions
    )
}

fn screen_title(session: &InteractiveSession) -> &'static str {
    match session.screen {
        Screen::Home => "Home",
        Screen::Safety => "Safety",
        Screen::Status => "Status",
        Screen::Install => "Install",
        Screen::Recovery => "Recovery",
        Screen::Confirm => "Confirm",
        Screen::Exit => "Exit",
    }
}

fn screen_lines(session: &InteractiveSession) -> Vec<Line<'static>> {
    match session.screen {
        Screen::Home => vec![
            Line::from(
                "Sentinel keeps the flow intentionally narrow: inspect safety, enable protection, review status, recover if needed, and leave.",
            ),
            Line::from(""),
            Line::from(format!("Current state: {}", session.runtime_state.mode.label())),
            Line::from(session.status_summary.clone()),
        ],
        Screen::Safety => {
            let check = session.runtime_state.last_safety_check.clone();
            let status =
                check.as_ref().map(|item| item.status.label()).unwrap_or("Not run");
            let issues = check
                .as_ref()
                .map(|item| {
                    if item.issues.is_empty() {
                        "No blocking issues found.".to_owned()
                    } else {
                        item.issues.join(" | ")
                    }
                })
                .unwrap_or_else(|| "Safety checks have not been run yet.".to_owned());
            vec![
                Line::from(format!("Safety status: {status}")),
                Line::from(""),
                Line::from(issues),
                Line::from(session.status_summary.clone()),
            ]
        }
        Screen::Status => vec![
            Line::from(format!(
                "Protection mode: {}",
                session.runtime_state.mode.label()
            )),
            Line::from(format!("Risk level: {}", session.risk_level.label())),
            Line::from(format!(
                "Runtime PID: {}",
                session
                    .runtime_state
                    .runtime_pid
                    .map(|pid| pid.to_string())
                    .unwrap_or_else(|| "Not running".to_owned())
            )),
            Line::from(format!(
                "Install path: {}",
                session
                    .install_state
                    .path_entry
                    .clone()
                    .unwrap_or_else(|| "Not installed in PATH".to_owned())
            )),
        ],
        Screen::Install => vec![
            Line::from(format!(
                "Installed: {}",
                if session.install_state.installed { "Yes" } else { "No" }
            )),
            Line::from(format!(
                "Current version: {}",
                session
                    .install_state
                    .installed_version
                    .clone()
                    .unwrap_or_else(|| "Unavailable".to_owned())
            )),
            Line::from(format!(
                "Target version: {}",
                session.install_state.target_version
            )),
            Line::from(format!(
                "Recommended installer action: {}",
                session.install_state.action.label()
            )),
            Line::from(session.install_state.last_install_result.clone()),
        ],
        Screen::Recovery => vec![
            Line::from(
                "Recovery uses the latest valid DNS snapshot captured before Sentinel changed network settings.",
            ),
            Line::from(""),
            Line::from(
                "If health degrades, recover before attempting another risky action.",
            ),
            Line::from(session.status_summary.clone()),
        ],
        Screen::Confirm => vec![
            Line::from("Confirm the selected sensitive action."),
            Line::from(""),
            Line::from("Press Enter or y to continue, Esc or n to cancel."),
        ],
        Screen::Exit => vec![Line::from("Sentinel exited cleanly.")],
    }
}

fn highlight_risk(session: &InteractiveSession) -> Style {
    match session.risk_level {
        crate::storage::state::RiskLevel::Normal => theme::success(),
        crate::storage::state::RiskLevel::Warning => theme::warning(),
        crate::storage::state::RiskLevel::Critical => theme::danger(),
    }
}

fn draw_confirmation(frame: &mut Frame<'_>, pending: ConfirmationAction) {
    let area = centered_rect(60, 26, frame.area());
    frame.render_widget(Clear, area);
    let label = match pending {
        ConfirmationAction::EnableProtection => "Enable protection",
        ConfirmationAction::DisableProtection => "Disable protection",
        ConfirmationAction::RecoverNetwork => "Recover network",
    };
    let modal = Paragraph::new(vec![
        Line::from(Span::styled(label, theme::title())),
        Line::from(""),
        Line::from("This action changes or restores live network state."),
        Line::from("Sentinel will stop immediately if safety guarantees are not met."),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter/y to continue or Esc/n to cancel.",
            theme::muted(),
        )),
    ])
    .wrap(Wrap { trim: true })
    .block(Block::default().borders(Borders::ALL).style(theme::frame()));
    frame.render_widget(modal, area);
}

fn centered_rect(horizontal: u16, vertical: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - vertical) / 2),
            Constraint::Percentage(vertical),
            Constraint::Percentage((100 - vertical) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - horizontal) / 2),
            Constraint::Percentage(horizontal),
            Constraint::Percentage((100 - horizontal) / 2),
        ])
        .split(popup_layout[1])[1]
}
