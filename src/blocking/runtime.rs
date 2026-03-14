use std::{
    net::{SocketAddr, UdpSocket as StdUdpSocket},
    process::{Command, Stdio},
};

use miette::{Context, IntoDiagnostic, Result, miette};
use tokio::net::UdpSocket;
use tracing::{error, info};

use crate::{
    app::AppPaths,
    blocking::{blocklist::BlocklistBundle, resolver},
    storage::config::ConfigStore,
};

pub async fn run_runtime(paths: AppPaths) -> Result<()> {
    let config = ConfigStore::new(paths.clone()).load()?;
    let upstream: SocketAddr = config.upstream_dns.parse().into_diagnostic()?;
    let bind_addr = paths.runtime_addr()?;
    let socket = UdpSocket::bind(bind_addr).await.into_diagnostic()?;
    let blocklist = BlocklistBundle::load()?;
    info!("runtime listening on {bind_addr}");

    let mut buffer = [0u8; 4096];
    loop {
        let (size, addr) = socket.recv_from(&mut buffer).await.into_diagnostic()?;
        let payload = buffer[..size].to_vec();
        match resolver::handle_query(&payload, upstream, &blocklist).await {
            Ok(response) => {
                socket.send_to(&response, addr).await.into_diagnostic()?;
            }
            Err(err) => {
                error!("dns handling failed: {err:?}");
            }
        }
    }
}

pub fn spawn_background() -> Result<u32> {
    let current_exe = std::env::current_exe().into_diagnostic()?;
    let child = Command::new(current_exe)
        .env("SENTINEL_INTERNAL_MODE", "runtime")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .into_diagnostic()
        .context("failed to launch the Sentinel DNS runtime")?;
    Ok(child.id())
}

pub fn port_available(addr: SocketAddr) -> bool {
    StdUdpSocket::bind(addr).is_ok()
}

pub fn process_alive(pid: u32) -> bool {
    Command::new("kill")
        .arg("-0")
        .arg(pid.to_string())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn stop_process(pid: u32) -> Result<()> {
    if !process_alive(pid) {
        return Ok(());
    }

    let status = Command::new("kill")
        .arg("-TERM")
        .arg(pid.to_string())
        .status()
        .into_diagnostic()
        .context("failed to stop the Sentinel DNS runtime")?;
    if status.success() {
        Ok(())
    } else {
        Err(miette!("failed to stop runtime process {pid}"))
    }
}
