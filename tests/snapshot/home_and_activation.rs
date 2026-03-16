use predicates::str::contains;

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn home_snapshot_stays_clean_and_focused() {
    let home = temp_home();
    scripted_command(&home, "exit", next_port())
        .assert()
        .success()
        .stdout(contains("Sentinel\nCLI guiada"))
        .stdout(contains("Pantalla: Inicio"))
        .stdout(contains("> Ejecutar chequeos de seguridad"));
}

#[test]
fn activation_snapshot_exposes_active_state() {
    let home = temp_home();
    scripted_command(&home, "enter,down,enter,confirm,exit", next_port())
        .assert()
        .success()
        .stdout(contains("Pantalla: Estado"))
        .stdout(contains("Proteccion: Activa"))
        .stdout(contains("Desactivar proteccion"));
}
