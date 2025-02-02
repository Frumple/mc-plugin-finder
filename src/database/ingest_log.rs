use crate::database::cornucopia::queries::ingest_log::{self, InsertIngestLogParams, IngestLogEntity};
use crate::database::cornucopia::types::public::IngestLogAction as CornucopiaIngestLogAction;
use crate::database::cornucopia::types::public::IngestLogRepository as CornucopiaIngestLogRepository;
use crate::database::cornucopia::types::public::IngestLogItem as CornucopiaIngestLogItem;

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use time::OffsetDateTime;
use thiserror::Error;
use tracing::instrument;

#[derive(Clone, Debug, PartialEq)]
pub enum IngestLogAction {
    Populate,
    Update
}

impl From<IngestLogAction> for CornucopiaIngestLogAction {
    fn from(action: IngestLogAction) -> Self {
        match action {
            IngestLogAction::Populate => CornucopiaIngestLogAction::Populate,
            IngestLogAction::Update => CornucopiaIngestLogAction::Update
        }
    }
}

impl From<CornucopiaIngestLogAction> for IngestLogAction {
    fn from(action: CornucopiaIngestLogAction) -> Self {
        match action {
            CornucopiaIngestLogAction::Populate => IngestLogAction::Populate,
            CornucopiaIngestLogAction::Update => IngestLogAction::Update
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum IngestLogRepository {
    Spigot,
    Modrinth,
    Hangar
}

impl From<IngestLogRepository> for CornucopiaIngestLogRepository {
    fn from(repository: IngestLogRepository) -> Self {
        match repository {
            IngestLogRepository::Spigot => CornucopiaIngestLogRepository::Spigot,
            IngestLogRepository::Modrinth => CornucopiaIngestLogRepository::Modrinth,
            IngestLogRepository::Hangar => CornucopiaIngestLogRepository::Hangar
        }
    }
}

impl From<CornucopiaIngestLogRepository> for IngestLogRepository {
    fn from(repository: CornucopiaIngestLogRepository) -> Self {
        match repository {
            CornucopiaIngestLogRepository::Spigot => IngestLogRepository::Spigot,
            CornucopiaIngestLogRepository::Modrinth => IngestLogRepository::Modrinth,
            CornucopiaIngestLogRepository::Hangar => IngestLogRepository::Hangar
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum IngestLogItem {
    Author,
    Resource,
    Project,
    Version
}

impl From<IngestLogItem> for CornucopiaIngestLogItem {
    fn from(repository: IngestLogItem) -> Self {
        match repository {
            IngestLogItem::Author => CornucopiaIngestLogItem::Author,
            IngestLogItem::Resource => CornucopiaIngestLogItem::Resource,
            IngestLogItem::Project => CornucopiaIngestLogItem::Project,
            IngestLogItem::Version => CornucopiaIngestLogItem::Version
        }
    }
}

impl From<CornucopiaIngestLogItem> for IngestLogItem {
    fn from(repository: CornucopiaIngestLogItem) -> Self {
        match repository {
            CornucopiaIngestLogItem::Author => IngestLogItem::Author,
            CornucopiaIngestLogItem::Resource => IngestLogItem::Resource,
            CornucopiaIngestLogItem::Project => IngestLogItem::Project,
            CornucopiaIngestLogItem::Version => IngestLogItem::Version
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IngestLog {
  pub action: IngestLogAction,
  pub repository: IngestLogRepository,
  pub item: IngestLogItem,
  pub date_started: OffsetDateTime,
  pub date_finished: OffsetDateTime,
  pub items_processed: i32
}

impl From<IngestLog> for InsertIngestLogParams {
    fn from(log: IngestLog) -> Self {
        InsertIngestLogParams {
            action: log.action.into(),
            repository: log.repository.into(),
            item: log.item.into(),
            date_started: log.date_started,
            date_finished: log.date_finished,
            items_processed: log.items_processed
        }
    }
}

impl From<IngestLogEntity> for IngestLog {
    fn from(entity: IngestLogEntity) -> Self {
        IngestLog {
            action: entity.action.into(),
            repository: entity.repository.into(),
            item: entity.item.into(),
            date_started: entity.date_started,
            date_finished: entity.date_finished,
            items_processed: entity.items_processed
        }
    }
}

#[derive(Debug, Error)]
enum IngestLogError {
    #[error("Unable to append ingest log: Database query failed: {source}")]
    DatabaseQueryFailed {
        source: anyhow::Error
    }
}

#[instrument(
    level = "debug",
    skip(db_pool)
)]
pub async fn insert_ingest_log(db_pool: &Pool, log: &IngestLog) -> Result<()> {
    let db_client = db_pool.get().await?;

    let db_result = ingest_log::insert_ingest_log()
        .params(&db_client, &log.clone().into())
        .await;

    match db_result {
        Ok(_) => Ok(()),
        Err(err) => Err(
            IngestLogError::DatabaseQueryFailed {
                source: err.into()
            }.into()
        )
    }
}

#[instrument(
    level = "debug",
    skip(db_pool)
)]
pub async fn get_last_ingest_log(db_pool: &Pool) -> Result<IngestLog> {
    let db_client = db_pool.get().await?;

    let log = ingest_log::get_last_ingest_log()
        .bind(&db_client)
        .one()
        .await?
        .into();

    Ok(log)
}

pub async fn get_ingest_logs(db_pool: &Pool) -> Result<Vec<IngestLog>> {
    let db_client = db_pool.get().await?;

    let logs = ingest_log::get_ingest_logs()
        .bind(&db_client)
        .all()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect();

    Ok(logs)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::database::test::DatabaseTestContext;

    use ::function_name::named;
    use speculoos::prelude::*;
    use time::macros::datetime;

    #[tokio::test]
    #[named]
    async fn should_insert_single_ingest_log_into_db() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let log = &create_test_ingest_logs()[0];

        // Act
        insert_ingest_log(&context.pool, log).await?;

        // Assert
        let retrieved_logs = get_ingest_logs(&context.pool).await?;

        assert_that(&retrieved_logs).has_length(1);
        assert_that(&retrieved_logs[0]).is_equal_to(log);

        let last_log = get_last_ingest_log(&context.pool).await?;

        assert_that(&last_log).is_equal_to(log);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_insert_multiple_ingest_logs_into_db() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let logs = &create_test_ingest_logs();

        // Act
        for log in logs.iter() {
            insert_ingest_log(&context.pool, log).await?;
        }

        // Assert
        let retrieved_logs = get_ingest_logs(&context.pool).await?;

        assert_that(&retrieved_logs).has_length(3);
        assert_that(&retrieved_logs[2]).is_equal_to(logs[2].clone());

        let last_log = get_last_ingest_log(&context.pool).await?;

        assert_that(&last_log).is_equal_to(logs[2].clone());

        // Teardown
        context.drop().await?;

        Ok(())
    }

    fn create_test_ingest_logs() -> Vec<IngestLog> {
        vec![
            IngestLog {
                action: IngestLogAction::Update,
                repository: IngestLogRepository::Spigot,
                item: IngestLogItem::Resource,
                date_started: datetime!(2020-01-01 0:00 UTC),
                date_finished: datetime!(2020-01-01 0:01 UTC),
                items_processed: 50
            },
            IngestLog {
                action: IngestLogAction::Update,
                repository: IngestLogRepository::Modrinth,
                item: IngestLogItem::Project,
                date_started: datetime!(2020-01-01 0:02 UTC),
                date_finished: datetime!(2020-01-01 0:03 UTC),
                items_processed: 30
            },
            IngestLog {
                action: IngestLogAction::Update,
                repository: IngestLogRepository::Hangar,
                item: IngestLogItem::Project,
                date_started: datetime!(2020-01-01 0:04 UTC),
                date_finished: datetime!(2020-01-01 0:05 UTC),
                items_processed: 10
            },
        ]

    }
}