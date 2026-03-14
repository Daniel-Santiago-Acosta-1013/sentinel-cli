use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn command() -> Command {
    Command::cargo_bin("sentinel").expect("binary exists")
}

#[test]
fn recover_restores_state_after_enable() {
    let temp = tempdir().unwrap();

    let mut enable = command();
    enable
        .env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10545")
        .arg("enable");
    enable.assert().success();

    let mut recover = command();
    recover
        .env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10545")
        .arg("recover");
    recover.assert().success().stdout(predicate::str::contains("Recovery completed"));
}
