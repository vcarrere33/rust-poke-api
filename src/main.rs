mod hello;
mod pokedex;
struct Pokemon;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use mongodb::{bson::doc, options::IndexOptions, Client, IndexModel};

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "pokemon";

async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<Pokemon>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    create_username_index(&client).await;

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
