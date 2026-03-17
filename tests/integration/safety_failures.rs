use predicates::str::contains;

use crate::support::{next_port, scripted_command, seed_fake_network, temp_home};

#[test]
fn safety_failure_prevents_enable_when_dns_port_is_busy() {
    let home = temp_home();
    let port = next_port();
    let mut command = scripted_command(&home, "enter,enter,enter,exit", port);
    command.env("SENTINEL_SIMULATE_BUSY_PORT", "1");
    command
        .assert()
        .success()
        .stdout(contains("El puerto DNS local"))
        .stdout(contains("Degradada"));
}

#[test]
fn safety_checks_warn_when_custom_dns_must_be_preserved() {
    let home = temp_home();
    seed_fake_network(&home, &[("Wi-Fi", &["1.1.1.1"]), ("Ethernet", &["8.8.8.8"])]);

    scripted_command(&home, "enter,enter,enter,down,enter,enter,exit", next_port())
        .assert()
        .success()
        .stdout(contains("◆ Logs de Sentinel"))
        .stdout(contains("Chequeo de seguridad"))
        .stdout(contains("Se detectaron DNS personalizados"))
        .stdout(contains("Advertencia"));
}
