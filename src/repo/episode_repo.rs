use sqlx::{Postgres, Pool};

use crate::models::episode::Episode;

impl Episode{
    pub async fn insert(&self, db: &Pool<Postgres>) -> Result<Episode, sqlx::Error> {
        sqlx::query_as::<_, Episode>(
            "
        INSERT INTO episode (release_fk, ep_name, url, dub_fk, position)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *;
        ",
        )
        .bind(&self.release_fk)
        .bind(&self.ep_name)
        .bind(&self.url)
        .bind(&self.dub_fk)
        .bind(&self.position)

        .fetch_one(db)
        .await
    }
    
    pub async fn get_by_id(id: i32, db: &Pool<Postgres>) -> Result<Episode, sqlx::Error> {
        sqlx::query_as::<_, Episode>(
            "
        SELECT *
        FROM episode
        WHERE id = $1;
        ",
        )
        .bind(id)
        .fetch_one(db)
        .await
    }
}