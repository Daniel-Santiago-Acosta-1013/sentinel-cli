use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command as ProcessCommand,
};

use assert_cmd::Command;
use tempfile::TempDir;

pub fn release_tempdir() -> TempDir {
    tempfile::tempdir().expect("release tempdir")
}

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn release_script(name: &str) -> PathBuf {
    repo_root().join("scripts/release").join(name)
}

pub fn release_command(name: &str) -> Command {
    let mut command = Command::new("sh");
    command.arg(release_script(name)).current_dir(repo_root());
    command
}

pub fn write_channel_state(
    state_dir: &Path,
    channel: &str,
    values: &[(&str, &str)],
) -> PathBuf {
    fs::create_dir_all(state_dir).expect("create state dir");
    let path = state_dir.join(format!("{channel}.env"));
    let mut body = String::new();
    for (key, value) in values {
        body.push_str(key);
        body.push('=');
        body.push_str(value);
        body.push('\n');
    }
    fs::write(&path, body).expect("write channel state");
    path
}

pub fn read_channel_state(state_dir: &Path, channel: &str) -> BTreeMap<String, String> {
    let path = state_dir.join(format!("{channel}.env"));
    let contents = fs::read_to_string(path).expect("read channel state");
    parse_key_values(&contents)
}

pub fn parse_key_values(text: &str) -> BTreeMap<String, String> {
    text.lines()
        .filter_map(|line| {
            let (key, value) = line.split_once('=')?;
            Some((key.trim().to_string(), value.trim().to_string()))
        })
        .collect()
}

pub fn create_mock_binary(dir: &Path) -> PathBuf {
    let path = dir.join("sentinel");
    fs::write(
        &path,
        "#!/bin/sh\nif [ \"${SENTINEL_INTERNAL_MODE:-}\" = \"print-version\" ]; then\n  echo 0.1.1\n  exit 0\nfi\necho sentinel\n",
    )
    .expect("write mock sentinel");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&path).expect("metadata").permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions).expect("chmod");
    }
    path
}

pub fn extract_output_map(output: &[u8]) -> BTreeMap<String, String> {
    parse_key_values(&String::from_utf8_lossy(output))
}

pub fn create_archive_with_binary(archive: &Path, binary: &Path) {
    let parent = binary.parent().expect("binary parent");
    let status = ProcessCommand::new("tar")
        .arg("-czf")
        .arg(archive)
        .arg("-C")
        .arg(parent)
        .arg("sentinel")
        .status()
        .expect("create tar archive");
    assert!(status.success(), "tar should succeed");
}
