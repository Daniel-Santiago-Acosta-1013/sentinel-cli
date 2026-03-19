use predicates::str::contains;

use crate::support::{
    cargo_binary, install_command, installed_binary, temp_home, write_executable,
};

#[test]
fn install_script_handles_first_time_install() {
    let install_root = temp_home();
    install_command(install_root.path(), &cargo_binary())
        .assert()
        .success()
        .stdout(contains("Sentinel action: install"));

    assert!(installed_binary(install_root.path()).exists());
}

#[test]
fn install_script_updates_older_binary() {
    let install_root = temp_home();
    let binary = installed_binary(install_root.path());
    write_executable(
        &binary,
        "#!/bin/sh\nif [ \"${SENTINEL_INTERNAL_MODE:-}\" = \"print-version\" ]; then\n  echo 0.0.1\n  exit 0\nfi\nexit 0\n",
    );

    install_command(install_root.path(), &cargo_binary())
        .assert()
        .success()
        .stdout(contains("Sentinel action: update"));
}

#[test]
fn install_script_reinstalls_damaged_binary() {
    let install_root = temp_home();
    let binary = installed_binary(install_root.path());
    write_executable(&binary, "#!/bin/sh\nexit 1\n");

    install_command(install_root.path(), &cargo_binary())
        .assert()
        .success()
        .stdout(contains("Sentinel action: reinstall"));
}

#[test]
fn install_script_reinstalls_same_version_when_artifact_differs() {
    let install_root = temp_home();
    let binary = installed_binary(install_root.path());
    write_executable(
        &binary,
        &format!(
            "#!/bin/sh\nif [ \"${{SENTINEL_INTERNAL_MODE:-}}\" = \"print-version\" ]; then\n  echo {}\n  exit 0\nfi\necho old-build\nexit 0\n",
            env!("CARGO_PKG_VERSION")
        ),
    );

    install_command(install_root.path(), &cargo_binary())
        .assert()
        .success()
        .stdout(contains("Sentinel action: reinstall"));
}
