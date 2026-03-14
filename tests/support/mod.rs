#![allow(dead_code)]

use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU16, Ordering},
};

use assert_cmd::Command;
use tempfile::TempDir;

static NEXT_PORT: AtomicU16 = AtomicU16::new(19053);

pub fn temp_home() -> TempDir {
    tempfile::tempdir().expect("temp home")
}

pub fn next_port() -> u16 {
    NEXT_PORT.fetch_add(1, Ordering::SeqCst)
}

pub fn scripted_command(home: &TempDir, script: &str, port: u16) -> Command {
    let mut command = Command::cargo_bin("sentinel").expect("sentinel binary");
    command
        .env("SENTINEL_HOME", home.path())
        .env("SENTINEL_FAKE_PLATFORM", "1")
        .env("SENTINEL_DNS_PORT", port.to_string())
        .env("SENTINEL_SCRIPT", script);
    command
}

pub fn install_command(install_dir: &Path, source_bin: &Path) -> Command {
    let mut command = Command::new("sh");
    command
        .arg("scripts/install-sentinel.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("SENTINEL_INSTALL_DIR", install_dir)
        .env("SENTINEL_INSTALL_SOURCE", source_bin);
    command
}

pub fn cargo_binary() -> PathBuf {
    PathBuf::from(assert_cmd::cargo::cargo_bin("sentinel"))
}

pub fn installed_binary(install_dir: &Path) -> PathBuf {
    install_dir.join("sentinel")
}

pub fn write_executable(path: &Path, contents: &str) {
    fs::write(path, contents).expect("write executable");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(path).expect("metadata").permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).expect("chmod");
    }
}
