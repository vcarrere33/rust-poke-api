mod hello;
mod models;
mod pokedex;
mod response;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(hello::hello)
            .service(pokedex::list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
