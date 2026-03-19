use std::io::{Stdout, Write, stdout};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use miette::IntoDiagnostic;

use crate::{app::AppResult, cli::InputEvent};

#[derive(Debug, Clone, Copy)]
pub struct TerminalCapabilities {
    pub color: bool,
    pub unicode: bool,
}

pub struct TerminalSession {
    stdout: Stdout,
}

impl TerminalSession {
    pub fn start() -> AppResult<Self> {
        terminal::enable_raw_mode().into_diagnostic()?;
        let mut stdout = stdout();
        execute!(stdout, cursor::Hide).into_diagnostic()?;
        Ok(Self { stdout })
    }

    pub fn width(&self) -> u16 {
        terminal::size().map(|(width, _)| width).unwrap_or(100)
    }

    pub fn capabilities(&self) -> TerminalCapabilities {
        TerminalCapabilities {
            color: std::env::var_os("NO_COLOR").is_none()
                && std::env::var("CLICOLOR").ok().as_deref() != Some("0"),
            unicode: std::env::var("TERM").ok().as_deref() != Some("dumb")
                && std::env::var("SENTINEL_ASCII_ONLY").ok().as_deref() != Some("1"),
        }
    }

    pub fn draw(&mut self, content: &str) -> AppResult<()> {
        self.clear_screen()?;
        write!(self.stdout, "{}", normalize_for_raw_terminal(content))
            .into_diagnostic()?;
        self.stdout.flush().into_diagnostic()?;
        Ok(())
    }

    pub fn redraw(&mut self, content: &str) -> AppResult<()> {
        execute!(
            self.stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::FromCursorDown)
        )
        .into_diagnostic()?;
        write!(self.stdout, "{}", normalize_for_raw_terminal(content))
            .into_diagnostic()?;
        self.stdout.flush().into_diagnostic()?;
        Ok(())
    }

    pub fn clear_screen(&mut self) -> AppResult<()> {
        execute!(self.stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))
            .into_diagnostic()?;
        Ok(())
    }

    pub fn read_input(&mut self) -> AppResult<InputEvent> {
        loop {
            let event = event::read().into_diagnostic()?;
            let Event::Key(key_event) = event else {
                continue;
            };
            if !matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                continue;
            }

            let input = match key_event.code {
                KeyCode::Up => Some(InputEvent::Up),
                KeyCode::Down => Some(InputEvent::Down),
                KeyCode::Enter => Some(InputEvent::Confirm),
                KeyCode::Backspace => Some(InputEvent::Backspace),
                KeyCode::Esc => Some(InputEvent::Back),
                KeyCode::Char(ch) if ch.eq_ignore_ascii_case(&'q') => {
                    Some(InputEvent::Exit)
                }
                KeyCode::Char(ch)
                    if !key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    Some(InputEvent::InsertChar(ch))
                }
                _ => None,
            };

            if let Some(input) = input {
                return Ok(input);
            }
        }
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = execute!(self.stdout, cursor::Show);
        let _ = terminal::disable_raw_mode();
    }
}

fn normalize_for_raw_terminal(content: &str) -> String {
    let mut normalized = content.replace("\r\n", "\n");
    normalized = normalized.replace('\n', "\r\n");
    if !normalized.ends_with("\r\n") {
        normalized.push_str("\r\n");
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::{TerminalCapabilities, normalize_for_raw_terminal};

    #[test]
    fn normalizes_newlines_for_raw_terminal_output() {
        assert_eq!(normalize_for_raw_terminal("uno\ndos\n"), "uno\r\ndos\r\n");
        assert_eq!(normalize_for_raw_terminal("uno"), "uno\r\n");
        assert_eq!(normalize_for_raw_terminal("uno\r\ndos"), "uno\r\ndos\r\n");
    }

    #[test]
    fn terminal_capabilities_struct_is_copyable() {
        let capabilities = TerminalCapabilities { color: true, unicode: true };
        assert!(capabilities.color);
        assert!(capabilities.unicode);
    }
}
