use axum::{
    body::Body,
    extract::State,
    http::{Method, StatusCode, Uri},
    response::Response,
    Router,
};
use std::env;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
};
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let env_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info,frontend_proxy=debug".into());
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    let backend_url = env::var("BACKEND_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080".into());

    let port = env::var("FRONTEND_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let static_dir = env::var("STATIC_DIR")
        .unwrap_or_else(|_| "../frontend".into());

    info!("Backend URL: {}", backend_url);
    info!("Static files directory: {}", static_dir);
    info!("Frontend will listen on 0.0.0.0:{}", port);

    let client = reqwest::Client::builder()
        .build()?;

    let app = Router::new()
        .route("/login", axum::routing::any(proxy_handler))
        .route("/register", axum::routing::any(proxy_handler))
        .route("/health", axum::routing::any(proxy_handler))
        .route("/users", axum::routing::any(proxy_handler))
        .route("/projects", axum::routing::any(proxy_handler))
        .route("/api/{*path}", axum::routing::any(proxy_handler))
        .route("/users/{*path}", axum::routing::any(proxy_handler))
        .route("/projects/{*path}", axum::routing::any(proxy_handler))
        .route("/tasks/{*path}", axum::routing::any(proxy_handler))
        .route("/memberships/{*path}", axum::routing::any(proxy_handler))
        .route("/assignments/{*path}", axum::routing::any(proxy_handler))
        .route("/export/{*path}", axum::routing::any(proxy_handler))
        .route("/notifications/{*path}", axum::routing::any(proxy_handler))
        .fallback_service(
            ServeDir::new(&static_dir)
                .precompressed_gzip()
                .precompressed_br()
                .append_index_html_on_directories(true)
        )
        .layer(CorsLayer::permissive())
        .with_state(AppState {
            backend_url,
            client,
        });

    let addr = format!("0.0.0.0:{}", port);
    info!("Frontend proxy server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    backend_url: String,
    client: reqwest::Client,
}

async fn proxy_handler(
    State(state): State<AppState>,
    method: Method,
    uri: Uri,
    headers: axum::http::HeaderMap,
    body: Body,
) -> Response {
    let path = uri.path();
    let query = uri.query().unwrap_or("");

    let backend_path_str = if path.starts_with("/api") {
        let stripped = path.strip_prefix("/api").unwrap_or(path);
        if stripped.is_empty() {
            "/"
        } else {
            stripped
        }
    } else {
        path
    };

    let backend_path = if backend_path_str.is_empty() {
        "/".to_string()
    } else if !backend_path_str.starts_with('/') {
        format!("/{}", backend_path_str)
    } else {
        backend_path_str.to_string()
    };

    let backend_url = if query.is_empty() {
        format!("{}{}", state.backend_url, backend_path)
    } else {
        format!("{}{}?{}", state.backend_url, backend_path, query)
    };

    info!("Proxying {} {} -> {}", method, path, backend_url);

    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to read request body: {}", e);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to read request body"))
                .unwrap();
        }
    };

    let mut request_builder = state.client
        .request(method.clone(), &backend_url);

    if !body_bytes.is_empty() {
        request_builder = request_builder.body(body_bytes.to_vec());
    }

    for (name, value) in headers.iter() {
        let name_str = name.as_str();
        if !matches!(name_str, "host" | "connection" | "content-length" | "transfer-encoding" | "upgrade") {
            if let Ok(value_str) = value.to_str() {
                request_builder = request_builder.header(name_str, value_str);
            }
        }
    }
    
    if !body_bytes.is_empty() {
        request_builder = request_builder.header("content-length", body_bytes.len().to_string());
    }

    let response = match request_builder.send().await {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                info!("Backend responded with {} for {}", status, backend_url);
            } else {
                error!("Backend responded with error {} for {}", status, backend_url);
            }
            resp
        },
        Err(e) => {
            error!("Failed to proxy request to backend {}: {} (method: {}, path: {})", 
                backend_url, e, method, path);
            return Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .header("content-type", "application/json")
                .body(Body::from(format!(r#"{{"error":"Backend connection error: {}"}}"#, e)))
                .unwrap();
        }
    };

    let status = StatusCode::from_u16(response.status().as_u16())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let mut response_builder = Response::builder()
        .status(status);

    for (name, value) in response.headers() {
        let name_str = name.as_str();
        if !matches!(name_str, "connection" | "transfer-encoding" | "content-encoding" | "upgrade") {
            if let Ok(name_header) = axum::http::HeaderName::from_bytes(name.as_str().as_bytes()) {
                if let Ok(value_header) = axum::http::HeaderValue::from_bytes(value.as_bytes()) {
                    response_builder = response_builder.header(name_header, value_header);
                }
            }
        }
    }

    let body_bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to read backend response: {}", e);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to read backend response"))
                .unwrap();
        }
    };

    response_builder
        .body(Body::from(body_bytes))
        .unwrap_or_else(|_| {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to build response"))
                .unwrap()
        })
}
