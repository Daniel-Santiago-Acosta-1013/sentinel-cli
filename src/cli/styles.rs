use crate::storage::state::ProtectionMode;

use super::navigation::ResultTone;

#[derive(Debug, Clone, Copy)]
pub struct StyleProfile {
    pub color: bool,
    pub unicode: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Tone {
    Info,
    Success,
    Warning,
    Error,
}

pub fn profile(transcript_mode: bool) -> StyleProfile {
    let no_color = transcript_mode
        || std::env::var_os("NO_COLOR").is_some()
        || std::env::var("CLICOLOR").ok().as_deref() == Some("0");
    let unicode = std::env::var("TERM").ok().as_deref() != Some("dumb")
        && std::env::var("SENTINEL_ASCII_ONLY").ok().as_deref() != Some("1");
    StyleProfile { color: !no_color, unicode }
}

pub fn title(label: &str, profile: StyleProfile) -> String {
    colorize(label, "1;36", profile)
}

pub fn section_title(label: &str, profile: StyleProfile) -> String {
    let prefix = if profile.unicode { "◆" } else { "*" };
    colorize(&format!("{prefix} {label}"), "1;37", profile)
}

pub fn emphasis(label: &str, profile: StyleProfile) -> String {
    colorize(label, "1;36", profile)
}

pub fn accent_blue(label: &str, profile: StyleProfile) -> String {
    colorize(label, "1;34", profile)
}

pub fn muted(label: &str, profile: StyleProfile) -> String {
    colorize(label, "2;37", profile)
}

pub fn warning(label: &str, profile: StyleProfile) -> String {
    colorize(label, "1;33", profile)
}

pub fn tone_text(label: &str, tone: ResultTone, profile: StyleProfile) -> String {
    let code = match tone {
        ResultTone::Info => "1;36",
        ResultTone::Success => "1;32",
        ResultTone::Warning => "1;33",
        ResultTone::Error => "1;31",
    };
    colorize(label, code, profile)
}

pub fn inline_badges(parts: &[String]) -> String {
    parts.join("  ")
}

pub fn status_badge(
    label: &str,
    value: &str,
    tone: Tone,
    profile: StyleProfile,
) -> String {
    let icon = match tone {
        Tone::Info => {
            if profile.unicode {
                "•"
            } else {
                "-"
            }
        }
        Tone::Success => {
            if profile.unicode {
                "✓"
            } else {
                "+"
            }
        }
        Tone::Warning => {
            if profile.unicode {
                "▲"
            } else {
                "!"
            }
        }
        Tone::Error => {
            if profile.unicode {
                "✕"
            } else {
                "x"
            }
        }
    };
    tone_text(&format!("{icon} {label}: {value}"), tone.into(), profile)
}

pub fn menu_line(label: &str, selected: bool, profile: StyleProfile) -> String {
    let pointer = if selected { if profile.unicode { "›" } else { ">" } } else { " " };
    let text = format!("{pointer} {label}");
    if selected { colorize(&text, "1;32", profile) } else { text }
}

pub fn menu_description(label: &str, selected: bool, profile: StyleProfile) -> String {
    let prefix = if selected { "    " } else { "  " };
    let text = format!("{prefix}{label}");
    muted(&text, profile)
}

pub fn tone_for_mode(mode: ProtectionMode) -> Tone {
    match mode {
        ProtectionMode::Inactive => Tone::Info,
        ProtectionMode::Active => Tone::Success,
        ProtectionMode::Degraded => Tone::Error,
        ProtectionMode::Recovering => Tone::Warning,
    }
}

pub fn tone_from_result(tone: ResultTone) -> Tone {
    match tone {
        ResultTone::Info => Tone::Info,
        ResultTone::Success => Tone::Success,
        ResultTone::Warning => Tone::Warning,
        ResultTone::Error => Tone::Error,
    }
}

fn colorize(label: &str, code: &str, profile: StyleProfile) -> String {
    if profile.color {
        format!("\u{1b}[{code}m{label}\u{1b}[0m")
    } else {
        label.to_owned()
    }
}

impl From<Tone> for ResultTone {
    fn from(value: Tone) -> Self {
        match value {
            Tone::Info => ResultTone::Info,
            Tone::Success => ResultTone::Success,
            Tone::Warning => ResultTone::Warning,
            Tone::Error => ResultTone::Error,
        }
    }
}
