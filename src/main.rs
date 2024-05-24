use actix_web::*;
use std::net::*;

use email_service::run;

async fn hello_world(req: HttpRequest) -> impl Responder{
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", name)
}

async fn hello_world_post(req: HttpRequest) -> impl Responder{
    let name = req.match_info().get("name").unwrap_or("World");
    HttpResponse::new(http::StatusCode::CREATED)
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind port 8000");
    run(listener)?.await
}