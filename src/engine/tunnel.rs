use std::net::SocketAddr;

use miette::{IntoDiagnostic, Result};
use tokio::net::UdpSocket;

/// Owns the local listener used by the DNS protection runtime.
pub struct TunnelHandle {
    socket: UdpSocket,
    bind_addr: SocketAddr,
}

impl TunnelHandle {
    pub async fn bind(bind_addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await.into_diagnostic()?;
        Ok(Self { socket, bind_addr })
    }

    pub fn bind_addr(&self) -> SocketAddr {
        self.bind_addr
    }

    pub fn socket(&self) -> &UdpSocket {
        &self.socket
    }
}
