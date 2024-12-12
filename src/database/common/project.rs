use crate::database::cornucopia::queries::common_project::{self, CommonProjectEntity, UpsertCommonProjectParams};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use thiserror::Error;
use time::OffsetDateTime;
use time::macros::datetime;
use tracing::{info, instrument};

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProject {
    pub id: Option<i32>,
    pub spigot: Option<CommonProjectSpigot>,
    pub modrinth: Option<CommonProjectModrinth>,
    pub hangar: Option<CommonProjectHangar>
}

impl From<CommonProject> for UpsertCommonProjectParams<String, String, String, String, String, String, String, String, String, String, String> {
    fn from(project: CommonProject) -> Self {
        let mut params = UpsertCommonProjectParams {
            id: project.id.map(i64::from),

            spigot_id: None,
            spigot_name: None,
            spigot_description: None,
            spigot_author: None,

            modrinth_id: None,
            modrinth_name: None,
            modrinth_description: None,
            modrinth_author: None,

            hangar_slug: None,
            hangar_name: None,
            hangar_description: None,
            hangar_author: None
        };

        if let Some(spigot) = project.spigot {
            params.spigot_id = Some(spigot.id);
            params.spigot_name = spigot.name;
            params.spigot_description = Some(spigot.description);
            params.spigot_author = Some(spigot.author);
        };

        if let Some(modrinth) = project.modrinth {
            params.modrinth_id = Some(modrinth.id);
            params.modrinth_name = Some(modrinth.name);
            params.modrinth_description = Some(modrinth.description);
            params.modrinth_author = Some(modrinth.author);
        }


        if let Some(hangar) = project.hangar {
            params.hangar_slug = Some(hangar.slug);
            params.hangar_name = Some(hangar.name);
            params.hangar_description = Some(hangar.description);
            params.hangar_author = Some(hangar.author);
        }

        params
    }
}

impl From<CommonProjectEntity> for CommonProject {
    fn from(entity: CommonProjectEntity) -> Self {
        let spigot = entity.spigot_id.map(|_| CommonProjectSpigot {
            id: entity.spigot_id.expect("Spigot id should not be None"),
            name: entity.spigot_name,
            description: entity.spigot_description.expect("Spigot description should not be None"),
            author: entity.spigot_author.expect("Spigot author should not be None")
        });

        let modrinth = entity.modrinth_id.clone().map(|_| CommonProjectModrinth {
            id: entity.modrinth_id.expect("Modrinth id should not be None"),
            name: entity.modrinth_name.expect("Modrinth name should not be None"),
            description: entity.modrinth_description.expect("Modrinth description should not be None"),
            author: entity.modrinth_author.expect("Modrinth author should not be None")
        });

        let hangar = entity.hangar_slug.clone().map(|_| CommonProjectHangar {
            slug: entity.hangar_slug.expect("Hangar slug should not be None"),
            name: entity.hangar_name.expect("Hangar name should not be None"),
            description: entity.hangar_description.expect("Hangar description should not be None"),
            author: entity.hangar_author.expect("Hangar author should not be None")
        });

        CommonProject {
            id: entity.id,
            spigot,
            modrinth,
            hangar
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProjectSpigot {
    pub id: i32,
    pub name: Option<String>,
    pub description: String,
    pub author: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProjectModrinth {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProjectHangar {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub author: String,
}

#[derive(Debug, Error)]
enum CommonProjectError {
    #[error("Skipping project: Database query failed: {source}")]
    DatabaseQueryFailed {
        id: Option<i32>,
        source: anyhow::Error
    }
}

#[instrument(
    level = "info",
    skip(db_pool)
)]
pub async fn get_merged_common_projects(db_pool: &Pool, update_date_later_than: Option<OffsetDateTime>) -> Result<Vec<CommonProject>> {
    let db_client = db_pool.get().await?;

    // If no update date is provided, default to January 1st 2000, which should merge all projects ever created
    let date_updated = match update_date_later_than {
        Some(date) => date,
        None => datetime!(2000-01-01 0:00 UTC)
    };

    let entities = common_project::get_merged_common_projects()
        .bind(&db_client, &date_updated)
        .all()
        .await?;

    let projects = entities.into_iter().map(|x| x.into()).collect();

    Ok(projects)
}

#[instrument(
    level = "info",
    skip(db_pool, common_projects)
)]
pub async fn upsert_common_projects(db_pool: &Pool, common_projects: &Vec<CommonProject>) -> Result<()> {
    let mut count = 0;

    for project in common_projects {
        upsert_common_project(db_pool, project).await?;
        count += 1;
    }

    info!("Common projects merged: {}", count);

    Ok(())
}

#[instrument(
    level = "debug",
    skip(db_pool)
)]
pub async fn upsert_common_project(db_pool: &Pool, project: &CommonProject) -> Result<()> {
    let db_client = db_pool.get().await?;
    let id = project.id;

    let db_result = common_project::upsert_common_project()
        .params(&db_client, &project.clone().into())
        .await;

    match db_result {
        Ok(_) => Ok(()),
        Err(err) => Err(
            CommonProjectError::DatabaseQueryFailed {
                id,
                source: err.into()
            }.into()
        )
    }
}

