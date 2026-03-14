use predicates::str::contains;

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn guided_activation_and_disable_round_trip_preserves_state() {
    let home = temp_home();
    let port = next_port();

    scripted_command(&home, "enter,down,enter,confirm,enter,confirm,exit", port)
        .assert()
        .success()
        .stdout(contains("Protection: Active"))
        .stdout(contains("Protection: Inactive"));

    let state_path = home.path().join("state/state.json");
    let state = std::fs::read_to_string(state_path).expect("state file");
    assert!(state.contains("\"mode\": \"inactive\""));
}
