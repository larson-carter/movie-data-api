mod handlers;
mod models;
mod cache;

use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;
use crate::cache::QueryCache;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let cache = QueryCache::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(cache.clone()))
            .route("/search", web::get().to(handlers::search_movies))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
