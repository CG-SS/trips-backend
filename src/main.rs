use actix_web::{App, HttpServer};

mod routes;
use crate::routes::get::greet;

use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

mod models;
use crate::models::*;
use diesel::prelude::*;
mod schema;
use crate::schema::peoples::dsl::peoples;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = &mut SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    let results = peoples
        .limit(5)
        .load::<Person>(conn)
        .expect("Error loading peoples");

    for person in results {
        println!("{}", person.id);
        println!("-----------\n");
        println!("{}", person.name);
    }

    HttpServer::new(|| {
        App::new()
            .service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}