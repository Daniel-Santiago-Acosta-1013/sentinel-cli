use predicates::str::contains;

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn starts_from_home_screen_without_flags() {
    let home = temp_home();
    scripted_command(&home, "exit", next_port())
        .assert()
        .success()
        .stdout(contains("Screen: Home"))
        .stdout(contains("Run safety checks"))
        .stdout(contains("Exit Sentinel"));
}

#[test]
fn risky_actions_require_explicit_confirmation() {
    let home = temp_home();
    scripted_command(&home, "down,enter,back,exit", next_port())
        .assert()
        .success()
        .stdout(contains("Screen: Confirm"))
        .stdout(contains("Action canceled before applying changes."));
}

#[test]
fn unsafe_activation_is_blocked_when_safety_fails() {
    let home = temp_home();
    let port = next_port();
    let mut command = scripted_command(&home, "enter,down,enter,confirm,exit", port);
    command.env("SENTINEL_SIMULATE_BUSY_PORT", "1");
    command
        .assert()
        .success()
        .stdout(contains("Protection: Degraded"))
        .stdout(contains("Safety checks failed"));
}
