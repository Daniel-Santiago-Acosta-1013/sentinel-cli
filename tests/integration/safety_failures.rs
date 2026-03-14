use predicates::str::contains;

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn safety_failure_prevents_enable_when_dns_port_is_busy() {
    let home = temp_home();
    let port = next_port();
    let mut command = scripted_command(&home, "enter,down,enter,confirm,exit", port);
    command.env("SENTINEL_SIMULATE_BUSY_PORT", "1");
    command
        .assert()
        .success()
        .stdout(contains("The local DNS port"))
        .stdout(contains("Protection: Degraded"));
}
