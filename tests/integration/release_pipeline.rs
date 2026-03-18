use predicates::str::contains;

use crate::support::release_fixtures::{
    extract_output_map, parse_key_values, read_channel_state, release_command,
    release_tempdir,
};

#[test]
fn authorized_release_builds_and_materializes_all_channels_in_mock_mode() {
    let state_dir = release_tempdir();
    let artifact_dir = release_tempdir();

    let authorize_output = release_command("authorize_release.sh")
        .env("RELEASE_TAG", "v0.1.0")
        .env("RELEASE_TAG_COMMIT", "abc123")
        .env("RELEASE_MAIN_HEAD", "abc123")
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
        .env("RELEASE_TAG", "v0.1.0")
        .env("RELEASE_TAG_COMMIT", "abc123")
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
        .env("RELEASE_MANIFEST_PATH", &manifest_path)
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success();

    release_command("publish_npm.sh")
        .env("RELEASE_MANIFEST_PATH", &manifest_path)
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success();

    release_command("publish_homebrew.sh")
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
    assert_eq!(github_state.get("VERSION"), Some(&"0.1.0".to_string()));
    assert_eq!(npm_state.get("VERSION"), Some(&"0.1.0".to_string()));
    assert_eq!(homebrew_state.get("VERSION"), Some(&"0.1.0".to_string()));
}

#[test]
fn partial_release_is_reported_when_homebrew_fails_after_npm() {
    let state_dir = release_tempdir();
    let artifact_dir = release_tempdir();

    let build_output = release_command("build_release_artifacts.sh")
        .env("RELEASE_TAG", "v0.1.0")
        .env("RELEASE_TAG_COMMIT", "abc123")
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
        .env("RELEASE_MANIFEST_PATH", &manifest_path)
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success();

    release_command("publish_npm.sh")
        .env("RELEASE_MANIFEST_PATH", &manifest_path)
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success();

    release_command("publish_homebrew.sh")
        .env("RELEASE_MANIFEST_PATH", &manifest_path)
        .env("RELEASE_STATE_DIR", state_dir.path())
        .env("RELEASE_FAIL_HOMEBREW", "1")
        .assert()
        .failure()
        .stderr(contains("simulated Homebrew publication failure"));

    release_command("summarize_release.sh")
        .env("RELEASE_TAG", "v0.1.0")
        .env("RELEASE_TAG_COMMIT", "abc123")
        .env("RELEASE_MAIN_HEAD", "abc123")
        .env("RELEASE_VERSION", "0.1.0")
        .env("RELEASE_STATE_DIR", state_dir.path())
        .assert()
        .success()
        .stdout(contains("GLOBAL_STATUS=partial"))
        .stdout(contains("NEXT_SAFE_ACTION=inspect channel states and resume only the missing or failed publication"));
}

#[test]
fn retry_detects_materialized_release_without_republishing() {
    let state_dir = release_tempdir();
    let artifact_dir = release_tempdir();

    let build_output = release_command("build_release_artifacts.sh")
        .env("RELEASE_TAG", "v0.1.0")
        .env("RELEASE_TAG_COMMIT", "abc123")
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
            .env("RELEASE_MANIFEST_PATH", &manifest_path)
            .env("RELEASE_STATE_DIR", state_dir.path())
            .assert()
            .success();
    }

    release_command("publish_npm.sh")
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
