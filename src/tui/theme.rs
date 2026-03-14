use ratatui::style::{Color, Modifier, Style};

pub fn frame() -> Style {
    Style::default().fg(Color::Rgb(219, 228, 240))
}

pub fn title() -> Style {
    Style::default().fg(Color::Rgb(244, 213, 141)).add_modifier(Modifier::BOLD)
}

pub fn accent() -> Style {
    Style::default().fg(Color::Rgb(116, 172, 255)).add_modifier(Modifier::BOLD)
}

pub fn success() -> Style {
    Style::default().fg(Color::Rgb(141, 213, 133))
}

pub fn warning() -> Style {
    Style::default().fg(Color::Rgb(244, 198, 118))
}

pub fn danger() -> Style {
    Style::default().fg(Color::Rgb(244, 123, 123))
}

pub fn muted() -> Style {
    Style::default().fg(Color::Rgb(150, 160, 176))
}

pub fn selected() -> Style {
    Style::default()
        .fg(Color::Rgb(18, 24, 38))
        .bg(Color::Rgb(244, 213, 141))
        .add_modifier(Modifier::BOLD)
}
