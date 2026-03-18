use predicates::str::contains;

use crate::support::release_fixtures::{
    parse_key_values, read_channel_state, release_command, release_tempdir,
    write_channel_state,
};

#[test]
fn authorize_release_requires_main_head_equality() {
    let output = release_command("authorize_release.sh")
        .env("RELEASE_TAG", "v0.1.0")
        .env("RELEASE_TAG_COMMIT", "old-main-commit")
        .env("RELEASE_MAIN_HEAD", "current-main-head")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let values = parse_key_values(&String::from_utf8_lossy(&output));
    assert_eq!(values.get("AUTHORIZED").map(String::as_str), Some("false"));
    assert_eq!(
        values.get("BLOCK_REASON").map(String::as_str),
        Some("tag_not_at_main_head")
    );
}

#[test]
fn authorize_release_requires_stable_semver_tags() {
    let output = release_command("authorize_release.sh")
        .env("RELEASE_TAG", "v0.1.0-rc.1")
        .env("RELEASE_TAG_COMMIT", "main-head")
        .env("RELEASE_MAIN_HEAD", "main-head")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let values = parse_key_values(&String::from_utf8_lossy(&output));
    assert_eq!(values.get("AUTHORIZED").map(String::as_str), Some("false"));
    assert_eq!(
        values.get("BLOCK_REASON").map(String::as_str),
        Some("invalid_tag_format")
    );
}

#[test]
fn authorize_release_requires_tag_and_project_version_match() {
    let output = release_command("resolve_version.sh")
        .env("RELEASE_TAG", "v9.9.9")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let values = parse_key_values(&String::from_utf8_lossy(&output));
    assert_eq!(values.get("VERSION_MATCH").map(String::as_str), Some("false"));
    assert_eq!(
        values.get("PROJECT_VERSION").map(String::as_str),
        Some("0.1.0")
    );
    assert_eq!(values.get("TAG_VERSION").map(String::as_str), Some("9.9.9"));
}

#[test]
fn inspect_release_state_reports_materialized_when_all_channels_exist() {
    let state_dir = release_tempdir();
    write_channel_state(
        state_dir.path(),
        "github-release",
        &[("STATUS", "materialized"), ("VERSION", "0.1.0")],
    );
    write_channel_state(
        state_dir.path(),
        "npm",
        &[("STATUS", "materialized"), ("VERSION", "0.1.0")],
    );
    write_channel_state(
        state_dir.path(),
        "homebrew",
        &[("STATUS", "materialized"), ("VERSION", "0.1.0")],
    );

    let output = release_command("inspect_release_state.sh")
        .env("RELEASE_STATE_DIR", state_dir.path())
        .env("RELEASE_ALREADY_MATERIALIZED", "1")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let values = parse_key_values(&String::from_utf8_lossy(&output));
    assert_eq!(
        values.get("OVERALL_STATUS").map(String::as_str),
        Some("materialized")
    );
}

#[test]
fn publish_npm_blocks_incompatible_existing_state() {
    let state_dir = release_tempdir();
    let artifact_dir = release_tempdir();

    std::fs::write(
        artifact_dir.path().join("release-manifest.env"),
        format!(
            "RELEASE_TAG=v0.1.0\nRELEASE_VERSION=0.1.0\nSOURCE_COMMIT=abc123\nARTIFACT_DIR={}\nCANONICAL_ARCHIVE=/tmp/archive.tar.gz\nCANONICAL_ARCHIVE_SHA256=deadbeef\n",
            artifact_dir.path().display()
        ),
    )
    .expect("write manifest");

    write_channel_state(
        state_dir.path(),
        "npm",
        &[
            ("STATUS", "incompatible"),
            ("VERSION", "0.1.0"),
            ("DETAILS", "existing package differs from authorized release"),
        ],
    );

    release_command("publish_npm.sh")
        .env("RELEASE_MANIFEST_PATH", artifact_dir.path().join("release-manifest.env"))
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .failure()
        .stderr(contains("incompatible"));

    let state = read_channel_state(state_dir.path(), "npm");
    assert_eq!(state.get("STATUS").map(String::as_str), Some("incompatible"));
}

#[test]
fn summarize_release_includes_required_audit_fields() {
    let state_dir = release_tempdir();
    write_channel_state(
        state_dir.path(),
        "github-release",
        &[
            ("STATUS", "materialized"),
            ("VERSION", "0.1.0"),
            ("COMMIT", "abc123"),
        ],
    );
    write_channel_state(
        state_dir.path(),
        "npm",
        &[("STATUS", "materialized"), ("VERSION", "0.1.0")],
    );
    write_channel_state(
        state_dir.path(),
        "homebrew",
        &[("STATUS", "materialized"), ("VERSION", "0.1.0")],
    );

    release_command("summarize_release.sh")
        .env("RELEASE_TAG", "v0.1.0")
        .env("RELEASE_TAG_COMMIT", "abc123")
        .env("RELEASE_MAIN_HEAD", "abc123")
        .env("RELEASE_VERSION", "0.1.0")
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success()
        .stdout(contains("GLOBAL_STATUS=completed"))
        .stdout(contains("AUTHORIZED_COMMIT=abc123"))
        .stdout(contains("TAG=v0.1.0"))
        .stdout(contains("NEXT_SAFE_ACTION="));
}
