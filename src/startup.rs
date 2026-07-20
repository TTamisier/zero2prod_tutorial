use crate::email_client::EmailClient;
use crate::routes::health_check;
use crate::routes::subscribe;
use axum::routing::post;
use axum::{Router, routing::get};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub email_client: EmailClient,
}

pub async fn run(listener: tokio::net::TcpListener, db_pool: PgPool, email_client: EmailClient) {
    tracing::info!("Starting server on {}", listener.local_addr().unwrap());
    let state = AppState {
        db_pool,
        email_client,
    };

    let app: Router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}
