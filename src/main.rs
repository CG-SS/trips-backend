use actix_web::{App, HttpServer};

mod routes;
use crate::routes::get::greet;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}