use email_service::run;
use email_service::configuration;
use sqlx::{Connection, PgConnection};
use std::net::*;

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let config = configuration::get_configuration().unwrap();
    println!("Configuration : {:?}", config);
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string()).await.unwrap();
    let server = run(listener, connection).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
} 