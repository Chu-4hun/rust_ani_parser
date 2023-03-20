use dotenv::dotenv;
use parsers::anixart_parser::*;
use sqlx::{postgres::PgPoolOptions};

pub mod models;
pub mod parser;
pub mod parsers;
pub mod repo;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

   print!("{} new episodes",parse_anixart( 2128, &pool).await);

    Ok(())
}
