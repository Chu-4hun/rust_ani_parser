use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct Dub {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
}