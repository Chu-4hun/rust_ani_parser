use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Episode {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub release_fk: i32,
    pub dub_fk: i32,
    pub ep_name: String,
    pub url: String,
    pub position: i32
}