#![allow(dead_code)]

pub mod release_fixtures;

use std::{
    fs,
    net::UdpSocket,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU16, Ordering},
    thread,
    time::Duration,
};

use assert_cmd::Command;
use hickory_proto::{
    op::{Message, Query, ResponseCode},
    rr::{Name, RecordType},
    serialize::binary::{BinEncodable, BinEncoder},
};
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

pub fn scripted_command_with_env(
    home: &TempDir,
    script: &str,
    port: u16,
    envs: &[(&str, &str)],
) -> Command {
    let mut command = scripted_command(home, script, port);
    for (key, value) in envs {
        command.env(key, value);
    }
    command
}

pub fn activation_script() -> &'static str {
    "enter,enter,enter,exit"
}

pub fn round_trip_activation_script() -> &'static str {
    "enter,enter,enter,enter,enter,enter,exit"
}

pub fn recovery_script() -> &'static str {
    "down,down,down,enter,enter,enter,exit"
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
    assert_cmd::cargo::cargo_bin("sentinel")
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

pub fn seed_fake_network(home: &TempDir, services: &[(&str, &[&str])]) {
    let state_dir = home.path().join("state");
    fs::create_dir_all(&state_dir).expect("create state dir");
    let mut payload = serde_json::json!({ "services": {} });
    for (service, dns) in services {
        payload["services"][service] = serde_json::json!(
            dns.iter().map(|item| item.to_string()).collect::<Vec<_>>()
        );
    }
    fs::write(
        state_dir.join("fake-network.json"),
        serde_json::to_string_pretty(&payload).expect("serialize fake network"),
    )
    .expect("write fake network");
}

pub fn read_fake_network(home: &TempDir) -> String {
    fs::read_to_string(home.path().join("state/fake-network.json"))
        .expect("read fake network")
}

pub fn query_dns_response_code(port: u16, domain: &str) -> ResponseCode {
    let mut message = Message::new();
    message.set_id(7);
    message.add_query(Query::query(
        Name::from_ascii(domain).expect("valid domain"),
        RecordType::A,
    ));

    let mut payload = Vec::new();
    let mut encoder = BinEncoder::new(&mut payload);
    message.emit(&mut encoder).expect("encode dns query");

    let socket = UdpSocket::bind("127.0.0.1:0").expect("bind local udp socket");
    socket
        .set_read_timeout(Some(Duration::from_millis(250)))
        .expect("set read timeout");

    for _ in 0..10 {
        socket
            .send_to(&payload, ("127.0.0.1", port))
            .expect("send dns query");
        let mut buffer = [0u8; 4096];
        match socket.recv_from(&mut buffer) {
            Ok((size, _)) => {
                let response =
                    Message::from_vec(&buffer[..size]).expect("decode dns response");
                return response.response_code();
            }
            Err(_) => thread::sleep(Duration::from_millis(100)),
        }
    }

    panic!("sentinel runtime did not answer on port {port}");
}
