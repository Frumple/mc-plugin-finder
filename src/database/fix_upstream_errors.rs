use crate::database::cornucopia::queries::fix_upstream_errors;

use anyhow::Result;
use deadpool_postgres::Pool;
use tracing::{info, instrument};

pub async fn fix_upstream_errors(db_pool: &Pool) -> Result<()> {
    remove_incorrect_source_repository_host_owner_and_name_from_spigot_resources(db_pool).await?;

    info!("Upstream errors fixed.");

    Ok(())
}

#[instrument(
    level = "info",
    skip(db_pool)
)]
async fn remove_incorrect_source_repository_host_owner_and_name_from_spigot_resources(db_pool: &Pool) -> Result<()> {
    let db_client = db_pool.get().await?;

    fix_upstream_errors::remove_incorrect_source_repository_host_owner_and_name_from_spigot_resources()
        .bind(&db_client)
        .await?;

    Ok(())
}