pub async fn get_common_projects(db_pool: &Pool) -> Result<Vec<CommonProject>> {
    let db_client = db_pool.get().await?;

    let entities = common_project::get_common_projects()
        .bind(&db_client)
        .all()
        .await?;

    let projects = entities.into_iter().map(|x| x.into()).collect();

    Ok(projects)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::database::spigot::author::SpigotAuthor;
    use crate::database::spigot::resource::{SpigotResource, upsert_spigot_resource};
    use crate::database::spigot::resource::test::populate_test_spigot_author_and_resource;
    use crate::database::spigot::test::SPIGOT_BASE64_TEST_ICON_DATA;

    use crate::database::modrinth::project::{ModrinthProject, upsert_modrinth_project};
    use crate::database::modrinth::project::test::populate_test_modrinth_project;

    use crate::database::hangar::project::{HangarProject, upsert_hangar_project};
    use crate::database::hangar::project::test::populate_test_hangar_project;

    use crate::database::source_repository::SourceRepository;

    use crate::database::test::DatabaseTestContext;

    use ::function_name::named;
    use speculoos::prelude::*;

    #[tokio::test]
    #[named]
    async fn should_only_merge_after_provided_update_date() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        // Create two resources with update_dates 2023-01-01 and 2024-01-01 respectively.
        let (spigot_author, _spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;

        let spigot_resource2 = SpigotResource {
            id: 2,
            name: "bar".to_string(),
            parsed_name: Some("bar".to_string()),
            description: "bar-description".to_string(),
            slug: "bar.2".to_string(),
            date_created: datetime!(2020-01-01 0:00 UTC),
            date_updated: datetime!(2024-01-01 0:00 UTC),
            latest_minecraft_version: Some("1.8".to_string()),
            downloads: 100,
            likes: 200,
            author_id: 1,
            version_id: 1,
            version_name: None,
            premium: false,
            icon_url: Some("data/resource_icons/1/1.jpg".to_string()),
            icon_data: Some(SPIGOT_BASE64_TEST_ICON_DATA.to_string()),
            source_url: Some("https://gitlab.com/bob/bar".to_string()),
            source_repository: Some(SourceRepository {
                host: "gitlab.com".to_string(),
                owner: "bob".to_string(),
                name: "bar".to_string()
            })
        };
        upsert_spigot_resource(&context.pool, &spigot_resource2).await?;

        // Act
        // Get merged common projects with update date greater than 2023-07-01.
        let merged_projects = get_merged_common_projects(&context.pool, Some(datetime!(2023-07-01 0:00 UTC))).await?;

        // Assert
        // Only the resource with date_updated 2024-01-01 should be merged.
        assert_that(&merged_projects).has_length(1);

        let merged_project = &merged_projects[0];
        assert_that(&merged_project.id).is_none();

        assert_spigot_fields_are_equal(merged_project, &spigot_author, &spigot_resource2);
        assert_modrinth_fields_are_none(merged_project);
        assert_hangar_fields_are_none(merged_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_insert_update_spigot_resource() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_author, mut spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;

        // Act 1 - Get merged project
        let merged_projects = get_merged_common_projects(&context.pool, None).await?;

        // Assert 1 - Verify merged project
        assert_that(&merged_projects).has_length(1);

        let merged_project = &merged_projects[0];
        assert_that(&merged_project.id).is_none();

        assert_spigot_fields_are_equal(merged_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_none(merged_project);
        assert_hangar_fields_are_none(merged_project);

        // Act 2 - Insert project
        upsert_common_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];
        assert_that(&inserted_project.id).is_some();

        assert_spigot_fields_are_equal(inserted_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_none(inserted_project);
        assert_hangar_fields_are_none(inserted_project);

        // Act 3 - Update project
        spigot_resource.parsed_name = Some("foo-updated".to_string());
        spigot_resource.description = "foo-updated-description".to_string();
        spigot_resource.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        let new_merged_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_none(updated_project);
        assert_hangar_fields_are_none(updated_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_insert_update_modrinth_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let mut modrinth_project = populate_test_modrinth_project(&context.pool).await?;

        // Act 1 - Get merged project
        let merged_projects = get_merged_common_projects(&context.pool, None).await?;

        // Assert 1 - Verify merged project
        assert_that(&merged_projects).has_length(1);

        let merged_project = &merged_projects[0];
        assert_that(&merged_project.id).is_none();

        assert_spigot_fields_are_none(merged_project);
        assert_modrinth_fields_are_equal(merged_project, &modrinth_project);
        assert_hangar_fields_are_none(merged_project);

        // Act 2 - Insert project
        upsert_common_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];
        assert_that(&inserted_project.id).is_some();

        assert_spigot_fields_are_none(inserted_project);
        assert_modrinth_fields_are_equal(inserted_project, &modrinth_project);
        assert_hangar_fields_are_none(inserted_project);

        // Act 3 - Update project
        modrinth_project.name = "foo-updated".to_string();
        modrinth_project.description = "foo-updated-description".to_string();
        modrinth_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_modrinth_project(&context.pool, &modrinth_project).await?;

        let new_merged_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_spigot_fields_are_none(updated_project);
        assert_modrinth_fields_are_equal(updated_project, &modrinth_project);
        assert_hangar_fields_are_none(updated_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_insert_update_hangar_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let mut hangar_project = populate_test_hangar_project(&context.pool).await?;

        // Act 1 - Get merged project
        let merged_projects = get_merged_common_projects(&context.pool, None).await?;

        // Assert 1 - Verify merged project
        assert_that(&merged_projects).has_length(1);

        let merged_project = &merged_projects[0];
        assert_that(&merged_project.id).is_none();

        assert_spigot_fields_are_none(merged_project);
        assert_modrinth_fields_are_none(merged_project);
        assert_hangar_fields_are_equal(merged_project, &hangar_project);

        // Act 2 - Insert project
        upsert_common_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];
        assert_that(&inserted_project.id).is_some();

        assert_spigot_fields_are_none(inserted_project);
        assert_modrinth_fields_are_none(inserted_project);
        assert_hangar_fields_are_equal(inserted_project, &hangar_project);

        // Act 3 - Update project
        hangar_project.name = "foo-updated".to_string();
        hangar_project.description = "foo-updated-description".to_string();
        hangar_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        let new_merged_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_spigot_fields_are_none(updated_project);
        assert_modrinth_fields_are_none(updated_project);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_insert_update_spigot_and_modrinth() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_author, mut spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;
        let mut modrinth_project = populate_test_modrinth_project(&context.pool).await?;

        // Act 1 - Get merged project
        let merged_projects = get_merged_common_projects(&context.pool, None).await?;

        // Assert 1 - Verify merged project
        assert_that(&merged_projects).has_length(1);

        let merged_project = &merged_projects[0];
        assert_that(&merged_project.id).is_none();

        assert_spigot_fields_are_equal(merged_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_equal(merged_project, &modrinth_project);
        assert_hangar_fields_are_none(merged_project);

        // Act 2 - Insert project
        upsert_common_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];
        assert_that(&inserted_project.id).is_some();

        assert_spigot_fields_are_equal(inserted_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_equal(inserted_project, &modrinth_project);
        assert_hangar_fields_are_none(inserted_project);

        // Act 3 - Update project
        spigot_resource.parsed_name = Some("foo-updated".to_string());
        spigot_resource.description = "foo-updated-description".to_string();
        spigot_resource.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        modrinth_project.name = "foo-updated".to_string();
        modrinth_project.description = "foo-updated-description".to_string();
        modrinth_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_modrinth_project(&context.pool, &modrinth_project).await?;

        let new_merged_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_equal(updated_project, &modrinth_project);
        assert_hangar_fields_are_none(updated_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_insert_update_spigot_and_hangar() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_author, mut spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;
        let mut hangar_project = populate_test_hangar_project(&context.pool).await?;

        // Act 1 - Get merged project
        let merged_projects = get_merged_common_projects(&context.pool, None).await?;

        // Assert 1 - Verify merged project
        assert_that(&merged_projects).has_length(1);

        let merged_project = &merged_projects[0];
        assert_that(&merged_project.id).is_none();

        assert_spigot_fields_are_equal(merged_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_none(merged_project);
        assert_hangar_fields_are_equal(merged_project, &hangar_project);

        // Act 2 - Insert project
        upsert_common_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];
        assert_that(&inserted_project.id).is_some();

        assert_spigot_fields_are_equal(inserted_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_none(inserted_project);
        assert_hangar_fields_are_equal(inserted_project, &hangar_project);

        // Act 3 - Update project
        spigot_resource.parsed_name = Some("foo-updated".to_string());
        spigot_resource.description = "foo-updated-description".to_string();
        spigot_resource.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        hangar_project.name = "foo-updated".to_string();
        hangar_project.description = "foo-updated-description".to_string();
        hangar_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        let new_merged_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_none(updated_project);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_insert_update_modrinth_and_hangar() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let mut modrinth_project = populate_test_modrinth_project(&context.pool).await?;
        let mut hangar_project = populate_test_hangar_project(&context.pool).await?;

        // Act 1 - Get merged project
        let merged_projects = get_merged_common_projects(&context.pool, None).await?;

        // Assert 1 - Verify merged project
        assert_that(&merged_projects).has_length(1);

        let merged_project = &merged_projects[0];
        assert_that(&merged_project.id).is_none();

        assert_spigot_fields_are_none(merged_project);
        assert_modrinth_fields_are_equal(merged_project, &modrinth_project);
        assert_hangar_fields_are_equal(merged_project, &hangar_project);

        // Act 2 - Insert project
        upsert_common_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];
        assert_that(&inserted_project.id).is_some();

        assert_spigot_fields_are_none(inserted_project);
        assert_modrinth_fields_are_equal(inserted_project, &modrinth_project);
        assert_hangar_fields_are_equal(inserted_project, &hangar_project);

        // Act 3 - Update project
        modrinth_project.name = "foo-updated".to_string();
        modrinth_project.description = "foo-updated-description".to_string();
        modrinth_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_modrinth_project(&context.pool, &modrinth_project).await?;

        hangar_project.name = "foo-updated".to_string();
        hangar_project.description = "foo-updated-description".to_string();
        hangar_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        let new_merged_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_spigot_fields_are_none(updated_project);
        assert_modrinth_fields_are_equal(updated_project, &modrinth_project);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_insert_update_spigot_modrinth_and_hangar() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_author, mut spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;
        let mut modrinth_project = populate_test_modrinth_project(&context.pool).await?;
        let mut hangar_project = populate_test_hangar_project(&context.pool).await?;

        // Act 1 - Get merged project
        let merged_projects = get_merged_common_projects(&context.pool, None).await?;

        // Assert 1 - Verify merged project
        assert_that(&merged_projects).has_length(1);

        let merged_project = &merged_projects[0];
        assert_that(&merged_project.id).is_none();

        assert_spigot_fields_are_equal(merged_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_equal(merged_project, &modrinth_project);
        assert_hangar_fields_are_equal(merged_project, &hangar_project);

        // Act 2 - Insert project
        upsert_common_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];
        assert_that(&inserted_project.id).is_some();

        assert_spigot_fields_are_equal(inserted_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_equal(inserted_project, &modrinth_project);
        assert_hangar_fields_are_equal(inserted_project, &hangar_project);

        // Act 3 - Update project
        spigot_resource.parsed_name = Some("foo-updated".to_string());
        spigot_resource.description = "foo-updated-description".to_string();
        spigot_resource.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        modrinth_project.name = "foo-updated".to_string();
        modrinth_project.description = "foo-updated-description".to_string();
        modrinth_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_modrinth_project(&context.pool, &modrinth_project).await?;

        hangar_project.name = "foo-updated".to_string();
        hangar_project.description = "foo-updated-description".to_string();
        hangar_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        let new_merged_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_equal(updated_project, &modrinth_project);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_modrinth_project_into_existing_spigot_resource() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_author, spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;

        let common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &common_projects[0]).await?;

        // Act
        let mut modrinth_project = populate_test_modrinth_project(&context.pool).await?;
        modrinth_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_modrinth_project(&context.pool, &modrinth_project).await?;

        let new_common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_common_projects[0]).await?;

        // Assert
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_equal(updated_project, &modrinth_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_spigot_resource_into_existing_modrinth_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let modrinth_project = populate_test_modrinth_project(&context.pool).await?;

        let common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &common_projects[0]).await?;

        // Act
        let (spigot_author, mut spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;
        spigot_resource.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        let new_common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_common_projects[0]).await?;

        // Assert
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_equal(updated_project, &modrinth_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_hangar_project_into_existing_modrinth_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let modrinth_project = populate_test_modrinth_project(&context.pool).await?;

        let common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &common_projects[0]).await?;

        // Act
        let mut hangar_project = populate_test_hangar_project(&context.pool).await?;
        hangar_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        let new_common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_common_projects[0]).await?;

        // Assert
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_modrinth_fields_are_equal(updated_project, &modrinth_project);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_modrinth_project_into_existing_hangar_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let hangar_project = populate_test_hangar_project(&context.pool).await?;

        let common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &common_projects[0]).await?;

        // Act
        let mut modrinth_project = populate_test_modrinth_project(&context.pool).await?;
        modrinth_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_modrinth_project(&context.pool, &modrinth_project).await?;

        let new_common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_common_projects[0]).await?;

        // Assert
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_modrinth_fields_are_equal(updated_project, &modrinth_project);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_hangar_project_into_existing_spigot_resource() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_author, spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;

        let common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &common_projects[0]).await?;

        // Act
        let mut hangar_project = populate_test_hangar_project(&context.pool).await?;
        hangar_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        let new_common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_common_projects[0]).await?;

        // Assert
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_spigot_resource_into_existing_hangar_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let hangar_project = populate_test_hangar_project(&context.pool).await?;

        let common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &common_projects[0]).await?;

        // Act
        let (spigot_author, mut spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;
        spigot_resource.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        let new_common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_common_projects[0]).await?;

        // Assert
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

        fn assert_spigot_fields_are_equal(common_project: &CommonProject, spigot_author: &SpigotAuthor, spigot_resource: &SpigotResource) {
        assert_that(&common_project.spigot).is_some();

        if let Some(spigot) = &common_project.spigot {
            assert_that(&spigot.id).is_equal_to(spigot_resource.id);
            assert_that(&spigot.name).is_equal_to(&spigot_resource.parsed_name);
            assert_that(&spigot.description).is_equal_to(&spigot_resource.description);
            assert_that(&spigot.author).is_equal_to(&spigot_author.name);
        }
    }

    fn assert_modrinth_fields_are_equal(common_project: &CommonProject, modrinth_project: &ModrinthProject) {
        assert_that(&common_project.modrinth).is_some();

        if let Some(modrinth) = &common_project.modrinth {
            assert_that(&modrinth.id).is_equal_to(&modrinth_project.id);
            assert_that(&modrinth.name).is_equal_to(&modrinth_project.name);
            assert_that(&modrinth.description).is_equal_to(&modrinth_project.description);
            assert_that(&modrinth.author).is_equal_to(&modrinth_project.author);
        }
    }

    fn assert_hangar_fields_are_equal(common_project: &CommonProject, hangar_project: &HangarProject) {
        assert_that(&common_project.hangar).is_some();

        if let Some(hangar) = &common_project.hangar {
            assert_that(&hangar.slug).is_equal_to(&hangar_project.slug);
            assert_that(&hangar.name).is_equal_to(&hangar_project.name);
            assert_that(&hangar.description).is_equal_to(&hangar_project.description);
            assert_that(&hangar.author).is_equal_to(&hangar_project.author);
        }
    }

    fn assert_spigot_fields_are_none(common_project: &CommonProject) {
        assert_that(&common_project.spigot).is_none();
    }

    fn assert_modrinth_fields_are_none(common_project: &CommonProject) {
        assert_that(&common_project.modrinth).is_none();
    }

    fn assert_hangar_fields_are_none(common_project: &CommonProject) {
        assert_that(&common_project.hangar).is_none();
    }
}