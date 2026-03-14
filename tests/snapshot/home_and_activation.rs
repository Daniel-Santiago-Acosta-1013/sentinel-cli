use predicates::str::contains;

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn home_snapshot_stays_clean_and_focused() {
    let home = temp_home();
    scripted_command(&home, "exit", next_port())
        .assert()
        .success()
        .stdout(contains("Sentinel\nProtection: Inactive"))
        .stdout(contains("Screen: Home"))
        .stdout(contains("> Run safety checks"));
}

#[test]
fn activation_snapshot_exposes_active_state() {
    let home = temp_home();
    scripted_command(&home, "enter,down,enter,confirm,exit", next_port())
        .assert()
        .success()
        .stdout(contains("Screen: Status"))
        .stdout(contains("Protection: Active"))
        .stdout(contains("Disable protection"));
}
