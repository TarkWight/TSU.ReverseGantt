use tokio::net::TcpListener;
use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/", get(root));

    let bind = env::var("BIND").ok().unwrap_or_else( || {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port: u16 = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
        format!("{host}:{port}")

    });
    let addr: SocketAddr = bind.parse().expect("invalid BIND/HOST/PORT");

    let listener  = TcpListener::bind(addr)
        .await
        .expect("bind failed");

    axum::serve(listener , app)
        .await
        .expect("server error");

}

async fn root() -> &'static str {
    "Reverse Gantt API"
}