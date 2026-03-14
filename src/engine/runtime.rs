use miette::{IntoDiagnostic, Result};
use tokio::signal;
use tracing::{error, info};

use crate::{
    app::AppPaths,
    engine::{dns, tunnel::TunnelHandle},
    storage::config::ConfigStore,
};

pub async fn run_runtime(paths: AppPaths) -> Result<()> {
    let config_store = ConfigStore::new(paths.clone());
    let config = config_store.load()?;
    let tunnel = TunnelHandle::bind(paths.runtime_addr()?).await?;
    info!("runtime listening on {}", tunnel.bind_addr());

    let mut buf = [0u8; 4096];
    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                info!("runtime shutting down");
                break;
            }
            recv = tunnel.socket().recv_from(&mut buf) => {
                let (size, addr) = recv.into_diagnostic()?;
                let data = buf[..size].to_vec();
                match dns::handle_query(&data, dns::default_upstream(), &config).await {
                    Ok(response) => {
                        tunnel.socket().send_to(&response, addr).await.into_diagnostic()?;
                    }
                    Err(err) => {
                        error!("dns handling failed: {err:?}");
                    }
                }
            }
        }
    }
    Ok(())
}
