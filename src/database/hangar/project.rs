use crate::database::source_repository::SourceRepository;
use crate::database::cornucopia::queries::hangar_project::{self, HangarProjectEntity, UpsertHangarProjectParams};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use thiserror::Error;
use time::OffsetDateTime;
use tracing::instrument;

#[derive(Clone, Debug, PartialEq)]
pub struct HangarProject {
    pub slug: String,
    pub author: String,
    pub name: String,
    pub description: String,
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub latest_minecraft_version: Option<String>,
    pub downloads: i32,
    pub stars: i32,
    pub watchers: i32,
    pub visibility: String,
    pub icon_url: String,
    pub version_name: Option<String>,
    pub source_url: Option<String>,
    pub source_repository: Option<SourceRepository>
}

impl From<HangarProject> for UpsertHangarProjectParams<String, String, String, String, String, String, String, String, String, String, String, String> {
    fn from(project: HangarProject) -> Self {
        let mut source_repository_host = None;
        let mut source_repository_owner = None;
        let mut source_repository_name = None;

        if let Some(repo) = project.source_repository {
            source_repository_host = Some(repo.host);
            source_repository_owner = Some(repo.owner);
            source_repository_name = Some(repo.name);
        }

        UpsertHangarProjectParams {
            slug: project.slug,
            author: project.author,
            name: project.name,
            description: project.description,
            date_created: project.date_created,
            date_updated: project.date_updated,
            latest_minecraft_version: project.latest_minecraft_version,
            downloads: project.downloads,
            stars: project.stars,
            watchers: project.watchers,
            visibility: project.visibility,
            icon_url: project.icon_url,
            version_name: project.version_name,
            source_url: project.source_url,
            source_repository_host,
            source_repository_owner,
            source_repository_name
        }
    }
}

impl From<HangarProjectEntity> for HangarProject {
    fn from(entity: HangarProjectEntity) -> Self {
        let mut source_repository = None;

        if entity.source_repository_host.is_some() &&
           entity.source_repository_owner.is_some() &&
           entity.source_repository_name.is_some() {
            source_repository = Some(SourceRepository {
                host: entity.source_repository_host.unwrap(),
                owner: entity.source_repository_owner.unwrap(),
                name: entity.source_repository_name.unwrap(),
                id: entity.source_repository_id
            })
        }

        HangarProject {
            slug: entity.slug,
            author: entity.author,
            name: entity.name,
            description: entity.description,
            date_created: entity.date_created,
            date_updated: entity.date_updated,
            latest_minecraft_version: entity.latest_minecraft_version,
            downloads: entity.downloads,
            stars: entity.stars,
            watchers: entity.watchers,
            visibility: entity.visibility,
            icon_url: entity.icon_url,
            version_name: entity.version_name,
            source_url: entity.source_url,
            source_repository
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
    level = "debug",
    skip(db_pool)
)]
pub async fn upsert_hangar_project(db_pool: &Pool, project: &HangarProject) -> Result<()> {
    let db_client = db_pool.get().await?;

    let db_result = hangar_project::upsert_hangar_project()
        .params(&db_client, &project.clone().into())
        .await;

    match db_result {
        Ok(_) => Ok(()),
        Err(err) => Err(
            HangarProjectError::DatabaseQueryFailed {
                slug: project.slug.clone(),
                source: err.into()
            }.into()
        )
    }
}

pub async fn get_hangar_projects(db_pool: &Pool) -> Result<Vec<HangarProject>> {
    let db_client = db_pool.get().await?;

    let projects = hangar_project::get_hangar_projects()
        .bind(&db_client)
        .all()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect();

    Ok(projects)
}

pub async fn get_latest_hangar_project_update_date(db_pool: &Pool) -> Result<OffsetDateTime> {
    let db_client = db_pool.get().await?;

    let date = hangar_project::get_latest_hangar_project_update_date()
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
    async fn should_insert_hangar_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let project = &create_test_hangar_projects()[0];

        // Act
        upsert_hangar_project(&context.pool, project).await?;

        // Assert
        let retrieved_projects = get_hangar_projects(&context.pool).await?;
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
        let _project = populate_test_hangar_project(&context.pool).await?;

        let updated_project = HangarProject {
            slug: "foo".to_string(),
            author: "Frumple".to_string(),
            name: "foo-updated".to_string(),
            description: "foo-description-updated".to_string(),
            date_created: datetime!(2020-01-01 0:00 UTC),
            date_updated: datetime!(2021-07-01 0:00 UTC),
            latest_minecraft_version: Some("1.22".to_string()),
            downloads: 100,
            stars: 200,
            watchers: 300,
            visibility: "public".to_string(),
            icon_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
            version_name: Some("v2.3.4".to_string()),
            source_url: Some("https://github.com/alice/foo-updated".to_string()),
            source_repository: Some(SourceRepository {
                host: "github.com".to_string(),
                owner: "alice".to_string(),
                name: "foo-updated".to_string(),
                id: None
            })
        };

        // Act
        upsert_hangar_project(&context.pool, &updated_project).await?;

        // Assert
        let retrieved_projects = get_hangar_projects(&context.pool).await?;
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
        let projects = create_test_hangar_projects();
        for project in projects {
            upsert_hangar_project(&context.pool, &project).await?;
        }

        // Act
        let latest_update_date = get_latest_hangar_project_update_date(&context.pool).await?;

        // Assert
        assert_that(&latest_update_date).is_equal_to(datetime!(2022-02-03 0:00 UTC));

        // Teardown
        context.drop().await?;

        Ok(())
    }

