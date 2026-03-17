use predicates::str::contains;

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn status_snapshot_surfaces_install_and_runtime_summary() {
    let home = temp_home();
    scripted_command(&home, "down,down,enter,exit", next_port())
        .assert()
        .success()
        .stdout(contains("◆ Estado actual"))
        .stdout(contains("Accion sugerida"));
}

#[test]
fn recovery_snapshot_stays_explicit_and_guided() {
    let home = temp_home();
    let port = next_port();
    scripted_command(&home, "down,enter,enter,enter,exit", port).assert().success();

    scripted_command(&home, "down,down,down,down,enter,enter,enter,exit", port)
        .assert()
        .success()
        .stdout(contains("◆ Recuperacion completada"))
        .stdout(contains("Sentinel restauro el ultimo snapshot valido"));
}
