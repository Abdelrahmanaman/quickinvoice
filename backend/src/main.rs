mod auth;
mod config;
mod db;
mod handlers;
mod models;
mod schema;
use std::env;

use crate::db::establish_connection;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::middleware::validator_middleware;
use dotenv::dotenv;
use redis::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    println!("🚀 Server started successfully");

    let db_pool = establish_connection();
    // Initialize the Redis client
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_client = Client::open(redis_url).expect("Failed to connect to Redis");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator_middleware);
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .service(auth::handlers::register)
            .service(auth::handlers::login)
            .service(auth::handlers::logout)
            .service(handlers::hello::hello)
            .service(handlers::webhook::handle_webhook)
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .service(auth::handlers::refresh_token)
                    .service(auth::handlers::api_hello),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
