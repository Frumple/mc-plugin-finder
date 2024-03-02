use crate::database::cornucopia::queries::hangar_project::{self, HangarProjectEntity, UpsertHangarProjectParams};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Client;
use thiserror::Error;
use time::OffsetDateTime;
use tracing::instrument;

#[derive(Clone, Debug, PartialEq)]
pub struct HangarProject {
    pub slug: String,
    pub owner: String,
    pub name: String,
    pub description: String,
    pub created_at: OffsetDateTime,
    pub last_updated: OffsetDateTime,
    pub visibility: String,
    pub avatar_url: String,
    pub version: String,
    pub source_code_link: Option<String>,
    pub source_repository_host: Option<String>,
    pub source_repository_owner: Option<String>,
    pub source_repository_name: Option<String>
}

impl From<HangarProject> for UpsertHangarProjectParams<String, String, String, String, String, String, String, String, String, String, String> {
    fn from(project: HangarProject) -> Self {
        UpsertHangarProjectParams {
            slug: project.slug,
            owner: project.owner,
            name: project.name,
            description: project.description,
            created_at: project.created_at,
            last_updated: project.last_updated,
            visibility: project.visibility,
            avatar_url: project.avatar_url,
            version: project.version,
            source_code_link: project.source_code_link,
            source_repository_host: project.source_repository_host,
            source_repository_owner: project.source_repository_owner,
            source_repository_name: project.source_repository_name
        }
    }
}

impl From<HangarProjectEntity> for HangarProject {
    fn from(entity: HangarProjectEntity) -> Self {
        HangarProject {
            slug: entity.slug,
            owner: entity.owner,
            name: entity.name,
            description: entity.description,
            created_at: entity.created_at,
            last_updated: entity.last_updated,
            visibility: entity.visibility,
            avatar_url: entity.avatar_url,
            version: entity.version,
            source_code_link: entity.source_code_link,
            source_repository_host: entity.source_repository_host,
            source_repository_owner: entity.source_repository_owner,
            source_repository_name: entity.source_repository_name
        }
    }
}

#[derive(Debug, Error)]
enum HangarProjectError {
    #[error("Skipping project {slug}: Database query failed: {source}")]
    DatabaseQueryFailed {
        slug: String,
        source: anyhow::Error
    }
}

#[instrument(
    level = "trace",
    skip(db_client)
)]
pub async fn upsert_hangar_project(db_client: &Client, project: HangarProject) -> Result<()> {
    let slug = project.slug.clone();

    let db_result = hangar_project::upsert_hangar_project()
        .params(db_client, &project.into())
        .await;

    match db_result {
        Ok(_) => Ok(()),
        Err(err) => Err(
            HangarProjectError::DatabaseQueryFailed {
                slug,
                source: err.into()
            }.into()
        )
    }
}

pub async fn get_hangar_projects(db_client: &Client) -> Result<Vec<HangarProject>> {
    let entities = hangar_project::get_hangar_projects()
        .bind(db_client)
        .all()
        .await?;

    let projects = entities.into_iter().map(|x| x.into()).collect();

    Ok(projects)
}

pub async fn get_latest_hangar_project_update_date(db_client: &Client) -> Result<OffsetDateTime> {
    let date = hangar_project::get_latest_hangar_project_update_date()
        .bind(db_client)
        .one()
        .await?;

    Ok(date)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::database::hangar::project::{upsert_hangar_project, get_latest_hangar_project_update_date};
    use crate::test::DatabaseTestContext;

    use ::function_name::named;
    use speculoos::prelude::*;
    use time::macros::datetime;

    #[tokio::test]
    #[named]
    async fn should_insert_hangar_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let project = &create_test_projects()[0];

        // Act
        upsert_hangar_project(&context.client, project.clone()).await?;

        // Assert
        let retrieved_projects = get_hangar_projects(&context.client).await?;
        let retrieved_project = &retrieved_projects[0];

        assert_that(&retrieved_projects).has_length(1);
        assert_that(&retrieved_project).is_equal_to(project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_update_hangar_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let project = &create_test_projects()[0];
        upsert_hangar_project(&context.client, project.clone()).await?;

        let updated_project = HangarProject {
            slug: "foo".to_string(),
            owner: "Frumple".to_string(),
            name: "foo-updated".to_string(),
            description: "foo-description-updated".to_string(),
            created_at: datetime!(2020-01-01 0:00 UTC),
            last_updated: datetime!(2021-07-01 0:00 UTC),
            visibility: "public".to_string(),
            avatar_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
            version: "v2.3.4".to_string(),
            source_code_link: Some("https://github.com/Frumple/foo-updated".to_string()),
            source_repository_host: Some("github.com".to_string()),
            source_repository_owner: Some("Frumple".to_string()),
            source_repository_name: Some("foo-updated".to_string())
        };

        // Act
        upsert_hangar_project(&context.client, updated_project.clone()).await?;

        // Assert
        let retrieved_projects = get_hangar_projects(&context.client).await?;
        let retrieved_project = &retrieved_projects[0];

        assert_that(&retrieved_projects).has_length(1);
        assert_that(&retrieved_project).is_equal_to(&updated_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_get_latest_hangar_project_update_date() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let projects = create_test_projects();
        for project in projects {
            upsert_hangar_project(&context.client, project).await?;
        }

        // Act
        let latest_update_date = get_latest_hangar_project_update_date(&context.client).await?;

        // Assert
        assert_that(&latest_update_date).is_equal_to(datetime!(2023-01-01 0:00 UTC));

        // Teardown
        context.drop().await?;

        Ok(())
    }

    fn create_test_projects() -> Vec<HangarProject> {
        vec![
            HangarProject {
                slug: "foo".to_string(),
                owner: "Frumple".to_string(),
                name: "foo".to_string(),
                description: "foo-description".to_string(),
                created_at: datetime!(2020-01-01 0:00 UTC),
                last_updated: datetime!(2021-01-01 0:00 UTC),
                visibility: "public".to_string(),
                avatar_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
                version: "v1.2.3".to_string(),
                source_code_link: Some("https://github.com/Frumple/foo".to_string()),
                source_repository_host: Some("github.com".to_string()),
                source_repository_owner: Some("Frumple".to_string()),
                source_repository_name: Some("foo".to_string())
            },
            HangarProject {
                slug: "bar".to_string(),
                owner: "Frumple".to_string(),
                name: "bar".to_string(),
                description: "bar-description".to_string(),
                created_at: datetime!(2020-01-01 0:00 UTC),
                last_updated: datetime!(2022-01-01 0:00 UTC),
                visibility: "public".to_string(),
                avatar_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
                version: "v1.2.3".to_string(),
                source_code_link: Some("https://github.com/Frumple/bar".to_string()),
                source_repository_host: Some("github.com".to_string()),
                source_repository_owner: Some("Frumple".to_string()),
                source_repository_name: Some("bar".to_string())
            },
            HangarProject {
                slug: "baz".to_string(),
                owner: "Frumple".to_string(),
                name: "baz".to_string(),
                description: "baz-description".to_string(),
                created_at: datetime!(2020-01-01 0:00 UTC),
                last_updated: datetime!(2023-01-01 0:00 UTC),
                visibility: "public".to_string(),
                avatar_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
                version: "v1.2.3".to_string(),
                source_code_link: Some("https://github.com/Frumple/baz".to_string()),
                source_repository_host: Some("github.com".to_string()),
                source_repository_owner: Some("Frumple".to_string()),
                source_repository_name: Some("baz".to_string())
            }
        ]
    }
}