use crate::database::cornucopia::queries::modrinth_project::{self, ModrinthProjectEntity, UpsertModrinthProjectParams};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use thiserror::Error;
use time::OffsetDateTime;
use tracing::instrument;

#[derive(Clone, Debug, PartialEq)]
pub struct ModrinthProject {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub author: String,
    pub date_created: OffsetDateTime,
    pub date_modified: OffsetDateTime,
    pub downloads: i32,
    pub version_id: Option<String>,
    pub version_name: Option<String>,
    pub icon_url: Option<String>,
    pub monetization_status: Option<String>,
    pub source_url: Option<String>,
    pub source_repository_host: Option<String>,
    pub source_repository_owner: Option<String>,
    pub source_repository_name: Option<String>
}

impl From<ModrinthProject> for UpsertModrinthProjectParams<String, String, String, String, String, String, String, String, String, String, String, String, String> {
    fn from(project: ModrinthProject) -> Self {
        UpsertModrinthProjectParams {
            id: project.id,
            slug: project.slug,
            title: project.title,
            description: project.description,
            author: project.author,
            date_created: project.date_created,
            date_modified: project.date_modified,
            downloads: project.downloads,
            version_id: project.version_id,
            version_name: project.version_name,
            icon_url: project.icon_url,
            monetization_status: project.monetization_status,
            source_url: project.source_url,
            source_repository_host: project.source_repository_host,
            source_repository_owner: project.source_repository_owner,
            source_repository_name: project.source_repository_name
        }
    }
}

impl From<ModrinthProjectEntity> for ModrinthProject {
    fn from(entity: ModrinthProjectEntity) -> Self {
        ModrinthProject {
            id: entity.id,
            slug: entity.slug,
            title: entity.title,
            description: entity.description,
            author: entity.author,
            date_created: entity.date_created,
            date_modified: entity.date_modified,
            downloads: entity.downloads,
            version_id: entity.version_id,
            version_name: entity.version_name,
            icon_url: entity.icon_url,
            monetization_status: entity.monetization_status,
            source_url: entity.source_url,
            source_repository_host: entity.source_repository_host,
            source_repository_owner: entity.source_repository_owner,
            source_repository_name: entity.source_repository_name
        }
    }
}

#[derive(Debug, Error)]
enum ModrinthProjectError {
    #[error("Skipping project {slug}: Database query failed: {source}")]
    DatabaseQueryFailed {
        slug: String,
        source: anyhow::Error
    }
}

#[instrument(
    level = "debug",
    skip(db_pool)
)]
pub async fn upsert_modrinth_project(db_pool: &Pool, project: &ModrinthProject) -> Result<()> {
    let db_client = db_pool.get().await?;

    let db_result = modrinth_project::upsert_modrinth_project()
        .params(&db_client, &project.clone().into())
        .await;

    match db_result {
        Ok(_) => Ok(()),
        Err(err) => Err(
            ModrinthProjectError::DatabaseQueryFailed {
                slug: project.slug.clone(),
                source: err.into()
            }.into()
        )
    }
}

pub async fn get_modrinth_projects(db_pool: &Pool) -> Result<Vec<ModrinthProject>> {
    let db_client = db_pool.get().await?;

    let entities = modrinth_project::get_modrinth_projects()
        .bind(&db_client)
        .all()
        .await?;

    let projects = entities.into_iter().map(|x| x.into()).collect();

    Ok(projects)
}

