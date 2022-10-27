use actix_web::web;
use actix_web::HttpResponse;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscriberFormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<SubscriberFormData>,
    connection: web::Data<PgPool>,
) -> HttpResponse {
    let res = sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES 
        ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        &form.email,
        &form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await;
    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
