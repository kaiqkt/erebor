use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;

pub struct App {
    router: Router,
    address: SocketAddr,
}

impl App {
    pub fn new(router: Router, address: SocketAddr) -> Self {
        Self { router, address }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.address)
            .await
            .with_context(|| format!("failed to listen on {}", self.address))?;

        axum::serve(listener, self.router)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .context("HTTP server failed")?;

        Ok(())
    }
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(%error, "failed to listen for shutdown signal");
    }
}
