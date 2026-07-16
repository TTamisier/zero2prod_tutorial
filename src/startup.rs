use crate::routes::health_check;
use crate::routes::subscribe;
use axum::routing::post;
use axum::{Router, routing::get};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

pub async fn run(listener: tokio::net::TcpListener, db_pool: PgPool) {
    tracing::info!("Starting server on {}", listener.local_addr().unwrap());
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
        .with_state(db_pool);

    axum::serve(listener, app).await.unwrap();
}
