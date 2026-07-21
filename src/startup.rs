use crate::configuration::Settings;
use crate::email_client::EmailClient;
use crate::routes::health_check;
use crate::routes::subscribe;
use axum::routing::post;
use axum::serve::Serve;
use axum::{Router, routing::get};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub email_client: EmailClient,
}

pub struct Application {
    port: u16,
    server: Serve<tokio::net::TcpListener, Router, Router>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        // Create database connection pool
        let connection_pool = get_connection_pool(&configuration.database).await;

        // Create email client
        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address");

        // Get email client timeout
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );

        // Start the server
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, email_client);
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn get_connection_pool(config: &crate::configuration::DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.with_db())
}

pub fn run(
    listener: tokio::net::TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Serve<tokio::net::TcpListener, Router, Router> {
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

    axum::serve(listener, app)
}
