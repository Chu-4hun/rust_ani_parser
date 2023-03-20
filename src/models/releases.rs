use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::episode::Episode;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Release {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub release_type: ReleaseType,
    pub release_name: String,
    pub release_date: Option<DateTime<Utc>>,
    pub rating: f32,
    pub min_age: i32,
    pub director: String,
    pub author: String,
    pub studio: String,
    pub description: String,
    pub img: String,
    pub external_id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, sqlx::Type, Copy)]
#[repr(i32)]
pub enum ReleaseType {
    Cinema,
    Series,
    Animation,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ReleaseWithEpisodes {
    pub release: Release,
    pub episodes: Vec<Episode>,
}
