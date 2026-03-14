use predicates::str::contains;

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn recovery_restores_fake_dns_after_an_active_session() {
    let home = temp_home();
    let port = next_port();

    scripted_command(&home, "enter,down,enter,confirm,exit", port)
        .assert()
        .success()
        .stdout(contains("Protection: Active"));

    scripted_command(&home, "down,down,down,down,enter,confirm,exit", port)
        .assert()
        .success()
        .stdout(contains("Recovery completed"));

    let network_state =
        std::fs::read_to_string(home.path().join("state/fake-network.json"))
            .expect("fake network state");
    assert!(network_state.contains("1.1.1.1"));
}
