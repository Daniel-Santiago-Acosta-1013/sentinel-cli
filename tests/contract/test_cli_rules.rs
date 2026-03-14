use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn command() -> Command {
    Command::cargo_bin("sentinel").expect("binary exists")
}

#[test]
fn allow_add_reports_domain_in_json() {
    let temp = tempdir().unwrap();
    let mut cmd = command();
    cmd.env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10533")
        .args(["--json", "allow", "add", "example.com"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"domain\": \"example.com\""));
}

#[test]
fn rules_list_includes_allow_entries() {
    let temp = tempdir().unwrap();

    let mut add = command();
    add.env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10534")
        .args(["allow", "add", "example.com"]);
    add.assert().success();

    let mut list = command();
    list.env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10534")
        .args(["rules", "list"]);
    list.assert().success().stdout(predicate::str::contains("example.com"));
}
