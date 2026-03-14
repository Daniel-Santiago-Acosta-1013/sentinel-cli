use predicates::str::contains;

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn status_snapshot_surfaces_install_and_runtime_summary() {
    let home = temp_home();
    scripted_command(&home, "down,down,enter,exit", next_port())
        .assert()
        .success()
        .stdout(contains("Screen: Status"))
        .stdout(contains("Install action: Install"));
}

#[test]
fn recovery_snapshot_stays_explicit_and_guided() {
    let home = temp_home();
    let port = next_port();
    scripted_command(&home, "enter,down,enter,confirm,exit", port).assert().success();

    scripted_command(&home, "down,down,down,down,enter,confirm,exit", port)
        .assert()
        .success()
        .stdout(contains("Screen: Recovery"))
        .stdout(contains("Recovery completed"));
}
