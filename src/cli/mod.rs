pub mod copy;
pub mod logo;
pub mod menu_state;
pub mod navigation;
pub mod output;
pub mod renderer;
pub mod spinner;
pub mod styles;
pub mod terminal;
pub mod views;

use miette::{Result, miette};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Up,
    Down,
    Confirm,
    Back,
    Exit,
}

pub fn parse_script(script: &str) -> Result<Vec<InputEvent>> {
    script
        .split(',')
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(|token| match token {
            "up" | "arriba" => Ok(InputEvent::Up),
            "down" | "abajo" => Ok(InputEvent::Down),
            "enter" | "confirm" | "confirmar" | "si" | "sí" | "s" => {
                Ok(InputEvent::Confirm)
            }
            "back" | "esc" | "cancel" | "cancelar" | "no" | "b" => Ok(InputEvent::Back),
            "quit" | "exit" | "q" | "salir" => Ok(InputEvent::Exit),
            other => Err(miette!("token de entrada no soportado: {other}")),
        })
        .collect()
}
