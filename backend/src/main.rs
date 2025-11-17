use tokio::net::TcpListener;
use axum::{routing::get, Json, Router};
use serde::Serialize;
use dotenvy::dotenv;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check));

    let bind = env::var("BIND").ok().unwrap_or_else( || {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port: u16 = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
        format!("{host}:{port}")

    });
    let addr: SocketAddr = bind.parse().expect("invalid BIND/HOST/PORT");

    let listener  = TcpListener::bind(addr)
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