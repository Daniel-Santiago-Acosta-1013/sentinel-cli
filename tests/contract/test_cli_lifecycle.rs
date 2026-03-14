use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn command() -> Command {
    Command::cargo_bin("sentinel").expect("binary exists")
}

#[test]
fn status_json_contains_required_contract_fields() {
    let temp = tempdir().unwrap();
    let mut cmd = command();
    cmd.env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10530")
        .args(["--json", "status"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"ok\": true"))
        .stdout(predicate::str::contains("\"command\": \"status\""))
        .stdout(predicate::str::contains("\"state\": \"inactive\""));
}

#[test]
fn enable_is_idempotent_when_already_active() {
    let temp = tempdir().unwrap();

    let mut first = command();
    first
        .env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10531")
        .arg("enable");
    first.assert().success();

    let mut second = command();
    second
        .env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10531")
        .arg("enable");
    second.assert().success().stdout(predicate::str::contains("already active"));
}

#[test]
fn disable_is_safe_when_runtime_is_inactive() {
    let temp = tempdir().unwrap();
    let mut cmd = command();
    cmd.env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10532")
        .arg("disable");
    cmd.assert().success().stdout(predicate::str::contains("inactive"));
}
