use sqlx::{Pool, Postgres};

use serde::{Deserialize, Serialize};

use crate::models::{
    dub::Dub,
    episode::Episode,
    releases::{Release, ReleaseType},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnixartRelease {
    pub code: i32,
    pub types: Vec<DubVariant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DubVariant {
    #[serde(rename = "@id")]
    pub id: i32,
    #[serde(rename = "id")]
    pub type_id: i32,
    pub name: String,
    pub workers: Option<String>,
    pub episodes_count: i32,
    pub view_count: i32,
}

pub async fn parse_anixart(
    release_id: i32,
    db: &Pool<Postgres>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let ani_release = get_release(release_id)
        .await
        .expect("Get releases function broke");

    let mut release = get_release_by_id(release_id).await?;
    if release.is_unique(&db).await.unwrap() {
        release = release.insert(&db).await.unwrap()
    } else {
        Err("Release already exist")?
    }

    let mut i: usize = 0;
    for dub_variant in ani_release.types {
        let dub_id: i32 = add_dubs_to_db(&dub_variant, &db).await.unwrap().id;
        for source_id in get_kodik_sources(release_id, dub_variant.type_id)
            .await
            .expect("get_kodik_sources failed")
        {
            for mut episode in get_episodes(release_id, dub_variant.type_id, source_id)
                .await
                .expect("gg")
            {
                episode.release_fk = release.id.unwrap();
                episode.dub_fk = dub_id;
                episode.insert(&db).await.unwrap();
                i += 1;
            }
        }
    }
    Ok(i)
}

pub async fn get_release(id: i32) -> Result<AnixartRelease, Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!("https://api.anixart.tv/episode/{}", id))
        .await?
        .json::<AnixartRelease>()
        .await?;
    Ok(resp)
}

pub async fn add_dubs_to_db(
    input: &DubVariant,
    db: &Pool<Postgres>,
) -> Result<Dub, Box<dyn std::error::Error>> {
    match Dub::get_if_exist(input.name.as_str(), &db).await {
        Ok(val) => Ok(val),
        Err(_) => Ok(Dub::insert_by_name(input.name.as_str(), &db).await?),
    }
}

pub async fn get_release_by_id(id: i32) -> Result<Release, Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!("https://api.anixart.tv/release/{}", id))
        .await?
        .text()
        .await?;
    let _json: serde_json::Value =
        serde_json::from_str(resp.as_str()).expect("JSON was not well-formatted");

    if _json["release"].is_null() {
        return Err("empty release")?;
    }
    let release = Release {
        id: None,
        release_type: ReleaseType::Animation,
        release_name: _json["release"]["title_ru"]
            .to_string()
            .replace("/", "")
            .replace('"', ""),
        release_date: None,
        rating: 0.0,
        min_age: _json["release"]["age_rating"]
            .as_i64()
            .unwrap_or(0)
            .try_into()
            .unwrap_or(0),
        director: _json["release"]["director"]
            .to_string()
            .replace("/", "")
            .replace('"', ""),
        author: _json["release"]["author"]
            .to_string()
            .replace("/", "")
            .replace('"', ""),
        studio: _json["release"]["studio"]
            .to_string()
            .replace("/", "")
            .replace('"', ""),
        description: _json["release"]["description"]
            .to_string()
            .replace("/", "")
            .replace('"', ""),
        img: _json["release"]["image"].to_string().replace('"', ""),
        external_id: id.to_string(),
    };
    Ok(release)
}
fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
pub async fn get_kodik_sources(
    release_id: i32,
    id: i32,
) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!(
        "https://api.anixart.tv/episode/{}/{}",
        release_id, id
    ))
    .await?
    .text()
    .await?;
    let mut ids: Vec<i64> = vec![];
    let json: serde_json::Value =
        serde_json::from_str(resp.as_str()).expect("JSON was not well-formatted");

    for s in json["sources"].as_array().unwrap() {
        if s["name"] == "Kodik" {
            ids.push(s["id"].as_i64().unwrap());
            print!("{} - {} - {}\n", s["id"], s["name"], s["type"]["name"])
        }
    }
    Ok(ids)
}
pub async fn get_episodes(
    release_id: i32,
    id: i32,
    source_id: i64,
) -> Result<Vec<Episode>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!(
        "https://api.anixart.tv/episode/{}/{}/{}",
        release_id, id, source_id
    ))
    .await?
    .text()
    .await?;
    let mut episodes: Vec<Episode> = vec![];
    let json: serde_json::Value =
        serde_json::from_str(resp.as_str()).expect("JSON was not well-formatted");

    for s in json["episodes"].as_array().unwrap() {
        episodes.push(Episode {
            release_fk: release_id,
            ep_name: s["name"].to_string().replace("/", "").replace('"', ""),
            url: rem_first_and_last(s["url"].as_str().unwrap_or("default"))
                .to_string()
                .replace('"', ""),
            id: None,
            dub_fk: id,
            position: s["position"].as_i64().unwrap().try_into().unwrap_or(0),
        });
        // println!("{}", s["name"]);
    }

    Ok(episodes)
}
