use sqlx::{Postgres, Pool};

use crate::models::dub::Dub;

impl Dub{
    pub async fn insert(&self, db: &Pool<Postgres>) -> Result<Dub, sqlx::Error> {
        sqlx::query_as::<_, Dub>(
            "
        INSERT INTO dub (name)
        VALUES ($1)
        RETURNING *;
        ",
        )
        .bind(&self.name)
        .fetch_one(db)
        .await
    }
    pub async fn insert_by_name(name:&str, db: &Pool<Postgres>) -> Result<Dub, sqlx::Error> {
        sqlx::query_as::<_, Dub>(
            "
        INSERT INTO dub (name)
        VALUES ($1)
        RETURNING *;
        ",
        )
        .bind(name)
        .fetch_one(db)
        .await
    }
    
    pub async fn get_by_id(id: i32, db: &Pool<Postgres>) -> Result<Dub, sqlx::Error> {
        sqlx::query_as::<_, Dub>(
            "
        SELECT *
        FROM dub
        WHERE id = $1;
        ",
        )
        .bind(id)
        .fetch_one(db)
        .await
    }

     pub async fn get_if_exist(name: &str, db: &Pool<Postgres>) -> Result<Dub, sqlx::Error> {
        let res = sqlx::query_as::<_, Dub>(
            "
        SELECT *
        FROM dub
        WHERE name = $1;
        ",
        )
        .bind(name)
        .fetch_one(db)
        .await?;
        Ok(res)
    }
}