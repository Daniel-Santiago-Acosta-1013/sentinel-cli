use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn command() -> Command {
    Command::cargo_bin("sentinel").expect("binary exists")
}

#[test]
fn allow_rule_is_added_and_removed() {
    let temp = tempdir().unwrap();

    let mut add = command();
    add.env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10543")
        .args(["allow", "add", "example.com"]);
    add.assert().success();

    let mut remove = command();
    remove
        .env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10543")
        .args(["allow", "remove", "example.com"]);
    remove.assert().success().stdout(predicate::str::contains("removed"));
}
