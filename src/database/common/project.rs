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
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub spigot_id: Option<i32>,
    pub spigot_name: Option<String>,
    pub spigot_author: Option<String>,
    pub spigot_tag: Option<String>,
    pub hangar_slug: Option<String>,
    pub hangar_name: Option<String>,
    pub hangar_owner: Option<String>,
    pub hangar_description: Option<String>
}

impl From<CommonProject> for UpsertCommonProjectParams<String, String, String, String, String, String, String> {
    fn from(project: CommonProject) -> Self {
        UpsertCommonProjectParams {
            id: project.id.map(i64::from),
            date_created: project.date_created,
            date_updated: project.date_updated,
            spigot_id: project.spigot_id,
            spigot_name: project.spigot_name,
            spigot_author: project.spigot_author,
            spigot_tag: project.spigot_tag,
            hangar_slug: project.hangar_slug,
            hangar_name: project.hangar_name,
            hangar_owner: project.hangar_owner,
            hangar_description: project.hangar_description
        }
    }
}

impl From<CommonProjectEntity> for CommonProject {
    fn from(entity: CommonProjectEntity) -> Self {
        CommonProject {
            id: entity.id,
            date_created: entity.date_created,
            date_updated: entity.date_updated,
            spigot_id: entity.spigot_id,
            spigot_name: entity.spigot_name,
            spigot_author: entity.spigot_author,
            spigot_tag: entity.spigot_tag,
            hangar_slug: entity.hangar_slug,
            hangar_name: entity.hangar_name,
            hangar_owner: entity.hangar_owner,
            hangar_description: entity.hangar_description
        }
    }
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
    let update_date = match update_date_later_than {
        Some(date) => date,
        None => datetime!(2000-01-01 0:00 UTC)
    };

    let entities = common_project::get_merged_common_projects()
        .bind(&db_client, &update_date)
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
    use crate::database::hangar::project::{HangarProject, upsert_hangar_project};
    use crate::database::hangar::project::test::populate_test_hangar_project;

    use crate::database::spigot::author::SpigotAuthor;

    use crate::database::spigot::resource::{SpigotResource, upsert_spigot_resource};
    use crate::database::spigot::resource::test::{populate_test_spigot_author_and_resource, test_resources};

    use crate::test::DatabaseTestContext;

    use ::function_name::named;
    use speculoos::prelude::*;

    #[tokio::test]
    #[named]
    async fn should_only_merge_after_provided_update_date() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        // Create two resources with update_dates 2021-01-01 and 2022-01-01 respectively.
        let (spigot_author, _spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;

        let spigot_resource2 = &test_resources()[1];
        upsert_spigot_resource(&context.pool, spigot_resource2).await?;

        // Act
        // Get merged common projects with update date greater than 2021-07-01.
        let merged_projects = get_merged_common_projects(&context.pool, Some(datetime!(2021-07-01 0:00 UTC))).await?;

        // Assert
        // Only the resource with update_date 2022-01-01 should be merged.
        assert_that(&merged_projects).has_length(1);

        let merged_project = &merged_projects[0];
        assert_that(&merged_project.id).is_none();
        assert_dates_are_equal_to_spigot_resource(merged_project, spigot_resource2);

        assert_spigot_fields_are_equal(merged_project, &spigot_author, spigot_resource2);
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
        assert_dates_are_equal_to_spigot_resource(merged_project, &spigot_resource);

        assert_spigot_fields_are_equal(merged_project, &spigot_author, &spigot_resource);
        assert_hangar_fields_are_none(merged_project);

        // Act 2 - Insert project
        upsert_common_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];
        assert_that(&inserted_project.id).is_some();
        assert_dates_are_equal_to_spigot_resource(inserted_project, &spigot_resource);

        assert_spigot_fields_are_equal(inserted_project, &spigot_author, &spigot_resource);
        assert_hangar_fields_are_none(inserted_project);

        // Act 3 - Update project
        spigot_resource.parsed_name = Some("foo-updated".to_string());
        spigot_resource.tag = "foo-updated-tag".to_string();
        spigot_resource.update_date = datetime!(2021-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        let new_merged_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();
        assert_dates_are_equal_to_spigot_resource(updated_project, &spigot_resource);

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
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
        assert_dates_are_equal_to_hangar_project(merged_project, &hangar_project);

        assert_spigot_fields_are_none(merged_project);
        assert_hangar_fields_are_equal(merged_project, &hangar_project);

        // Act 2 - Insert project
        upsert_common_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];
        assert_that(&inserted_project.id).is_some();
        assert_dates_are_equal_to_hangar_project(inserted_project, &hangar_project);

