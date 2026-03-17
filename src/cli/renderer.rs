use crate::cli::{menu_state::MenuSession, navigation::Route, styles, views};

pub fn render(session: &MenuSession, terminal_width: u16) -> String {
    let profile = styles::profile(session.transcript_mode);
    views::render(session, terminal_width as usize, profile)
}

pub fn render_snapshot(session: &MenuSession) -> String {
    render(session, 100)
}

pub fn render_progress_preview(
    session: &MenuSession,
    terminal_width: u16,
    label: &str,
) -> String {
    let mut preview = session.clone();
    preview.route = Route::Progress;
    preview.progress_label = Some(label.to_owned());
    preview.progress_step = preview.progress_step.wrapping_add(1);
    render(&preview, terminal_width)
}
