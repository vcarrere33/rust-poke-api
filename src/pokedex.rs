use crate::models::Pokemon;
use actix_web::{get, web, HttpResponse};
use futures::stream::StreamExt;
use mongodb::{Client, Collection};

const DB_NAME: &str = "rust-poke-api";
const COLL_NAME: &str = "pokemons";

/// list 50 last pokemons `/pokedex`
#[get("/pokedex")]
pub async fn list(client: web::Data<Client>) -> HttpResponse {
    let logs_collection: Collection<Pokemon> = client.database(DB_NAME).collection(COLL_NAME);

    let mut cursor = logs_collection.find(None, None).await.unwrap();

    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            Err(msg) => {
                return HttpResponse::InternalServerError().body(msg.to_string());
            }
        }
    }
    HttpResponse::Ok().json(results)
}
