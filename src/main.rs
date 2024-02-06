use crate::collector::HttpServer;
use crate::collector::spigot::{SpigotClient, SpigotServer};

use anyhow::Result;
use deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use tokio_postgres::NoTls;

mod collector;
mod cornucopia;

#[tokio::main]
async fn main() -> Result<()> {
    let db_pool = create_db_pool().await?;
    let db_client = db_pool.get().await?;
    let spigot_server = SpigotServer::new().await;

    let spigot_client = SpigotClient::new(spigot_server)?;

    let count = spigot_client.populate_spigot_authors(&db_client).await?;

    // let count = spigot_client.update_spigot_authors(&db_client).await?;

    // let count = spigot_client.populate_spigot_resources(&db_client).await?;

    println!("Items added: {:?}", count);

    Ok(())
}

async fn create_db_pool() -> Result<Pool, CreatePoolError> {
    let mut config = Config::new();
    config.user = Some(String::from("postgres"));
    config.password = Some(String::from("postgres"));
    config.host = Some(String::from("127.0.0.1"));
    config.port = Some(5432);
    config.dbname = Some(String::from("postgres"));
    config.create_pool(Some(Runtime::Tokio1), NoTls)
}