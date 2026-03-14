use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn command() -> Command {
    Command::cargo_bin("sentinel").expect("binary exists")
}

#[test]
fn recover_reports_restored_in_json() {
    let temp = tempdir().unwrap();

    let mut enable = command();
    enable
        .env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10535")
        .arg("enable");
    enable.assert().success();

    let mut recover = command();
    recover
        .env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10535")
        .args(["--json", "recover"]);
    recover.assert().success().stdout(predicate::str::contains("\"restored\": true"));
}

#[test]
fn events_accept_limit_contract() {
    let temp = tempdir().unwrap();
    let mut cmd = command();
    cmd.env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10536")
        .args(["events", "--limit", "5"]);
    cmd.assert().success();
}
