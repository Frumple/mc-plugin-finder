use crate::collector::spigot::SpigotClient;

use anyhow::Result;
use deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use tokio_postgres::NoTls;

mod collector;
mod cornucopia;

#[tokio::main]
async fn main() -> Result<()> {
    let db_pool = create_db_pool().await?;
    let db_client = db_pool.get().await?;

    let spigot_client = SpigotClient::new(db_client)?;
    let population_result = spigot_client.populate_all_spigot_authors().await;
    println!("Authors populated: {:?}", population_result.count);
    if population_result.error.is_some() {
        println!("Error: {:#}", population_result.error.unwrap());
    }

    Ok(())
}

async fn create_db_pool() -> Result<Pool, CreatePoolError> {
    let mut config = Config::new();
    config.user = Some(String::from("postgres"));
    config.password = Some(String::from("postgres"));
    config.host = Some(String::from("127.0.0.1"));
    config.port = Some(5435);
    config.dbname = Some(String::from("postgres"));
    config.create_pool(Some(Runtime::Tokio1), NoTls)
}