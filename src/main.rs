use crate::collector::HttpServer;
use crate::collector::spigot::{SpigotClient, SpigotServer};
use crate::database::{DatabaseCredentials, DatabaseManager};

use anyhow::Result;

mod collector;
mod database;

#[tokio::main]
async fn main() -> Result<()> {
    let db_credentials = DatabaseCredentials {
        user: "postgres".to_string(),
        password: "postgres".to_string(),
        host: "localhost".to_string(),
        port: 5432,
        dbname: "mc_plugin_finder".to_string()
    };

    let db_manager = DatabaseManager::new(db_credentials).await?;
    let db_client = db_manager.get_client().await?;
    let spigot_server = SpigotServer::new().await;

    let spigot_client = SpigotClient::new(spigot_server)?;

    // let count = spigot_client.populate_spigot_authors(&db_client).await?;

    // let count = spigot_client.update_spigot_authors(&db_client).await?;

    let count = spigot_client.populate_spigot_resources(&db_client).await?;

    // let count = spigot_client.update_spigot_resources(&db_client).await?;

    println!("Items added: {:?}", count);

    Ok(())
}