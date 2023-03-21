use dotenv::dotenv;
use parsers::anixart_parser::*;
use sqlx::postgres::PgPoolOptions;

pub mod models;
pub mod parser;
pub mod parsers;
pub mod repo;

use std::{error::Error, io, str::FromStr};

fn read_values<T: FromStr>() -> Result<Vec<T>, T::Err> {
    println!("enter releases IDs with whitespaces");
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("could not read from stdin");
    s.trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let numbers: Vec<i32> = read_values::<i32>().unwrap_or(vec![3050]);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    for num in 0..numbers[0] {
        match parse_anixart(num, &pool).await {
            Ok(count) => println!("{} new episodes", count),
            Err(e) => {
                println!("An error: {}; skipped.", e);
                continue;
            }
        }
    }

    Ok(())
}
