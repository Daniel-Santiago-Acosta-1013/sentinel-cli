use std::process::Command as ProcessCommand;

use predicates::str::contains;

use crate::support::release_fixtures::{
    create_release_repo_fixture, current_tag, current_version, parse_key_values,
    read_channel_state, release_command, release_tempdir, write_channel_state,
};

#[test]
fn workflow_is_centralized_in_dispatch_and_staged_jobs() {
    let workflow = std::fs::read_to_string(
        crate::support::release_fixtures::repo_root().join(".github/workflows/release.yml"),
    )
    .expect("read release workflow");

    assert!(workflow.contains("workflow_dispatch"));
    assert!(workflow.contains("inputs:"));
    assert!(workflow.contains("prepare-version"));
    assert!(workflow.contains("needs: prepare-version"));
    assert!(workflow.contains("update_versions.sh"));
    assert!(workflow.contains("git push origin HEAD:main"));
    assert!(workflow.contains("git push origin ${{ steps.align.outputs.RELEASE_TAG }}"));
}

#[test]
fn authorize_release_requires_main_head_equality() {
    let output = release_command("authorize_release.sh")
        .env("RELEASE_TAG", current_tag())
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
        .env("RELEASE_TAG", format!("{}-rc.1", current_tag()))
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
fn resolve_version_reports_mismatch_when_tag_and_project_diverge() {
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
        Some(current_version())
    );
    assert_eq!(values.get("TAG_VERSION").map(String::as_str), Some("9.9.9"));
}

#[test]
fn update_versions_aligns_surfaces_and_creates_commit_and_tag() {
    let repo = create_release_repo_fixture();
    let output = release_command("update_versions.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_VERSION_INPUT", "0.2.0")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let values = parse_key_values(&String::from_utf8_lossy(&output));
    assert_eq!(
        values.get("RELEASE_VERSION").map(String::as_str),
        Some("0.2.0")
    );
    assert_eq!(values.get("RELEASE_TAG").map(String::as_str), Some("v0.2.0"));

    let cargo_toml =
        std::fs::read_to_string(repo.path().join("Cargo.toml")).expect("read Cargo.toml");
    let package_json = std::fs::read_to_string(repo.path().join("packaging/npm/package.json"))
        .expect("read package.json");
    let formula = std::fs::read_to_string(
        repo.path().join("packaging/homebrew/sentinel.rb.tpl"),
    )
    .expect("read formula template");
    assert!(cargo_toml.contains("version = \"0.2.0\""));
    assert!(package_json.contains("\"version\": \"0.2.0\""));
    assert!(formula.contains("version \"0.2.0\""));

    let tag_list = ProcessCommand::new("git")
        .args(["tag", "--list", "v0.2.0"])
        .current_dir(repo.path())
        .output()
        .expect("list git tags");
    assert_eq!(String::from_utf8_lossy(&tag_list.stdout).trim(), "v0.2.0");
}

#[test]
fn inspect_release_state_reports_materialized_when_all_channels_exist() {
    let state_dir = release_tempdir();
    write_channel_state(
        state_dir.path(),
        "github-release",
        &[("STATUS", "materialized"), ("VERSION", current_version())],
    );
    write_channel_state(
        state_dir.path(),
        "npm",
        &[("STATUS", "materialized"), ("VERSION", current_version())],
    );
    write_channel_state(
        state_dir.path(),
        "homebrew",
        &[("STATUS", "materialized"), ("VERSION", current_version())],
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
    let tag = current_tag();

    std::fs::write(
        artifact_dir.path().join("release-manifest.env"),
        format!(
            "RELEASE_TAG={tag}\nRELEASE_VERSION={}\nSOURCE_COMMIT=abc123\nARTIFACT_DIR={}\nCANONICAL_ARCHIVE=/tmp/archive.tar.gz\nCANONICAL_ARCHIVE_SHA256=deadbeef\n",
            current_version(),
            artifact_dir.path().display()
        ),
    )
    .expect("write manifest");

    write_channel_state(
        state_dir.path(),
        "npm",
        &[
            ("STATUS", "incompatible"),
            ("VERSION", current_version()),
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
    let tag = current_tag();

    write_channel_state(
        state_dir.path(),
        "github-release",
        &[
            ("STATUS", "materialized"),
            ("VERSION", current_version()),
            ("COMMIT", "abc123"),
        ],
    );
    write_channel_state(
        state_dir.path(),
        "npm",
        &[("STATUS", "materialized"), ("VERSION", current_version())],
    );
    write_channel_state(
        state_dir.path(),
        "homebrew",
        &[("STATUS", "materialized"), ("VERSION", current_version())],
    );

    release_command("summarize_release.sh")
        .env("RELEASE_TAG", &tag)
        .env("RELEASE_TAG_COMMIT", "abc123")
        .env("RELEASE_MAIN_HEAD", "abc123")
        .env("RELEASE_VERSION", current_version())
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success()
        .stdout(contains("GLOBAL_STATUS=completed"))
        .stdout(contains("AUTHORIZED_COMMIT=abc123"))
        .stdout(contains(format!("TAG={tag}")))
        .stdout(contains("NEXT_SAFE_ACTION="));
}
