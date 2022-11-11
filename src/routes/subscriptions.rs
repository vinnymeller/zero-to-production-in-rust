use actix_web::web;
use actix_web::HttpResponse;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct SubscriberFormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<SubscriberFormData>,
    connection: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
        );
    let _request_span_guard = request_span.enter();
    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.name,
        form.email
    );
    let query_span = tracing::info_span!("Saving new subscriber details in the database.");
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES
        ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        &form.email,
        &form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved.",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
