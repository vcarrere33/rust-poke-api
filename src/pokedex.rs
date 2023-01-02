struct Pokemon;
use crate::response::Response;
use actix_web::{get, HttpResponse, Responder};

pub type Pokedex = Response<Pokemon>;

/// list 50 last pokemons `/pokedex`
#[get("/pokedex")]
pub fn list() -> HttpResponse {}
