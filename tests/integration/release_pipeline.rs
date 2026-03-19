use predicates::str::contains;

use crate::support::release_fixtures::{
    create_release_repo_fixture, extract_output_map, parse_key_values, read_channel_state,
    release_command, release_tempdir,
};

#[test]
fn version_alignment_job_commits_and_tags_release_in_fixture_repo() {
    let repo = create_release_repo_fixture();
    let output = release_command("update_versions.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_VERSION_INPUT", "0.2.0")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let values = extract_output_map(&output);
    assert_eq!(
        values.get("RELEASE_VERSION").map(String::as_str),
        Some("0.2.0")
    );
    assert_eq!(values.get("RELEASE_TAG").map(String::as_str), Some("v0.2.0"));

    let head = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo.path())
        .output()
        .expect("git rev-parse");
    assert_eq!(
        values.get("RELEASE_COMMIT").map(String::as_str),
        Some(String::from_utf8_lossy(&head.stdout).trim())
    );
}

#[test]
fn authorized_release_builds_and_materializes_all_channels_after_version_alignment()
{
    let repo = create_release_repo_fixture();
    let state_dir = release_tempdir();
    let artifact_dir = release_tempdir();

    let align_output = release_command("update_versions.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_VERSION_INPUT", "0.2.0")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let align_values = extract_output_map(&align_output);

    let authorize_output = release_command("authorize_release.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_TAG", "v0.2.0")
        .env(
            "RELEASE_TAG_COMMIT",
            align_values.get("RELEASE_COMMIT").expect("release commit"),
        )
        .env(
            "RELEASE_MAIN_HEAD",
            align_values.get("RELEASE_COMMIT").expect("release commit"),
        )
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let authorize_values = extract_output_map(&authorize_output);
    assert_eq!(
        authorize_values.get("AUTHORIZED").map(String::as_str),
        Some("true")
    );

    let build_output = release_command("build_release_artifacts.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_VERSION", "0.2.0")
        .env("RELEASE_TAG", "v0.2.0")
        .env(
            "SOURCE_COMMIT",
            align_values.get("RELEASE_COMMIT").expect("release commit"),
        )
        .env("RELEASE_ARTIFACT_DIR", artifact_dir.path())
        .env("RELEASE_USE_MOCK_BUILD", "1")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let build_values = extract_output_map(&build_output);
    let manifest_path = build_values
        .get("MANIFEST_PATH")
        .expect("manifest path")
        .clone();

    release_command("publish_github_release.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_MANIFEST_PATH", &manifest_path)
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success();

    release_command("publish_npm.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_MANIFEST_PATH", &manifest_path)
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success();

    release_command("publish_homebrew.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_MANIFEST_PATH", &manifest_path)
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success();

    let inspect_output = release_command("inspect_release_state.sh")
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let inspect_values = parse_key_values(&String::from_utf8_lossy(&inspect_output));
    assert_eq!(
        inspect_values.get("OVERALL_STATUS").map(String::as_str),
        Some("completed")
    );

    let github_state = read_channel_state(state_dir.path(), "github-release");
    let npm_state = read_channel_state(state_dir.path(), "npm");
    let homebrew_state = read_channel_state(state_dir.path(), "homebrew");
    assert_eq!(github_state.get("VERSION"), Some(&"0.2.0".to_string()));
    assert_eq!(npm_state.get("VERSION"), Some(&"0.2.0".to_string()));
    assert_eq!(homebrew_state.get("VERSION"), Some(&"0.2.0".to_string()));
}

#[test]
fn partial_release_is_reported_when_homebrew_fails_after_npm() {
    let repo = create_release_repo_fixture();
    let state_dir = release_tempdir();
    let artifact_dir = release_tempdir();

    let align_output = release_command("update_versions.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_VERSION_INPUT", "0.2.0")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let align_values = extract_output_map(&align_output);

    let build_output = release_command("build_release_artifacts.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_VERSION", "0.2.0")
        .env("RELEASE_TAG", "v0.2.0")
        .env(
            "SOURCE_COMMIT",
            align_values.get("RELEASE_COMMIT").expect("release commit"),
        )
        .env("RELEASE_ARTIFACT_DIR", artifact_dir.path())
        .env("RELEASE_USE_MOCK_BUILD", "1")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let build_values = extract_output_map(&build_output);
    let manifest_path = build_values
        .get("MANIFEST_PATH")
        .expect("manifest path")
        .clone();

    release_command("publish_github_release.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_MANIFEST_PATH", &manifest_path)
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success();

    release_command("publish_npm.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_MANIFEST_PATH", &manifest_path)
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success();

    release_command("publish_homebrew.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_MANIFEST_PATH", &manifest_path)
        .env("RELEASE_STATE_DIR", state_dir.path())
        .env("RELEASE_FAIL_HOMEBREW", "1")
        .assert()
        .failure()
        .stderr(contains("simulated Homebrew publication failure"));

    release_command("summarize_release.sh")
        .env("RELEASE_TAG", "v0.2.0")
        .env(
            "RELEASE_TAG_COMMIT",
            align_values.get("RELEASE_COMMIT").expect("release commit"),
        )
        .env(
            "RELEASE_MAIN_HEAD",
            align_values.get("RELEASE_COMMIT").expect("release commit"),
        )
        .env("RELEASE_VERSION", "0.2.0")
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success()
        .stdout(contains("GLOBAL_STATUS=partial"))
        .stdout(contains(
            "NEXT_SAFE_ACTION=inspect channel states and resume only the missing or failed publication",
        ));
}

#[test]
fn retry_detects_materialized_release_without_republishing() {
    let repo = create_release_repo_fixture();
    let state_dir = release_tempdir();
    let artifact_dir = release_tempdir();

    let align_output = release_command("update_versions.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_VERSION_INPUT", "0.2.0")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let align_values = extract_output_map(&align_output);

    let build_output = release_command("build_release_artifacts.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_VERSION", "0.2.0")
        .env("RELEASE_TAG", "v0.2.0")
        .env(
            "SOURCE_COMMIT",
            align_values.get("RELEASE_COMMIT").expect("release commit"),
        )
        .env("RELEASE_ARTIFACT_DIR", artifact_dir.path())
        .env("RELEASE_USE_MOCK_BUILD", "1")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let build_values = extract_output_map(&build_output);
    let manifest_path = build_values
        .get("MANIFEST_PATH")
        .expect("manifest path")
        .clone();

    for script in [
        "publish_github_release.sh",
        "publish_npm.sh",
        "publish_homebrew.sh",
    ] {
        release_command(script)
            .env("RELEASE_REPO_ROOT", repo.path())
            .env("RELEASE_MANIFEST_PATH", &manifest_path)
            .env("RELEASE_STATE_DIR", state_dir.path())
            .assert()
            .success();
    }

    release_command("publish_npm.sh")
        .env("RELEASE_REPO_ROOT", repo.path())
        .env("RELEASE_MANIFEST_PATH", &manifest_path)
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success()
        .stdout(contains("STATUS=materialized"));

    release_command("inspect_release_state.sh")
        .env("RELEASE_STATE_DIR", state_dir.path())
        .env("RELEASE_ALREADY_MATERIALIZED", "1")
        .assert()
        .success()
        .stdout(contains("OVERALL_STATUS=materialized"));
}
