use crate::collector::HttpServer;
use crate::collector::spigot::{SpigotClient, SpigotServer};
use crate::database::Database;

use anyhow::Result;

mod collector;
mod cornucopia;
mod database;

#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::new(
        "postgres".to_string(),
        "postgres".to_string(),
        "127.0.0.1".to_string(),
        5432,
        "postgres".to_string()
    );

    let db_pool = db.create_pool().await?;
    let db_client = db_pool.get().await?;
    let spigot_server = SpigotServer::new().await;

    let spigot_client = SpigotClient::new(spigot_server)?;

    let count = spigot_client.populate_spigot_authors(&db_client).await?;

    // let count = spigot_client.update_spigot_authors(&db_client).await?;

    // let count = spigot_client.populate_spigot_resources(&db_client).await?;

    // let count = spigot_client.update_spigot_resources(&db_client).await?;

    println!("Items added: {:?}", count);

    Ok(())
}