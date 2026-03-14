use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn command() -> Command {
    Command::cargo_bin("sentinel").expect("binary exists")
}

#[test]
fn status_human_output_is_titled() {
    let temp = tempdir().unwrap();
    let mut cmd = command();
    cmd.env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10541")
        .args(["--no-color", "status"]);
    cmd.assert().success().stdout(predicate::str::contains("Sentinel Status"));
}

#[test]
fn json_output_remains_stable_for_status() {
    let temp = tempdir().unwrap();
    let mut cmd = command();
    cmd.env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10542")
        .args(["--json", "status"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"command\": \"status\""))
        .stdout(predicate::str::contains("\"message\": \"Status loaded\""));
}
