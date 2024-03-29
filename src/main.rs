use crate::collector::HttpServer;
use crate::collector::hangar::{HangarClient, HangarServer};
use crate::collector::modrinth::{ModrinthClient, ModrinthServer};
use crate::collector::spigot::{SpigotClient, SpigotServer};
use crate::database::Database;
use crate::database::common::project::{get_merged_common_projects, upsert_common_projects};
use crate::database::hangar::project::get_latest_hangar_project_update_date;
use crate::database::modrinth::project::get_latest_modrinth_project_update_date;
use crate::database::spigot::author::get_highest_spigot_author_id;
use crate::database::spigot::resource::get_latest_spigot_resource_update_date;

use anyhow::Result;

use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

mod collector;
mod database;

const LIVE_DB_NAME: &str = "mc_plugin_finder";

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let subscriber = tracing_subscriber::fmt()
        // .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Initialize database client
    let db = get_db();
    let db_pool = db.create_pool(LIVE_DB_NAME).await?;

    // Initialize API clients
    let spigot_server = SpigotServer::new().await;
    let spigot_client = SpigotClient::new(spigot_server)?;

    let modrinth_server = ModrinthServer::new().await;
    let modrinth_client = ModrinthClient::new(modrinth_server)?;

    let hangar_server = HangarServer::new().await;
    let hangar_client = HangarClient::new(hangar_server)?;

    /* Spigot */

    /* Populate Spigot Authors */
    // spigot_client.populate_spigot_authors(&db_pool).await?;

    /* Update Spigot Authors */
    // let highest_author_id = get_highest_spigot_author_id(&db_pool).await?;
    // info!("Highest id: {:?}", highest_author_id);
    // spigot_client.update_spigot_authors(&db_pool, highest_author_id).await?;

    /* Populate Spigot Resources (without versions) */
    // spigot_client.populate_spigot_resources(&db_pool).await?;

    /* Populate Spigot Resource Versions */
    // spigot_client.populate_spigot_resource_versions(&db_pool).await?;

    /* Update Spigot Resources */
    // let latest_spigot_resource_update_date = get_latest_spigot_resource_update_date(&db_pool).await?;
    // info!("Latest update date: {:?}", latest_spigot_resource_update_date);
    // spigot_client.update_spigot_resources(&db_pool, latest_spigot_resource_update_date).await?;

    /* Modrinth */

    /* Populate Modrinth Projects (without versions) */
    // modrinth_client.populate_modrinth_projects(&db_pool).await?;

    /* Populate Modrinth Project Versions */
    // modrinth_client.populate_modrinth_project_versions(&db_pool).await?;

    /* Update Modrinth Projects */
    // let latest_modrinth_project_update_date = get_latest_modrinth_project_update_date(&db_pool).await?;
    // info!("Latest update date: {:?}", latest_modrinth_project_update_date);
    // modrinth_client.update_modrinth_projects(&db_pool, latest_modrinth_project_update_date).await?;

    /* Hangar */

    /* Populate Hangar Projects (without versions) */
    // hangar_client.populate_hangar_projects(&db_pool).await?;

    /* Populate Hangar Project Versions */
    // hangar_client.populate_hangar_project_versions(&db_pool).await?;

    /* Update Hangar Projects */
    // let latest_hangar_project_update_date = get_latest_hangar_project_update_date(&db_pool).await?;
    // info!("Latest update date: {:?}|", latest_hangar_project_update_date);
    // hangar_client.update_hangar_projects(&db_pool, latest_hangar_project_update_date).await?;

    /* Common */

    /* Merge Common Projects */
    // let common_projects = get_merged_common_projects(&db_pool, None).await?;
    // upsert_common_projects(&db_pool, &common_projects).await?;

    Ok(())
}

fn get_db() -> Database {
    Database {
        user: "postgres".to_string(),
        password: "postgres".to_string(),
        host: "localhost".to_string(),
        port: 5433
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use deadpool_postgres::Pool;
    use std::fs::read_to_string;

    const DEFAULT_POSTGRES_DB_NAME: &str = "postgres";

    pub struct DatabaseTestContext {
        db_name: String,
        base_pool: Pool,
        pub pool: Pool
    }

    impl DatabaseTestContext {
        pub async fn new(db_name: &str) -> Self {
            let db = get_db();

            let base_pool = db.create_pool(DEFAULT_POSTGRES_DB_NAME)
                .await
                .expect("could not create database pool");

            Self::drop_database(&base_pool, db_name)
                .await
                .expect("could not drop database before re-creating it");
            Self::create_database(&base_pool, db_name)
                .await
                .expect("could not create database");

            let pool = db.create_pool(db_name)
                .await
                .expect("could not create database pool");

            Self::run_migration(&pool)
                .await
                .expect("could not run migration");

            Self {
                db_name: db_name.to_string(),
                base_pool,
                pool
            }
        }

        async fn create_database(pool: &Pool, db_name: &str) -> Result<()> {
            let client = pool.get().await?;

            let statement = format!("CREATE DATABASE {};", db_name);
            client.execute(&statement, &[]).await?;

            Ok(())
        }

        async fn run_migration(pool: &Pool) -> Result<()> {
            let client = pool.get().await?;

            let schema_text = read_to_string("schema.sql")?
               .parse::<String>()?;

            client.batch_execute(schema_text.as_str()).await?;

            Ok(())
        }

        async fn drop_database(pool: &Pool, db_name: &str) -> Result<()> {
            let client = pool.get().await?;

            let disconnect_users_statement =
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1;";
            client.execute(disconnect_users_statement, &[&db_name]).await?;

            let drop_database_statement = format!("DROP DATABASE IF EXISTS {};", db_name);
            client.execute(&drop_database_statement, &[]).await?;

            Ok(())
        }

        // TODO: When async drop trait is implemented in Rust, use that instead
        pub async fn drop(&self) -> Result<()> {
            Self::drop_database(&self.base_pool, &self.db_name).await?;

            Ok(())
        }
    }
}