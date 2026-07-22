use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn unsubscribe_returns_200_if_user_is_subscribed() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/messages"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Ajoute le user avec le statut confirmation pending
    app.post_subscriptions(body.into()).await;
    // Interception de la requête mail
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    // Extraction du lien de confirmation d'abonnement
    let confirmation_links = app.get_confirmation_links(email_request);

    // add a user to the database with confirmed status
    // le confirmation link c'est le lien genre `http://127.0.0.1:{port}/subscriptions/confirm?subscription_token=XXXXXX`
    // Donc faire un requête GET dessus va taper notre endpoint de confirmation
    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let token = sqlx::query!(
        "SELECT subscription_token FROM subscription_token \
        JOIN subscriptions ON subscription_token.subscriber_id = subscriptions.id \
        WHERE subscriptions.email = $1",
        "ursula_le_guin@gmail.com"
    )
    .fetch_one(&app.db_pool)
    .await
    .unwrap();

    // Act
    reqwest::get(&format!(
        "{}/subscriptions/unsubscribe?subscription_token={}",
        app.address, token.subscription_token
    ))
    .await
    .unwrap();

    // Assert
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_optional(&app.db_pool)
        .await
        .expect("Failed to query database");

    assert!(saved.is_none());
}
