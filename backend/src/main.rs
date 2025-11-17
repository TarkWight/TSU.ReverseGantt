mod config;

use axum::{routing::get, Json, Router};
use dotenvy::dotenv;
use serde::Serialize;
use tokio::net::TcpListener;
use std::net::SocketAddr;

use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = Config::from_env()?;
    let addr: SocketAddr = config.bind_address().parse()?;

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check));

    let listener = TcpListener::bind(addr)
        .await
        .expect("bind failed");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "reverse-gantt-backend",
    })
}

async fn root() -> &'static str {
    "Reverse Gantt API"
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};

        let mut sigterm = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        let mut sigint  = signal(SignalKind::interrupt()).expect("failed to install SIGINT handler");

        tokio::select! {
            _ = sigterm.recv() => {
                tracing::info!("Received SIGTERM, shutting down");
            },
            _ = sigint.recv() => {
                tracing::info!("Received SIGINT (Ctrl+C), shutting down");
            },
        }
    }

    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
        tracing::info!("Received Ctrl+C, shutting down");
    }
}