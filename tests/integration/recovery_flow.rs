use predicates::str::contains;

use crate::support::{
    activation_script, next_port, read_fake_network, recovery_script, scripted_command,
    seed_fake_network, temp_home,
};

#[test]
fn recovery_restores_fake_dns_after_an_active_session() {
    let home = temp_home();
    let port = next_port();
    seed_fake_network(
        &home,
        &[("Wi-Fi", &["1.1.1.1"]), ("USB 10/100 LAN", &["9.9.9.9"])],
    );

    scripted_command(&home, activation_script(), port)
        .assert()
        .success()
        .stdout(contains("Proteccion: Activa"));

    scripted_command(&home, recovery_script(), port)
        .assert()
        .success()
        .stdout(contains("Recuperacion completada"));

    let network_state = read_fake_network(&home);
    assert!(network_state.contains("1.1.1.1"));
    assert!(network_state.contains("9.9.9.9"));
}
