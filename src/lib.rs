pub mod configuration;
use std::net::TcpListener;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use sqlx::*;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn fetch_from_db(connection: web::Data<PgPool>) -> HttpResponse{
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        "mustafa",
        "mustafa",
        "muhammad",
        "mustafa",
    ).execute(connection.as_ref())
    .await{
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn run(tcp_listener : TcpListener, db_connection : PgPool) -> Result<Server> {
    let connection = web::Data::new(db_connection);
    let server = HttpServer::new(move || {
    App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/fetch", web::get().to(fetch_from_db))
        .app_data(connection.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}