use predicates::str::contains;

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn home_snapshot_stays_clean_and_focused() {
    let home = temp_home();
    scripted_command(&home, "exit", next_port())
        .assert()
        .success()
        .stdout(contains("CLI guiada para proteger tu DNS"))
        .stdout(contains("◆ Inicio"))
        .stdout(contains("› Ejecutar chequeos de seguridad"));
}

#[test]
fn activation_snapshot_exposes_active_state() {
    let home = temp_home();
    scripted_command(&home, "down,enter,enter,enter,exit", next_port())
        .assert()
        .success()
        .stdout(contains("◆ Proteccion activada"))
        .stdout(contains("Proteccion: Activa"))
        .stdout(contains("Volver al inicio"));
}
