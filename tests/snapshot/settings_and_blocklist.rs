use predicates::str::contains;

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn blocked_domain_list_snapshot_shows_catalog_and_actions() {
    let home = temp_home();
    scripted_command(&home, "down,down,enter,enter,exit", next_port())
        .assert()
        .success()
        .stdout(contains("◆ Dominios bloqueados"))
        .stdout(contains("Dominio seleccionado: 2mdn.net"))
        .stdout(contains("Agregar dominio"))
        .stdout(contains("Editar dominio seleccionado"));
}

#[test]
fn blocked_domain_editor_snapshot_accepts_text_input() {
    let home = temp_home();
    scripted_command(
        &home,
        "down,down,enter,enter,enter,text:ejemplo.com,exit",
        next_port(),
    )
    .assert()
    .success()
    .stdout(contains("◆ Agregar dominio bloqueado"))
    .stdout(contains("Valor actual"))
    .stdout(contains("ejemplo.com"));
}

#[test]
fn blocked_domain_empty_state_is_explicit() {
    let home = temp_home();
    std::fs::write(home.path().join("blocklist.txt"), "").expect("write empty blocklist");

    scripted_command(&home, "down,down,enter,enter,exit", next_port())
        .assert()
        .success()
        .stdout(contains("◆ Dominios bloqueados"))
        .stdout(contains("No hay dominios bloqueados configurados todavia."));
}
