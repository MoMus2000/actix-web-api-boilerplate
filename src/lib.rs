pub mod configuration;

use std::net::TcpListener;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use sqlx::PgConnection;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(tcp_listener : TcpListener, db_connection : PgConnection) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_connection);
    let server = HttpServer::new(move || {
    App::new()
        .route("/health_check", web::get().to(health_check))
        .app_data(connection.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}