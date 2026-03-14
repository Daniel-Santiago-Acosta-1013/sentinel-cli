use predicates::str::contains;

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn recovery_flow_restores_control_from_the_interactive_session() {
    let home = temp_home();
    let port = next_port();

    scripted_command(&home, "enter,down,enter,confirm,exit", port)
        .assert()
        .success()
        .stdout(contains("Protection: Active"));

    scripted_command(&home, "down,down,down,down,enter,confirm,exit", port)
        .assert()
        .success()
        .stdout(contains("Screen: Recovery"))
        .stdout(contains("Recovery completed"));
}

#[test]
fn degraded_runtime_is_visible_in_status_screen() {
    let home = temp_home();
    let port = next_port();

    scripted_command(&home, "enter,down,enter,confirm,exit", port).assert().success();

    let state_path = home.path().join("state/state.json");
    let mut state: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&state_path).expect("state file"))
            .expect("valid state");
    state["runtime_pid"] = serde_json::json!(999_999u32);
    std::fs::write(&state_path, serde_json::to_string_pretty(&state).expect("serialize"))
        .expect("write state");

    scripted_command(&home, "down,down,enter,exit", port)
        .assert()
        .success()
        .stdout(contains("Protection: Degraded"))
        .stdout(contains("Recovery is recommended"));
}
