use crossterm::event::{KeyCode, KeyEvent};
use miette::{Result, miette};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Up,
    Down,
    Confirm,
    Back,
    Exit,
}

pub fn translate_key(key: KeyEvent) -> Option<InputEvent> {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => Some(InputEvent::Up),
        KeyCode::Down | KeyCode::Char('j') => Some(InputEvent::Down),
        KeyCode::Enter => Some(InputEvent::Confirm),
        KeyCode::Esc | KeyCode::Backspace => Some(InputEvent::Back),
        KeyCode::Char('q') => Some(InputEvent::Exit),
        KeyCode::Char('y') => Some(InputEvent::Confirm),
        KeyCode::Char('n') => Some(InputEvent::Back),
        _ => None,
    }
}

pub fn parse_script(script: &str) -> Result<Vec<InputEvent>> {
    script
        .split(',')
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(|token| match token {
            "up" => Ok(InputEvent::Up),
            "down" => Ok(InputEvent::Down),
            "enter" | "confirm" => Ok(InputEvent::Confirm),
            "back" | "esc" | "cancel" => Ok(InputEvent::Back),
            "quit" | "exit" | "q" => Ok(InputEvent::Exit),
            other => Err(miette!("unsupported scripted input token: {other}")),
        })
        .collect()
}
