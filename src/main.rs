use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use std::collections::HashMap;

mod models;
mod handlers;
mod routes;

use models::DB;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: DB = std::sync::Arc::new(std::sync::Mutex::new(HashMap::new()));

    println!("Server running on http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()         
            .allow_any_origin()         
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