pub async fn get_latest_modrinth_project_update_date(db_pool: &Pool) -> Result<OffsetDateTime> {
    let db_client = db_pool.get().await?;

    let date = modrinth_project::get_latest_modrinth_project_update_date()
        .bind(&db_client)
        .one()
        .await?;

    Ok(date)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test::DatabaseTestContext;

    use ::function_name::named;
    use speculoos::prelude::*;
    use time::macros::datetime;

    #[tokio::test]
    #[named]
    async fn should_insert_modrinth_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let project = &create_test_modrinth_projects()[0];

        // Act
        upsert_modrinth_project(&context.pool, project).await?;

        // Assert
        let retrieved_projects = get_modrinth_projects(&context.pool).await?;
        let retrieved_project = &retrieved_projects[0];

        assert_that(&retrieved_projects).has_length(1);
        assert_that(&retrieved_project).is_equal_to(project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_update_modrinth_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let project = populate_test_modrinth_project(&context.pool);

        let updated_project = ModrinthProject {
            id: "aaaaaaaa".to_string(),
            slug: "foo".to_string(),
            title: "foo-updated".to_string(),
            description: "foo-description-updated".to_string(),
            author: "Frumple".to_string(),
            date_created: datetime!(2020-01-01 0:00 UTC),
            date_modified: datetime!(2021-07-01 0:00 UTC),
            downloads: 100,
            version_id: Some("aaaa2222".to_string()),
            version_name: Some("v2.3.4".to_string()),
            icon_url: Some("https://cdn.modrinth.com/data/aaaaaaaa/icon.png".to_string()),
            monetization_status: None,
            source_url: Some("https://github.com/Frumple/foo-updated".to_string()),
            source_repository_host: Some("github.com".to_string()),
            source_repository_owner: Some("Frumple".to_string()),
            source_repository_name: Some("foo-updated".to_string())
        };

        // Act
        upsert_modrinth_project(&context.pool, &updated_project).await?;

        // Assert
        let retrieved_projects = get_modrinth_projects(&context.pool).await?;
        let retrieved_project = &retrieved_projects[0];

        assert_that(&retrieved_projects).has_length(1);
        assert_that(&retrieved_project).is_equal_to(&updated_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_get_latest_modrinth_project_update_date() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let projects = create_test_modrinth_projects();
        for project in projects {
            upsert_modrinth_project(&context.pool, &project).await?;
        }

        // Act
        let latest_update_date = get_latest_modrinth_project_update_date(&context.pool).await?;

        // Assert
        assert_that(&latest_update_date).is_equal_to(datetime!(2023-01-01 0:00 UTC));

        // Teardown
        context.drop().await?;

        Ok(())
    }

    pub async fn populate_test_modrinth_project(db_pool: &Pool) -> Result<ModrinthProject> {
        let project = &create_test_modrinth_projects()[0];
        upsert_modrinth_project(db_pool, project).await?;
        Ok(project.clone())
    }

    fn create_test_modrinth_projects() -> Vec<ModrinthProject> {
        vec![
            ModrinthProject {
                id: "aaaaaaaa".to_string(),
                slug: "foo".to_string(),
                title: "foo".to_string(),
                description: "foo-description".to_string(),
                author: "Frumple".to_string(),
                date_created: datetime!(2020-01-01 0:00 UTC),
                date_modified: datetime!(2021-01-01 0:00 UTC),
                downloads: 100,
                version_id: Some("aaaa1111".to_string()),
                version_name: Some("v1.2.3".to_string()),
                icon_url: Some("https://cdn.modrinth.com/data/aaaaaaaa/icon.png".to_string()),
                monetization_status: None,
                source_url: Some("https://github.com/Frumple/foo".to_string()),
                source_repository_host: Some("github.com".to_string()),
                source_repository_owner: Some("Frumple".to_string()),
                source_repository_name: Some("foo".to_string())
            },
            ModrinthProject {
                id: "bbbbbbbb".to_string(),
                slug: "bar".to_string(),
                title: "bar".to_string(),
                description: "bar-description".to_string(),
                author: "Frumple".to_string(),
                date_created: datetime!(2020-01-01 0:00 UTC),
                date_modified: datetime!(2022-01-01 0:00 UTC),
                downloads: 100,
                version_id: Some("bbbb1111".to_string()),
                version_name: Some("v1.2.3".to_string()),
                icon_url: Some("https://cdn.modrinth.com/data/bbbbbbbb/icon.png".to_string()),
                monetization_status: None,
                source_url: Some("https://github.com/Frumple/bar".to_string()),
                source_repository_host: Some("github.com".to_string()),
                source_repository_owner: Some("Frumple".to_string()),
                source_repository_name: Some("foo".to_string())
            },
            ModrinthProject {
                id: "cccccccc".to_string(),
                slug: "baz".to_string(),
                title: "baz".to_string(),
                description: "baz-description".to_string(),
                author: "Frumple".to_string(),
                date_created: datetime!(2020-01-01 0:00 UTC),
                date_modified: datetime!(2023-01-01 0:00 UTC),
                downloads: 100,
                version_id: Some("cccc1111".to_string()),
                version_name: Some("v1.2.3".to_string()),
                icon_url: Some("https://cdn.modrinth.com/data/cccccccc/icon.png".to_string()),
                monetization_status: None,
                source_url: Some("https://github.com/Frumple/baz".to_string()),
                source_repository_host: Some("github.com".to_string()),
                source_repository_owner: Some("Frumple".to_string()),
                source_repository_name: Some("foo".to_string())
            },
        ]
    }
}