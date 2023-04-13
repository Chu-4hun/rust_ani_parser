use dotenv::dotenv;
use parsers::anixart_parser::*;
use sqlx::postgres::PgPoolOptions;

pub mod models;
pub mod parser;
pub mod parsers;
pub mod repo;

use std::{error::Error, env};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let count_from:u32 =  args[1].parse::<u32>().unwrap_or(0);
    let count_to:u32 =  args[2].parse::<u32>().unwrap_or(0);
    dbg!(args);
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    for num in count_from..count_to {
        match parse_anixart(num.try_into().unwrap_or(0), &pool).await {
            Ok(count) => println!("{} new episodes", count),
            Err(e) => {
                println!("An error: {}; skipped.", e);
                continue;
            }
        }
    }

    Ok(())
}
