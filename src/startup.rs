use crate::routes::health_check;
use crate::routes::subscribe;
use axum::routing::post;
use axum::{Router, routing::get};
use sqlx::PgPool;

pub async fn run(listener: tokio::net::TcpListener, db_pool: PgPool) {
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(db_pool);

    axum::serve(listener, app).await.unwrap();
}
