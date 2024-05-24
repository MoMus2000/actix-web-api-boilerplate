use sqlx::{PgConnection, Connection};
use email_service::configuration::get_configuration;
mod utils;

#[actix_rt::test]
async fn check_db_connection(){
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    eprintln!("Connection String : {}", connection_string);

    let _ = PgConnection::connect(&connection_string)
    .await
    .expect("Failed to connect to Postgres.");
}