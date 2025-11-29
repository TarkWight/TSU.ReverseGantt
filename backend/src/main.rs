mod config;
mod db;
mod domain;
mod utils;
mod services;
mod api;

use std::net::SocketAddr;

use axum::Router;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber;

use crate::config::Config;
use crate::services::{
    DependencyServiceImpl,
    DependencyService,
    ProjectServiceImpl,
    ProjectService,
    ReviewServiceImpl,
    ReviewService,
    ScheduleServiceImpl,
    ScheduleService,
    TaskServiceImpl,
    TaskService,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let config = Config::from_env()?;
    let addr: SocketAddr = config.bind_address().parse()?;

    tracing::info!("Connecting to database...");
    let pool = db::create_pool(&config.database.url).await?;
    tracing::info!("Database connection established");

    let project_service: Box<dyn ProjectService> =
        Box::new(ProjectServiceImpl::new(pool.clone()));
    let task_service: Box<dyn TaskService> =
        Box::new(TaskServiceImpl::new(pool.clone()));
    let dependency_service: Box<dyn DependencyService> =
        Box::new(DependencyServiceImpl::new(pool.clone()));
    let schedule_service: Box<dyn ScheduleService> =
        Box::new(ScheduleServiceImpl::new(pool.clone()));
    let review_service: Box<dyn ReviewService> =
        Box::new(ReviewServiceImpl::new(pool));

    let app: Router = api::create_router(
        project_service,
        task_service,
        dependency_service,
        schedule_service,
        review_service,
    );

    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};

        let mut sigterm =
            signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        let mut sigint =
            signal(SignalKind::interrupt()).expect("failed to install SIGINT handler");

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