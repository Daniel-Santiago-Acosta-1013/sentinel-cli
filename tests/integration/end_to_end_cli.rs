use predicates::str::contains;

use crate::support::{
    cargo_binary, install_command, installed_binary, next_port,
    round_trip_activation_script, temp_home,
};

#[test]
fn installed_binary_can_run_the_interactive_flow_end_to_end() {
    let install_root = temp_home();
    let home = temp_home();
    let port = next_port();

    install_command(install_root.path(), &cargo_binary())
        .assert()
        .success()
        .stdout(contains("Sentinel action: install"));

    let installed = installed_binary(install_root.path());
    let output = std::process::Command::new(installed)
        .env("SENTINEL_HOME", home.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", port.to_string())
        .env("SENTINEL_SCRIPT", round_trip_activation_script())
        .output()
        .expect("run installed sentinel");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Proteccion: Activa"));
    assert!(stdout.contains("Proteccion: Inactiva"));
}

#[test]
fn installed_binary_persists_blocked_domain_crud_changes() {
    let install_root = temp_home();
    let home = temp_home();
    let port = next_port();

    install_command(install_root.path(), &cargo_binary())
        .assert()
        .success()
        .stdout(contains("Sentinel action: install"));

    let installed = installed_binary(install_root.path());
    let output = std::process::Command::new(installed)
        .env("SENTINEL_HOME", home.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", port.to_string())
        .env(
            "SENTINEL_SCRIPT",
            "down,down,enter,enter,enter,text:example.com,enter,down,enter,text:edited-example.com,enter,exit",
        )
        .output()
        .expect("run installed sentinel");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Se agrego `example.com` al catalogo activo."));
    assert!(stdout.contains(
        "El dominio seleccionado ahora se guarda como `edited-example.com`."
    ));

    let blocklist =
        std::fs::read_to_string(home.path().join("blocklist.txt")).expect("read blocklist");
    assert!(blocklist.contains("edited-example.com"));
    assert!(!blocklist.contains("\nexample.com\n"));
}
