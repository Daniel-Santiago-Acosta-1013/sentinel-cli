use predicates::str::contains;

use crate::support::{
    cargo_binary, install_command, installed_binary, next_port, temp_home,
};

#[test]
fn installer_places_sentinel_in_the_target_path_and_binary_is_callable() {
    let install_root = temp_home();
    let home = temp_home();
    let port = next_port();

    install_command(install_root.path(), &cargo_binary())
        .assert()
        .success()
        .stdout(contains("Executable path"));

    let installed = installed_binary(install_root.path());
    assert!(installed.exists());

    let mut command = std::process::Command::new(installed);
    command
        .env("SENTINEL_HOME", home.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", port.to_string())
        .env("SENTINEL_SCRIPT", "exit");
    let output = command.output().expect("run installed sentinel");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Screen: Home"));
}
