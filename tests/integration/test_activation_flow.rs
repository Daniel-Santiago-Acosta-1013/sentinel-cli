use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn command() -> Command {
    Command::cargo_bin("sentinel").expect("binary exists")
}

#[test]
fn enable_then_status_then_disable_round_trip() {
    let temp = tempdir().unwrap();

    let mut enable = command();
    enable
        .env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10540")
        .arg("enable");
    enable.assert().success().stdout(predicate::str::contains("Protection is active"));

    let mut status = command();
    status
        .env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10540")
        .arg("status");
    status.assert().success().stdout(predicate::str::contains("active"));

    let mut disable = command();
    disable
        .env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10540")
        .arg("disable");
    disable.assert().success().stdout(predicate::str::contains("inactive"));
}
