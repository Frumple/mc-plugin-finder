use crate::hangar::{HangarClient, HangarServer};
use crate::modrinth::{ModrinthClient, ModrinthServer};
use crate::spigot::{SpigotClient, SpigotServer};

use mc_plugin_finder::database::get_db;

use mc_plugin_finder::database::hangar::project::get_latest_hangar_project_update_date;
use mc_plugin_finder::database::modrinth::project::get_latest_modrinth_project_update_date;
use mc_plugin_finder::database::spigot::author::get_highest_spigot_author_id;
use mc_plugin_finder::database::spigot::resource::get_latest_spigot_resource_update_date;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use deadpool_postgres::Pool;
use tracing::{info, warn};
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::fmt::format::FmtSpan;
use url::Url;

pub mod hangar;
pub mod modrinth;
pub mod spigot;

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
    Refresh,
    /// Update all items and refresh
    UpdateAllAndRefresh
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
    let appender = tracing_appender::rolling::daily("logs/ingest", "ingest.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(appender);

    let file_layer = Layer::new()
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(false);

    let console_layer = Layer::new()
        .with_writer(std::io::stdout)
        .with_span_events(FmtSpan::CLOSE);

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with(file_layer)
        .with(console_layer);
    tracing::subscriber::set_global_default(subscriber)?;

    // Load environment variables from .env file if it exists
    match dotenvy::dotenv() {
        Ok(_) => info!("Environment variables successfully loaded from .env file."),
        Err(_) => warn!("Could not load environment variables from .env file, falling back to set variables...")
    };

    // Initialize database client
    let db = get_db();
    let db_pool = db.create_pool().await?;

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
                            update_spigot_authors(&spigot_client, &db_pool).await?;
                        },
                        UpdateSpigotItems::Resources => {
                            update_spigot_resources(&spigot_client, &db_pool).await?;
                        }
                    }
                },
                UpdateRepositorySubcommand::Modrinth { item } => {
                    let modrinth_server = ModrinthServer::new().await;
                    let modrinth_client = ModrinthClient::new(modrinth_server)?;

                    match item {
                        UpdateModrinthItems::Projects => {
                            update_modrinth_projects(&modrinth_client, &db_pool).await?;
                        }
                    }
                },
                UpdateRepositorySubcommand::Hangar { item } => {
                    let hangar_server = HangarServer::new().await;
                    let hangar_client = HangarClient::new(hangar_server)?;

                    match item {
                        UpdateHangarItems::Projects => {
                            update_hangar_projects(&hangar_client, &db_pool).await?;
                        }
                    }
                },
            }

        },
        ActionSubcommand::Refresh => {
            refresh_common_projects(&db_pool).await?;
        },
        ActionSubcommand::UpdateAllAndRefresh => {
            info!("Updating all items...");

            let spigot_server = SpigotServer::new().await;
            let spigot_client = SpigotClient::new(spigot_server)?;

            let modrinth_server = ModrinthServer::new().await;
            let modrinth_client = ModrinthClient::new(modrinth_server)?;

            let hangar_server = HangarServer::new().await;
            let hangar_client = HangarClient::new(hangar_server)?;

            update_spigot_authors(&spigot_client, &db_pool).await?;
            update_spigot_resources(&spigot_client, &db_pool).await?;
            update_modrinth_projects(&modrinth_client, &db_pool).await?;
            update_hangar_projects(&hangar_client, &db_pool).await?;
            refresh_common_projects(&db_pool).await?;
        }
    }

    Ok(())
}

async fn update_spigot_authors(spigot_client: &SpigotClient<SpigotServer>, db_pool: &Pool) -> Result<()> {
    let highest_author_id = get_highest_spigot_author_id(db_pool).await?;
    info!("Updating Spigot Authors with ID higher than: {}", highest_author_id);
    spigot_client.update_spigot_authors(db_pool, highest_author_id).await?;

    Ok(())
}

async fn update_spigot_resources(spigot_client: &SpigotClient<SpigotServer>, db_pool: &Pool) -> Result<()> {
    let latest_update_date = get_latest_spigot_resource_update_date(db_pool).await?;
    info!("Updating Spigot Resources since: {}", latest_update_date);
    spigot_client.update_spigot_resources(db_pool, latest_update_date).await?;

    Ok(())
}

async fn update_modrinth_projects(modrinth_client: &ModrinthClient<ModrinthServer>, db_pool: &Pool) -> Result<()> {
    let latest_update_date = get_latest_modrinth_project_update_date(db_pool).await?;
    info!("Updating Modrinth Projects since: {}", latest_update_date);
    modrinth_client.update_modrinth_projects(db_pool, latest_update_date).await?;

    Ok(())
}

async fn update_hangar_projects(hangar_client: &HangarClient<HangarServer>, db_pool: &Pool) -> Result<()> {
    let latest_update_date = get_latest_hangar_project_update_date(db_pool).await?;
    info!("Updating Hangar Projects since: {}", latest_update_date);
    hangar_client.update_hangar_projects(db_pool, latest_update_date).await?;

    Ok(())
}

async fn refresh_common_projects(db_pool: &Pool) -> Result<()> {
    info!("Refreshing common projects...");
    mc_plugin_finder::database::common::project::refresh_common_projects(db_pool).await?;

    Ok(())
}