    pub async fn populate_test_hangar_project(db_pool: &Pool) -> Result<HangarProject> {
        let project = &create_test_hangar_projects()[0];
        upsert_hangar_project(db_pool, project).await?;
        Ok(project.clone())
    }

    pub async fn populate_test_hangar_projects(db_pool: &Pool) -> Result<Vec<HangarProject>> {
        let projects = create_test_hangar_projects();
        for project in &projects {
            upsert_hangar_project(db_pool, project).await?
        }
        Ok(projects)
    }

    fn create_test_hangar_projects() -> Vec<HangarProject> {
        vec![
            HangarProject {
                slug: "foo".to_string(),
                author: "alice".to_string(),
                name: "foo-hangar".to_string(),
                description: "foo-hangar-description".to_string(),
                date_created: datetime!(2022-01-01 0:00 UTC),
                date_updated: datetime!(2022-02-03 0:00 UTC),
                latest_minecraft_version: Some("1.21.4".to_string()),
                downloads: 100,
                stars: 200,
                watchers: 200,
                visibility: "public".to_string(),
                icon_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
                version_name: Some("v1.2.3".to_string()),
                source_url: Some("https://github.com/alice/foo".to_string()),
                source_repository: Some(SourceRepository {
                    host: "github.com".to_string(),
                    owner: "alice".to_string(),
                    name: "foo".to_string(),
                    id: None
                })
            },
            HangarProject {
                slug: "bar".to_string(),
                author: "bob".to_string(),
                name: "bar-hangar".to_string(),
                description: "bar-hangar-description".to_string(),
                date_created: datetime!(2022-01-02 0:00 UTC),
                date_updated: datetime!(2022-02-02 0:00 UTC),
                latest_minecraft_version: Some("1.8".to_string()),
                downloads: 300,
                stars: 100,
                watchers: 300,
                visibility: "public".to_string(),
                icon_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
                version_name: Some("v1.2.3".to_string()),
                source_url: Some("https://gitlab.com/bob/bar".to_string()),
                source_repository: Some(SourceRepository {
                    host: "gitlab.com".to_string(),
                    owner: "bob".to_string(),
                    name: "bar".to_string(),
                    id: None
                })
            },
            HangarProject {
                slug: "baz".to_string(),
                author: "eve".to_string(),
                name: "baz-hangar".to_string(),
                description: "baz-hangar-description".to_string(),
                date_created: datetime!(2022-01-03 0:00 UTC),
                date_updated: datetime!(2022-02-01 0:00 UTC),
                latest_minecraft_version: Some("1.16.5".to_string()),
                downloads: 200,
                stars: 300,
                watchers: 100,
                visibility: "public".to_string(),
                icon_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
                version_name: Some("v1.2.3".to_string()),
                source_url: Some("https://bitbucket.org/eve/baz".to_string()),
                source_repository: Some(SourceRepository {
                    host: "bitbucket.org".to_string(),
                    owner: "eve".to_string(),
                    name: "baz".to_string(),
                    id: None
                })
            }
        ]
    }
}