        assert_spigot_fields_are_none(inserted_project);
        assert_hangar_fields_are_equal(inserted_project, &hangar_project);

        // Act 3 - Update project
        hangar_project.name = "foo-updated".to_string();
        hangar_project.description = "foo-updated-description".to_string();
        hangar_project.last_updated = datetime!(2021-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        let new_merged_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();
        assert_dates_are_equal_to_hangar_project(updated_project, &hangar_project);

        assert_spigot_fields_are_none(updated_project);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_merge_insert_update_spigot_resource_and_hangar_project() -> Result<()> {
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
        assert_dates_are_equal_to_spigot_resource(merged_project, &spigot_resource);

        assert_spigot_fields_are_equal(merged_project, &spigot_author, &spigot_resource);
        assert_hangar_fields_are_equal(merged_project, &hangar_project);

        // Act 2 - Insert project
        upsert_common_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];
        assert_that(&inserted_project.id).is_some();
        assert_dates_are_equal_to_spigot_resource(merged_project, &spigot_resource);

        assert_spigot_fields_are_equal(inserted_project, &spigot_author, &spigot_resource);
        assert_hangar_fields_are_equal(inserted_project, &hangar_project);

        // Act 3 - Update project
        spigot_resource.parsed_name = Some("foo-updated".to_string());
        spigot_resource.tag = "foo-updated-tag".to_string();
        spigot_resource.update_date = datetime!(2021-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        hangar_project.name = "foo-updated".to_string();
        hangar_project.description = "foo-updated-description".to_string();
        hangar_project.last_updated = datetime!(2021-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        let new_merged_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();
        assert_dates_are_equal_to_spigot_resource(updated_project, &spigot_resource);

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
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
        hangar_project.last_updated = datetime!(2021-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        let new_common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_common_projects[0]).await?;

        // Assert
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();
        assert_dates_are_equal_to_hangar_project(updated_project, &hangar_project);

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
        spigot_resource.update_date = datetime!(2021-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        let new_common_projects = get_merged_common_projects(&context.pool, None).await?;
        upsert_common_project(&context.pool, &new_common_projects[0]).await?;

        // Assert
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];
        assert_that(&updated_project.id).is_some();
        assert_dates_are_equal_to_spigot_resource(updated_project, &spigot_resource);

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    fn assert_dates_are_equal_to_spigot_resource(common_project: &CommonProject, spigot_resource: &SpigotResource) {
        assert_that(&common_project.date_created).is_equal_to(spigot_resource.release_date);
        assert_that(&common_project.date_updated).is_equal_to(spigot_resource.update_date);
    }

    fn assert_dates_are_equal_to_hangar_project(common_project: &CommonProject, hangar_project: &HangarProject) {
        assert_that(&common_project.date_created).is_equal_to(hangar_project.created_at);
        assert_that(&common_project.date_updated).is_equal_to(hangar_project.last_updated);
    }

    fn assert_spigot_fields_are_equal(common_project: &CommonProject, spigot_author: &SpigotAuthor, spigot_resource: &SpigotResource) {
        assert_that(&common_project.spigot_id).is_some().is_equal_to(spigot_resource.id);
        assert_that(&common_project.spigot_name).is_equal_to(&spigot_resource.parsed_name);
        assert_that(&common_project.spigot_author).is_some().is_equal_to(&spigot_author.name);
        assert_that(&common_project.spigot_tag).is_some().is_equal_to(&spigot_resource.tag);
    }

    fn assert_hangar_fields_are_equal(common_project: &CommonProject, hangar_project: &HangarProject) {
        assert_that(&common_project.hangar_slug).is_some().is_equal_to(&hangar_project.slug);
        assert_that(&common_project.hangar_name).is_some().is_equal_to(&hangar_project.name);
        assert_that(&common_project.hangar_owner).is_some().is_equal_to(&hangar_project.owner);
        assert_that(&common_project.hangar_description).is_some().is_equal_to(&hangar_project.description);
    }

    fn assert_spigot_fields_are_none(common_project: &CommonProject) {
        assert_that(&common_project.spigot_id).is_none();
        assert_that(&common_project.spigot_name).is_none();
        assert_that(&common_project.spigot_author).is_none();
        assert_that(&common_project.spigot_tag).is_none();
    }

    fn assert_hangar_fields_are_none(common_project: &CommonProject) {
        assert_that(&common_project.hangar_slug).is_none();
        assert_that(&common_project.hangar_name).is_none();
        assert_that(&common_project.hangar_owner).is_none();
        assert_that(&common_project.hangar_description).is_none();
    }
}