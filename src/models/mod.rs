use mongodb::bson;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Pokemon {
    #[serde(rename = "_id")]
    id: bson::oid::ObjectId,
    pub name: Name,
    #[serde(rename = "type")]
    pub type_: Vec<String>,
    pub base: Base,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Name {
    english: String,
    japanese: String,
    chinese: String,
    french: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Base {
    hp: i32,
    attack: i32,
    defense: i32,
    #[serde(rename = "Sp. Attack")]
    spattack: i32,
    #[serde(rename = "Sp. Defense")]
    spdefense: i32,
    speed: i32,
}
