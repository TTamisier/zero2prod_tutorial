use axum::Form;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Router, routing::get};

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(Form(_form): Form<FormData>) -> StatusCode {
    StatusCode::OK
}

pub async fn run(listener: tokio::net::TcpListener) {
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    axum::serve(listener, app).await.unwrap();
}
