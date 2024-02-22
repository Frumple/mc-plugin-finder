use crate::collector::HttpServer;
use crate::collector::spigot::{SpigotClient, SpigotServer};
use crate::database::Database;

use anyhow::Result;

mod collector;
mod database;

const LIVE_DB_NAME: &str = "mc_plugin_finder";

#[tokio::main]
async fn main() -> Result<()> {
    let db = get_db();
    let db_pool = db.create_pool(LIVE_DB_NAME).await?;
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

fn get_db() -> Database {
    Database {
        user: "postgres".to_string(),
        password: "postgres".to_string(),
        host: "localhost".to_string(),
        port: 5432
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use deadpool_postgres::Client;
    use std::fs::read_to_string;

    const DEFAULT_POSTGRES_DB_NAME: &str = "postgres";

    pub struct DatabaseTestContext {
        // db: Database,
        db_name: String,
        base_client: Client,
        pub client: Client
    }

    impl DatabaseTestContext {
        pub async fn new(db_name: &str) -> Self {
            let db = get_db();

            // TODO: Have all tests use a single common base pool to the default postgres database
            let base_pool = db.create_pool(DEFAULT_POSTGRES_DB_NAME)
                .await
                .expect("could not create database pool");
            let base_client = base_pool.get()
                .await
                .expect("could not get database client");

            Self::drop_database(&base_client, db_name)
                .await
                .expect("could not drop database before re-creating it");
            Self::create_database(&base_client, db_name)
                .await
                .expect("could not create database");

            let new_pool = db.create_pool(db_name)
                .await
                .expect("could not create database pool");
            let new_client = new_pool.get()
                .await
                .expect("could not get database client");

            Self::run_migration(&new_client)
                .await
                .expect("could not run migration");

            Self {
                // db,
                db_name: db_name.to_string(),
                base_client,
                client: new_client
            }
        }

        async fn create_database(client: &Client, db_name: &str) -> Result<()> {
            let statement = format!("CREATE DATABASE {};", db_name);
            client.execute(&statement, &[]).await?;

            Ok(())
        }

        async fn run_migration(client: &Client) -> Result<()> {
            let schema_text = read_to_string("schema.sql")?
               .parse::<String>()?;

            for statement in schema_text.split(';') {
                client.execute(statement, &[]).await?;
            }

            Ok(())
        }

        async fn drop_database(client: &Client, db_name: &str) -> Result<()> {
            let disconnect_users_statement =
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1;";
            client.execute(disconnect_users_statement, &[&db_name]).await?;

            let drop_database_statement = format!("DROP DATABASE IF EXISTS {};", db_name);
            client.execute(&drop_database_statement, &[]).await?;

            Ok(())
        }

        // TODO: When async drop trait is implemented in Rust, use that instead
        pub async fn drop(&self) -> Result<()> {
            Self::drop_database(&self.base_client, &self.db_name).await?;

            Ok(())
        }
    }
}