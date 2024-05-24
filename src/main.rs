use sqlx::PgPool;
use std::net::*;

use email_service::{configuration, run};


#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind port 8000");
    let config = configuration::get_configuration().unwrap();
    println!("Configuration : {:?}", config);
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string())
    .await
    .expect("Failed to connect to Postgres.");
    run(listener, connection).unwrap().await
}