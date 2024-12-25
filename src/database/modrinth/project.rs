use crate::database::source_repository::SourceRepository;
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
    pub name: String,
    pub description: String,
    pub author: String,
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub latest_minecraft_version: Option<String>,
    pub downloads: i32,
    pub follows: i32,
    pub version_id: Option<String>,
    pub version_name: Option<String>,
    pub status: String,
    pub icon_url: Option<String>,
    pub source_url: Option<String>,
    pub source_repository: Option<SourceRepository>
}

impl From<ModrinthProject> for UpsertModrinthProjectParams<String, String, String, String, String, String, String, String, String, String, String, String, String, String> {
    fn from(project: ModrinthProject) -> Self {
        let mut source_repository_host = None;
        let mut source_repository_owner = None;
        let mut source_repository_name = None;

        if let Some(repo) = project.source_repository {
            source_repository_host = Some(repo.host);
            source_repository_owner = Some(repo.owner);
            source_repository_name = Some(repo.name);
        }

        UpsertModrinthProjectParams {
            id: project.id,
            slug: project.slug,
            name: project.name,
            description: project.description,
            author: project.author,
            date_created: project.date_created,
            date_updated: project.date_updated,
            latest_minecraft_version: project.latest_minecraft_version,
            downloads: project.downloads,
            follows: project.follows,
            version_id: project.version_id,
            version_name: project.version_name,
            status: project.status,
            icon_url: project.icon_url,
            source_url: project.source_url,
            source_repository_host,
            source_repository_owner,
            source_repository_name
        }
    }
}

impl From<ModrinthProjectEntity> for ModrinthProject {
    fn from(entity: ModrinthProjectEntity) -> Self {
        let mut source_repository = None;

        if entity.source_repository_host.is_some() &&
           entity.source_repository_owner.is_some() &&
           entity.source_repository_name.is_some() {
            source_repository = Some(SourceRepository {
                host: entity.source_repository_host.unwrap(),
                owner: entity.source_repository_owner.unwrap(),
                name: entity.source_repository_name.unwrap()
            })
        }

        ModrinthProject {
            id: entity.id,
            slug: entity.slug,
            name: entity.name,
            description: entity.description,
            author: entity.author,
            date_created: entity.date_created,
            date_updated: entity.date_updated,
            latest_minecraft_version: entity.latest_minecraft_version,
            downloads: entity.downloads,
            follows: entity.follows,
            version_id: entity.version_id,
            version_name: entity.version_name,
            status: entity.status,
            icon_url: entity.icon_url,
            source_url: entity.source_url,
            source_repository
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
    use crate::database::test::DatabaseTestContext;

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
        let _project = populate_test_modrinth_project(&context.pool).await?;

        let updated_project = ModrinthProject {
            id: "aaaaaaaa".to_string(),
            slug: "foo".to_string(),
            name: "foo-updated".to_string(),
            description: "foo-description-updated".to_string(),
            author: "Frumple".to_string(),
            date_created: datetime!(2020-01-01 0:00 UTC),
            date_updated: datetime!(2021-07-01 0:00 UTC),
            latest_minecraft_version: Some("1.22".to_string()),
            downloads: 100,
            follows: 200,
            version_id: Some("aaaa2222".to_string()),
            version_name: Some("v2.3.4".to_string()),
            status: "archived".to_string(),
            icon_url: Some("https://cdn.modrinth.com/data/aaaaaaaa/icon.png".to_string()),
            source_url: Some("https://github.com/alice/foo-updated".to_string()),
            source_repository: Some(SourceRepository {
                host: "github.com".to_string(),
                owner: "alice".to_string(),
                name: "foo-updated".to_string()
            })
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
        assert_that(&latest_update_date).is_equal_to(datetime!(2021-02-03 0:00 UTC));

        // Teardown
        context.drop().await?;

        Ok(())
    }

    pub async fn populate_test_modrinth_project(db_pool: &Pool) -> Result<ModrinthProject> {
        let project = &create_test_modrinth_projects()[0];
        upsert_modrinth_project(db_pool, project).await?;
        Ok(project.clone())
    }

    pub async fn populate_test_modrinth_projects(db_pool: &Pool) -> Result<Vec<ModrinthProject>> {
        let projects = create_test_modrinth_projects();
        for project in &projects {
            upsert_modrinth_project(db_pool, project).await?;
        }
        Ok(projects)
    }

    fn create_test_modrinth_projects() -> Vec<ModrinthProject> {
        vec![
            ModrinthProject {
                id: "aaaaaaaa".to_string(),
                slug: "foo".to_string(),
                name: "foo-modrinth".to_string(),
                description: "foo-modrinth-description".to_string(),
                author: "alice".to_string(),
                date_created: datetime!(2021-01-01 0:00 UTC),
                date_updated: datetime!(2021-02-03 0:00 UTC),
                latest_minecraft_version: Some("1.21".to_string()),
                downloads: 100,
                follows: 200,
                version_id: Some("aaaa1111".to_string()),
                version_name: Some("v1.2.3".to_string()),
                status: "approved".to_string(),
                icon_url: Some("https://cdn.modrinth.com/data/aaaaaaaa/icon.png".to_string()),
                source_url: Some("https://github.com/alice/foo".to_string()),
                source_repository: Some(SourceRepository {
                    host: "github.com".to_string(),
                    owner: "alice".to_string(),
                    name: "foo".to_string()
                })
            },
            ModrinthProject {
                id: "bbbbbbbb".to_string(),
                slug: "bar".to_string(),
                name: "bar-modrinth".to_string(),
                description: "bar-modrinth-description".to_string(),
                author: "bob".to_string(),
                date_created: datetime!(2021-01-02 0:00 UTC),
                date_updated: datetime!(2021-02-02 0:00 UTC),
                latest_minecraft_version: Some("1.8".to_string()),
                downloads: 300,
                follows: 300,
                version_id: Some("bbbb1111".to_string()),
                version_name: Some("v1.2.3".to_string()),
                status: "approved".to_string(),
                icon_url: Some("https://cdn.modrinth.com/data/bbbbbbbb/icon.png".to_string()),
                source_url: Some("https://gitlab.com/bob/bar".to_string()),
                source_repository: Some(SourceRepository {
                    host: "gitlab.com".to_string(),
                    owner: "bob".to_string(),
                    name: "bar".to_string()
                })
            },
            ModrinthProject {
                id: "cccccccc".to_string(),
                slug: "baz".to_string(),
                name: "baz-modrinth".to_string(),
                description: "baz-modrinth-description".to_string(),
                author: "eve".to_string(),
                date_created: datetime!(2021-01-03 0:00 UTC),
                date_updated: datetime!(2021-02-01 0:00 UTC),
                latest_minecraft_version: Some("1.16".to_string()),
                downloads: 200,
                follows: 100,
                version_id: Some("cccc1111".to_string()),
                version_name: Some("v1.2.3".to_string()),
                status: "approved".to_string(),
                icon_url: Some("https://cdn.modrinth.com/data/cccccccc/icon.png".to_string()),
                source_url: Some("https://bitbucket.org/eve/baz".to_string()),
                source_repository: Some(SourceRepository {
                    host: "bitbucket.org".to_string(),
                    owner: "eve".to_string(),
                    name: "baz".to_string()
                })
            },
        ]
    }
}