use crate::routes::health_check;
use crate::routes::subscribe;
use axum::routing::post;
use axum::{Router, routing::get};

pub async fn run(listener: tokio::net::TcpListener) {
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    axum::serve(listener, app).await.unwrap();
}
