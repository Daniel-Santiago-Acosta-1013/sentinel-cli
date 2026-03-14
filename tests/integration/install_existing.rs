use predicates::str::contains;

use crate::support::{
    cargo_binary, install_command, installed_binary, temp_home, write_executable,
};

#[test]
fn installer_updates_existing_version_and_reinstalls_broken_one() {
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

    write_executable(&binary, "#!/bin/sh\nexit 1\n");

    install_command(install_root.path(), &cargo_binary())
        .assert()
        .success()
        .stdout(contains("Sentinel action: reinstall"));
}
