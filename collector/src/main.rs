use crate::hangar::{HangarClient, HangarServer};
use crate::modrinth::{ModrinthClient, ModrinthServer};
use crate::spigot::{SpigotClient, SpigotServer};

use mc_plugin_finder::database::get_db;
use mc_plugin_finder::database::common::project::refresh_common_projects;
use mc_plugin_finder::database::hangar::project::get_latest_hangar_project_update_date;
use mc_plugin_finder::database::modrinth::project::get_latest_modrinth_project_update_date;
use mc_plugin_finder::database::spigot::author::get_highest_spigot_author_id;
use mc_plugin_finder::database::spigot::resource::get_latest_spigot_resource_update_date;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use url::Url;

pub mod hangar;
pub mod modrinth;
pub mod spigot;

const LIVE_DB_NAME: &str = "mc_plugin_finder";

pub trait HttpServer {
    #[allow(async_fn_in_trait)]
    async fn new() -> Self;
    fn base_url(&self) -> Url;
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CommandLineArguments {
    #[command(subcommand)]
    action: ActionSubcommand
}

#[derive(Subcommand)]
enum ActionSubcommand {
    /// Get all items: Run this initially to populate an empty database
    Populate {
        #[command(subcommand)]
        repository: PopulateRepositorySubcommand
    },
    /// Get only items since the last populate/update: Run this periodically to keep the database up-to-date
    Update {
        #[command(subcommand)]
        repository: UpdateRepositorySubcommand
    },
    /// Refresh common projects: Run this after populating/updating all repositories
    Refresh
}

#[derive(Subcommand)]
enum PopulateRepositorySubcommand {
    /// Spigot authors, resources, or versions
    Spigot {
        #[arg(value_enum)]
        item: PopulateSpigotItems
    },
    /// Modrinth projects or versions
    Modrinth {
        #[arg(value_enum)]
        item: PopulateModrinthItems
    },
    /// Hangar projects or versions
    Hangar {
        #[arg(value_enum)]
        item: PopulateHangarItems
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PopulateSpigotItems {
    Authors,
    Resources,
    Versions
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PopulateModrinthItems {
    Projects,
    Versions
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PopulateHangarItems {
    Projects,
    Versions
}

#[derive(Subcommand)]
enum UpdateRepositorySubcommand {
    /// Spigot authors or resources
    Spigot {
        #[arg(value_enum)]
        item: UpdateSpigotItems
    },
    /// Modrinth projects
    Modrinth {
        #[arg(value_enum)]
        item: UpdateModrinthItems
    },
    /// Hangar projects
    Hangar {
        #[arg(value_enum)]
        item: UpdateHangarItems
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum UpdateSpigotItems {
    Authors,
    Resources
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum UpdateModrinthItems {
    Projects
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum UpdateHangarItems {
    Projects
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = CommandLineArguments::parse();

    // Initialize tracing
    let subscriber = tracing_subscriber::fmt()
        // .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Initialize database client
    let db = get_db();
    let db_pool = db.create_pool(LIVE_DB_NAME).await?;

    match &cli.action {
        ActionSubcommand::Populate { repository } => {
            match repository {
                PopulateRepositorySubcommand::Spigot { item } => {
                    let spigot_server = SpigotServer::new().await;
                    let spigot_client = SpigotClient::new(spigot_server)?;

                    match item {
                        PopulateSpigotItems::Authors => {
                            info!("Populating Spigot Authors...");
                            spigot_client.populate_spigot_authors(&db_pool).await?;
                        },
                        PopulateSpigotItems::Resources => {
                            info!("Populating Spigot Resources...");
                            spigot_client.populate_spigot_resources(&db_pool).await?;
                        },
                        PopulateSpigotItems::Versions => {
                            info!("Populating Spigot Versions...");
                            spigot_client.populate_spigot_versions(&db_pool).await?;
                        }
                    }
                },
                PopulateRepositorySubcommand::Modrinth { item } => {
                    let modrinth_server = ModrinthServer::new().await;
                    let modrinth_client = ModrinthClient::new(modrinth_server)?;

                    match item {
                         PopulateModrinthItems::Projects => {
                            info!("Populating Modrinth Projects...");
                            modrinth_client.populate_modrinth_projects(&db_pool).await?;
                        },
                        PopulateModrinthItems::Versions => {
                            info!("Populating Modrinth Versions...");
                            modrinth_client.populate_modrinth_versions(&db_pool).await?;
                        }
                    }
                },
                PopulateRepositorySubcommand::Hangar { item } => {
                    let hangar_server = HangarServer::new().await;
                    let hangar_client = HangarClient::new(hangar_server)?;

                    match item {
                        PopulateHangarItems::Projects => {
                           info!("Populating Hangar Projects...");
                           hangar_client.populate_hangar_projects(&db_pool).await?;
                       },
                       PopulateHangarItems::Versions => {
                           info!("Populating Hangar Versions...");
                           hangar_client.populate_hangar_versions(&db_pool).await?;
                       }
                   }
                },
            }
        },
        ActionSubcommand::Update { repository } => {
            match repository {
                UpdateRepositorySubcommand::Spigot { item } => {
                    let spigot_server = SpigotServer::new().await;
                    let spigot_client = SpigotClient::new(spigot_server)?;

                    match item {
                        UpdateSpigotItems::Authors => {
                            let highest_author_id = get_highest_spigot_author_id(&db_pool).await?;
                            info!("Updating Spigot Authors with ID higher than: {}", highest_author_id);
                            spigot_client.update_spigot_authors(&db_pool, highest_author_id).await?;
                        },
                        UpdateSpigotItems::Resources => {
                            let latest_update_date = get_latest_spigot_resource_update_date(&db_pool).await?;
                            info!("Updating Spigot Resources since: {}", latest_update_date);
                            spigot_client.update_spigot_resources(&db_pool, latest_update_date).await?;
                        }
                    }
                },
                UpdateRepositorySubcommand::Modrinth { item } => {
                    let modrinth_server = ModrinthServer::new().await;
                    let modrinth_client = ModrinthClient::new(modrinth_server)?;

                    match item {
                        UpdateModrinthItems::Projects => {
                            let latest_update_date = get_latest_modrinth_project_update_date(&db_pool).await?;
                            info!("Updating Modrinth Projects since: {}", latest_update_date);
                            modrinth_client.update_modrinth_projects(&db_pool, latest_update_date).await?;
                        }
                    }
                },
                UpdateRepositorySubcommand::Hangar { item } => {
                    let hangar_server = HangarServer::new().await;
                    let hangar_client = HangarClient::new(hangar_server)?;

                    match item {
                        UpdateHangarItems::Projects => {
                            let latest_update_date = get_latest_hangar_project_update_date(&db_pool).await?;
                            info!("Updating Hangar Projects since: {}", latest_update_date);
                            hangar_client.update_hangar_projects(&db_pool, latest_update_date).await?;
                        }
                    }
                }
            }

        },
        ActionSubcommand::Refresh => {
            info!("Refreshing common projects...");
            refresh_common_projects(&db_pool).await?;
        }
    }

    Ok(())
}