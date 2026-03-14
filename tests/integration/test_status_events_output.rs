use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn command() -> Command {
    Command::cargo_bin("sentinel").expect("binary exists")
}

#[test]
fn events_output_lists_recent_items() {
    let temp = tempdir().unwrap();

    let mut enable = command();
    enable
        .env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10546")
        .arg("enable");
    enable.assert().success();

    let mut events = command();
    events
        .env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10546")
        .args(["--no-color", "events", "--limit", "5"]);
    events
        .assert()
        .success()
        .stdout(predicate::str::contains("Recent Events"))
        .stdout(predicate::str::contains("activate"));
}
