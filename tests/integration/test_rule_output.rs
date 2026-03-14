use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn command() -> Command {
    Command::cargo_bin("sentinel").expect("binary exists")
}

#[test]
fn rules_list_shows_table_headers() {
    let temp = tempdir().unwrap();
    let mut cmd = command();
    cmd.env("SENTINEL_HOME", temp.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", "10544")
        .args(["--no-color", "rules", "list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Kind"))
        .stdout(predicate::str::contains("Value"));
}
