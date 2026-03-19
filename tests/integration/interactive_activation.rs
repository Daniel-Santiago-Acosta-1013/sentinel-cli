use std::io::Write;

use predicates::{prelude::PredicateBooleanExt, str::contains};

use crate::support::{
    activation_script, next_port, round_trip_activation_script, scripted_command,
    temp_home,
};

#[test]
fn guided_activation_and_disable_round_trip_preserves_state() {
    let home = temp_home();
    let port = next_port();

    scripted_command(&home, round_trip_activation_script(), port)
        .assert()
        .success()
        .stdout(contains("Proteccion: Activa"))
        .stdout(contains("Proteccion: Inactiva"));

    let state_path = home.path().join("state/state.json");
    let state = std::fs::read_to_string(state_path).expect("state file");
    assert!(state.contains("\"mode\": \"inactive\""));
}

#[test]
fn blocked_queries_feed_the_activity_table_since_activation() {
    let home = temp_home();
    let port = next_port();

    scripted_command(&home, activation_script(), port)
        .assert()
        .success()
        .stdout(contains("Proteccion: Activa"));

    let events_path = home.path().join("state/events.jsonl");
    let blocked_event = serde_json::json!({
        "id": "blocked-event-1",
        "timestamp": chrono::Utc::now(),
        "kind": "blocked_domain",
        "severity": "info",
        "blocked_domain": "2mdn.net",
        "message": "Sentinel bloqueo una consulta DNS para 2mdn.net"
    });
    std::fs::OpenOptions::new()
        .append(true)
        .open(&events_path)
        .expect("open events")
        .write_all(format!("{}\n", blocked_event).as_bytes())
        .expect("append blocked event");

    scripted_command(&home, "down,enter,exit", port)
        .assert()
        .success()
        .stdout(contains("Actividad de bloqueo"))
        .stdout(contains("Bloqueos desde la activac"))
        .stdout(contains("2mdn.net (1)"))
        .stdout(contains("Ultimo bloqueo"))
        .stdout(contains("Sin datos").not());

    scripted_command(&home, activation_script(), port)
        .assert()
        .success()
        .stdout(contains("Proteccion: Inactiva"));
}
