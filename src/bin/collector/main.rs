use crate::hangar::{HangarClient, HangarServer};
use crate::modrinth::{ModrinthClient, ModrinthServer};
use crate::spigot::{SpigotClient, SpigotServer};

use mc_plugin_finder::database::get_db;
use mc_plugin_finder::database::common::project::{get_merged_common_projects, upsert_common_projects};
use mc_plugin_finder::database::hangar::project::get_latest_hangar_project_update_date;
use mc_plugin_finder::database::modrinth::project::get_latest_modrinth_project_update_date;
use mc_plugin_finder::database::spigot::author::get_highest_spigot_author_id;
use mc_plugin_finder::database::spigot::resource::get_latest_spigot_resource_update_date;

use anyhow::Result;

use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use url::Url;

pub mod hangar;
pub mod modrinth;
pub mod spigot;
pub mod util;

const LIVE_DB_NAME: &str = "mc_plugin_finder";

pub trait HttpServer {
  async fn new() -> Self;
  fn base_url(&self) -> Url;
}

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

    info!("hi");

    Ok(())
}