use predicates::{prelude::*, str::contains};

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn home_snapshot_stays_clean_and_focused() {
    let home = temp_home();
    scripted_command(&home, "exit", next_port())
        .assert()
        .success()
        .stdout(contains(env!("CARGO_PKG_VERSION")))
        .stdout(contains("◆ Inicio"))
        .stdout(contains("Elige una accion principal").not())
        .stdout(contains("Campo").not())
        .stdout(contains("Ejecutar chequeos de seguridad").not())
        .stdout(contains("› Activar Sentinel"));
}

#[test]
fn activation_snapshot_exposes_active_state() {
    let home = temp_home();
    scripted_command(&home, "enter,enter,enter,exit", next_port())
        .assert()
        .success()
        .stdout(contains("◆ Sentinel activado"))
        .stdout(contains("Proteccion: Activa"))
        .stdout(contains("Volver al inicio"));
}
