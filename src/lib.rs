use axum::http::StatusCode;
use axum::{Router, routing::get};

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn run(listener: tokio::net::TcpListener) {
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check));

    axum::serve(listener, app).await.unwrap();
}
