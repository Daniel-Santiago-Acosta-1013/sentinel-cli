use predicates::str::contains;

use crate::support::{
    next_port, scripted_command, scripted_command_with_env, seed_fake_network, temp_home,
};

#[test]
fn recovery_flow_restores_control_from_the_interactive_session() {
    let home = temp_home();
    let port = next_port();

    scripted_command(&home, "enter,down,enter,confirm,exit", port)
        .assert()
        .success()
        .stdout(contains("Proteccion: Activa"));

    scripted_command(&home, "down,down,down,down,enter,confirm,exit", port)
        .assert()
        .success()
        .stdout(contains("Pantalla: Recuperacion"))
        .stdout(contains("Sentinel restauro el ultimo snapshot valido"));
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
        .stdout(contains("Proteccion: Degradada"))
        .stdout(contains("Se recomienda recuperar la red"));
}

#[test]
fn restore_mismatch_stays_degraded_and_requests_recovery() {
    let home = temp_home();
    let port = next_port();

    scripted_command(&home, "enter,down,enter,confirm,exit", port)
        .assert()
        .success()
        .stdout(contains("Proteccion: Activa"));

    scripted_command_with_env(
        &home,
        "down,enter,confirm,exit",
        port,
        &[("SENTINEL_SIMULATE_RESTORE_MISMATCH", "1")],
    )
    .assert()
    .success()
    .stdout(contains("Proteccion: Degradada"))
    .stdout(contains("La restauracion no coincide con el snapshot"));
}

#[test]
fn startup_with_previous_degraded_state_prioritizes_recovery() {
    let home = temp_home();
    seed_fake_network(
        &home,
        &[("Wi-Fi", &["1.1.1.1"]), ("USB 10/100 LAN", &["9.9.9.9"])],
    );
    let state_path = home.path().join("state/state.json");
    std::fs::create_dir_all(state_path.parent().expect("state dir"))
        .expect("create state dir");
    std::fs::write(
        &state_path,
        serde_json::json!({
            "mode": "degraded",
            "status_summary": "La restauracion no coincide con el snapshot en: Wi-Fi.",
            "risk_level": "critical",
            "last_message": "Sentinel detecto diferencias y requiere recuperacion.",
            "runtime_pid": null,
            "runtime_addr": null,
            "snapshot_id": "snapshot-prueba",
            "last_transition_at": "2026-03-16T00:00:00Z",
            "blocklist_version": "0.1.0-121",
            "blocklist_domain_count": 121,
            "last_safety_check": null,
            "last_verification_result": {
                "checked_at": "2026-03-16T00:00:00Z",
                "matches_snapshot": false,
                "mismatched_services": ["Wi-Fi"],
                "summary": "La restauracion no coincide con el snapshot en: Wi-Fi."
            }
        })
        .to_string(),
    )
    .expect("write degraded state");

    scripted_command(&home, "exit", next_port())
        .assert()
        .success()
        .stdout(contains("Pantalla: Recuperacion"))
        .stdout(contains("Recuperar red"));
}
