pub fn header(label: &str, no_color: bool) -> String {
    if no_color { label.to_owned() } else { format!("\u{001b}[1;36m{label}\u{001b}[0m") }
}

pub fn accent(label: &str, no_color: bool) -> String {
    if no_color { label.to_owned() } else { format!("\u{001b}[1;34m{label}\u{001b}[0m") }
}
