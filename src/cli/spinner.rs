use super::styles::StyleProfile;

const UNICODE_FRAMES: [&str; 8] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"];
const ASCII_FRAMES: [&str; 4] = ["-", "\\", "|", "/"];

pub fn frame(step: usize, profile: StyleProfile) -> &'static str {
    if profile.unicode {
        UNICODE_FRAMES[step % UNICODE_FRAMES.len()]
    } else {
        ASCII_FRAMES[step % ASCII_FRAMES.len()]
    }
}

pub fn line(label: &str, step: usize, profile: StyleProfile) -> String {
    format!("{} {}", frame(step, profile), label)
}
