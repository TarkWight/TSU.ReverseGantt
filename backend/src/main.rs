use tokio::net::TcpListener;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let listener  = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("bind failed");

    axum::serve(listener , app)
        .await
        .expect("server error");

}

async fn root() -> &'static str {
    "Reverse Gantt API"
}