use predicates::{prelude::PredicateBooleanExt, str::contains};

use crate::support::{next_port, scripted_command, temp_home};

#[test]
fn starts_from_home_screen_without_flags() {
    let home = temp_home();
    scripted_command(&home, "exit", next_port())
        .assert()
        .success()
        .stdout(contains("____"))
        .stdout(contains("◆ Inicio"))
        .stdout(contains("Activar Sentinel"))
        .stdout(contains("Ajustes"))
        .stdout(contains("Riesgo: Normal").not())
        .stdout(contains("Salir de Sentinel"));
}

#[test]
fn risky_actions_require_explicit_confirmation() {
    let home = temp_home();
    scripted_command(&home, "enter,back,exit", next_port())
        .assert()
        .success()
        .stdout(contains("◆ Confirmacion"))
        .stdout(contains("La accion sensible fue cancelada antes de cambiar la red."));
}

#[test]
fn unsafe_activation_is_blocked_when_safety_fails() {
    let home = temp_home();
    let port = next_port();
    let mut command = scripted_command(&home, "enter,enter,enter,exit", port);
    command.env("SENTINEL_SIMULATE_BUSY_PORT", "1");
    command
        .assert()
        .success()
        .stdout(contains("Activacion con advertencias"))
        .stdout(contains("Proteccion: Degradada"))
        .stdout(contains("ya esta en uso por otro proceso"));
}

#[test]
fn status_view_surfaces_recent_logs_with_exact_context() {
    let home = temp_home();
    let port = next_port();
    let mut command =
        scripted_command(&home, "enter,enter,enter,down,enter,enter,exit", port);
    command.env("SENTINEL_SIMULATE_BUSY_PORT", "1");
    command
        .assert()
        .success()
        .stdout(contains("◆ Estado de Sentinel"))
        .stdout(contains("◆ Logs de Sentinel"))
        .stdout(contains("Chequeo de seguridad"))
        .stdout(contains("ya esta en uso por otro proceso."));
}

#[test]
fn activation_recovers_reclaimable_sentinel_port_automatically() {
    let home = temp_home();
    let port = next_port();
    let mut command = scripted_command(&home, "enter,enter,enter,exit", port);
    command
        .env("SENTINEL_SIMULATE_BUSY_PORT", "1")
        .env("SENTINEL_SIMULATE_RECLAIMABLE_PORT", "1");
    command
        .assert()
        .success()
        .stdout(contains("Sentinel activado"))
        .stdout(contains("Proteccion: Activa"));
}

#[test]
fn settings_branch_exposes_blocked_domain_management_without_leaving_the_menu_flow()
{
    let home = temp_home();
    scripted_command(&home, "down,down,enter,enter,exit", next_port())
        .assert()
        .success()
        .stdout(contains("◆ Ajustes"))
        .stdout(contains("◆ Dominios bloqueados"))
        .stdout(contains("Volver a Ajustes"));
}

#[test]
fn blocked_domain_crud_is_guided_and_keeps_the_catalog_consistent() {
    let home = temp_home();
    scripted_command(
        &home,
        "down,down,enter,enter,enter,text:example.com,enter,down,enter,text:edited-example.com,enter,down,down,enter,exit",
        next_port(),
    )
    .assert()
    .success()
    .stdout(contains("Se agrego `example.com` al catalogo activo."))
    .stdout(contains(
        "El dominio seleccionado ahora se guarda como `edited-example.com`.",
    ))
    .stdout(contains("Se elimino `edited-example.com` del catalogo activo."));
}

#[test]
fn invalid_or_duplicate_domains_are_rejected_with_explicit_copy() {
    let home = temp_home();
    scripted_command(
        &home,
        "down,down,enter,enter,enter,text:2mdn.net,enter,back,enter,text:http://bad-domain,enter,exit",
        next_port(),
    )
    .assert()
    .success()
    .stdout(contains("ese dominio ya existe en el catalogo activo"))
    .stdout(contains(
        "introduce solo el nombre de dominio, sin protocolo ni rutas",
    ));
}
