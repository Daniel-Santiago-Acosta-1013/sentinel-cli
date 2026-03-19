use predicates::{prelude::*, str::contains};

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn status_snapshot_surfaces_install_and_runtime_summary() {
    let home = temp_home();
    scripted_command(&home, "down,enter,exit", next_port())
        .assert()
        .success()
        .stdout(contains("◆ Estado de Sentinel"))
        .stdout(contains("Actividad de bloqueo"))
        .stdout(contains("Accion sugerida").not())
        .stdout(contains("Logs"));
}

#[test]
fn blocked_activation_snapshot_stays_explicit_and_guided() {
    let home = temp_home();
    let port = next_port();
    let mut command = scripted_command(&home, "enter,enter,enter,exit", port);
    command.env("SENTINEL_SIMULATE_BUSY_PORT", "1");
    command
        .assert()
        .success()
        .stdout(contains("Activacion con advertencias"))
        .stdout(contains("Proteccion: Degradada"))
        .stdout(contains("Razon exacta").not())
        .stdout(contains("ya esta en uso por otro proceso."));
}

#[test]
fn logs_view_returns_to_previous_context() {
    let home = temp_home();
    let port = next_port();
    let mut command =
        scripted_command(&home, "enter,enter,enter,down,enter,enter,back,exit", port);
    command.env("SENTINEL_SIMULATE_BUSY_PORT", "1");
    command
        .assert()
        .success()
        .stdout(contains("◆ Logs de Sentinel"))
        .stdout(contains("Volviste a la vista anterior sin perder el contexto."))
        .stdout(contains("◆ Estado de Sentinel"));
}

#[test]
fn recovery_snapshot_stays_explicit_and_guided() {
    let home = temp_home();
    let port = next_port();
    scripted_command(&home, "enter,enter,enter,exit", port).assert().success();

    scripted_command(&home, "down,down,down,enter,enter,enter,exit", port)
        .assert()
        .success()
        .stdout(contains("◆ Recuperacion completada"))
        .stdout(contains("Sentinel restauro el ultimo snapshot valido"));
}
