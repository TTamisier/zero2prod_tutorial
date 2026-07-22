use crate::routes::get_subscriber_id_from_token;
use crate::startup::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct UnsubscribeParameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Delete a confirmed subscriber", skip(parameters, state))]
pub async fn unsubscribe(
    parameters: Query<UnsubscribeParameters>,
    State(state): State<AppState>,
) -> StatusCode {
    let pool = &state.db_pool;
    let id = match get_subscriber_id_from_token(pool, &parameters.subscription_token).await {
        Ok(id) => id,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
    match id {
        None => return StatusCode::UNAUTHORIZED,
        Some(subscriber_id) => {
            if delete_subscriber(pool, subscriber_id).await.is_err() {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            return StatusCode::OK;
        }
    }
}

pub async fn delete_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM subscription_token WHERE subscriber_id = $1",
        subscriber_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    sqlx::query!("DELETE FROM subscriptions WHERE id = $1", subscriber_id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}
