use crate::database::cornucopia::queries::final_project::{self, FinalProjectEntity, UpsertFinalProjectParams};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use thiserror::Error;
use time::OffsetDateTime;
use tracing::{info, instrument};

#[derive(Clone, Debug, PartialEq)]
pub struct FinalProject {
    pub id: Option<i32>,
    pub release_date: OffsetDateTime,
    pub update_date: OffsetDateTime,
    pub spigot_id: Option<i32>,
    pub spigot_name: Option<String>,
    pub spigot_author: Option<String>,
    pub spigot_tag: Option<String>,
    pub hangar_slug: Option<String>,
    pub hangar_name: Option<String>,
    pub hangar_owner: Option<String>,
    pub hangar_description: Option<String>
}

impl From<FinalProject> for UpsertFinalProjectParams<String, String, String, String, String, String, String> {
    fn from(project: FinalProject) -> Self {
        UpsertFinalProjectParams {
            id: project.id.map(i64::from),
            release_date: project.release_date,
            update_date: project.update_date,
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

impl From<FinalProjectEntity> for FinalProject {
    fn from(entity: FinalProjectEntity) -> Self {
        FinalProject {
            id: entity.id,
            release_date: entity.release_date,
            update_date: entity.update_date,
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
enum FinalProjectError {
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
pub async fn get_merged_projects(db_pool: &Pool) -> Result<Vec<FinalProject>> {
    let db_client = db_pool.get().await?;

    let entities = final_project::get_merged_projects()
        .bind(&db_client)
        .all()
        .await?;

    let projects = entities.into_iter().map(|x| x.into()).collect();

    Ok(projects)
}

#[instrument(
    level = "info",
    skip(db_pool, final_projects)
)]
pub async fn upsert_final_projects(db_pool: &Pool, final_projects: &Vec<FinalProject>) -> Result<()> {
    let mut count = 0;

    for project in final_projects {
        upsert_final_project(db_pool, project).await?;
        count += 1;
    }

    info!("Final projects merged: {}", count);

    Ok(())
}

#[instrument(
    level = "debug",
    skip(db_pool)
)]
pub async fn upsert_final_project(db_pool: &Pool, project: &FinalProject) -> Result<()> {
    let db_client = db_pool.get().await?;
    let id = project.id;

    let db_result = final_project::upsert_final_project()
        .params(&db_client, &project.clone().into())
        .await;

    match db_result {
        Ok(_) => Ok(()),
        Err(err) => Err(
            FinalProjectError::DatabaseQueryFailed {
                id,
                source: err.into()
            }.into()
        )
    }
}

pub async fn get_final_projects(db_pool: &Pool) -> Result<Vec<FinalProject>> {
    let db_client = db_pool.get().await?;

    let entities = final_project::get_final_projects()
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
    use crate::database::spigot::resource::test::populate_test_spigot_author_and_resource;

    use crate::test::DatabaseTestContext;

    use ::function_name::named;
    use speculoos::prelude::*;
    use time::macros::datetime;

    #[tokio::test]
    #[named]
    async fn should_merge_insert_update_spigot_resource() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_author, mut spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;

        // Act 1 - Get merged project
        let merged_projects = get_merged_projects(&context.pool).await?;

        // Assert 1 - Verify merged project
        assert_that(&merged_projects).has_length(1);

        let merged_project = &merged_projects[0];
        assert_that(&merged_project.id).is_none();
        assert_dates_are_equal_to_spigot_resource(merged_project, &spigot_resource);

        assert_spigot_fields_are_equal(merged_project, &spigot_author, &spigot_resource);
        assert_hangar_fields_are_none(merged_project);

        // Act 2 - Insert project
        upsert_final_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_final_projects(&context.pool).await?;

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

        let new_merged_projects = get_merged_projects(&context.pool).await?;
        upsert_final_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_final_projects(&context.pool).await?;

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
        let merged_projects = get_merged_projects(&context.pool).await?;

        // Assert 1 - Verify merged project
        assert_that(&merged_projects).has_length(1);

        let merged_project = &merged_projects[0];
        assert_that(&merged_project.id).is_none();
        assert_dates_are_equal_to_hangar_project(merged_project, &hangar_project);

        assert_spigot_fields_are_none(merged_project);
        assert_hangar_fields_are_equal(merged_project, &hangar_project);

        // Act 2 - Insert project
        upsert_final_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_final_projects(&context.pool).await?;

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

        let new_merged_projects = get_merged_projects(&context.pool).await?;
        upsert_final_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_final_projects(&context.pool).await?;

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
        let merged_projects = get_merged_projects(&context.pool).await?;

        // Assert 1 - Verify merged project
        assert_that(&merged_projects).has_length(1);

        let merged_project = &merged_projects[0];
        assert_that(&merged_project.id).is_none();
        assert_dates_are_equal_to_spigot_resource(merged_project, &spigot_resource);

        assert_spigot_fields_are_equal(merged_project, &spigot_author, &spigot_resource);
        assert_hangar_fields_are_equal(merged_project, &hangar_project);

        // Act 2 - Insert project
        upsert_final_project(&context.pool, merged_project).await?;

        // Assert 2 - Verify project was inserted
        let inserted_projects = get_final_projects(&context.pool).await?;

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

        let new_merged_projects = get_merged_projects(&context.pool).await?;
        upsert_final_project(&context.pool, &new_merged_projects[0]).await?;

        // Assert 3 - Verify project was updated
        let updated_projects = get_final_projects(&context.pool).await?;

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

        let final_projects = get_merged_projects(&context.pool).await?;
        upsert_final_project(&context.pool, &final_projects[0]).await?;

        // Act
        let mut hangar_project = populate_test_hangar_project(&context.pool).await?;
        hangar_project.last_updated = datetime!(2021-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        let new_final_projects = get_merged_projects(&context.pool).await?;
        upsert_final_project(&context.pool, &new_final_projects[0]).await?;

        // Assert
        let updated_projects = get_final_projects(&context.pool).await?;

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

        let final_projects = get_merged_projects(&context.pool).await?;
        upsert_final_project(&context.pool, &final_projects[0]).await?;

        // Act
        let (spigot_author, mut spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;
        spigot_resource.update_date = datetime!(2021-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        let new_final_projects = get_merged_projects(&context.pool).await?;
        upsert_final_project(&context.pool, &new_final_projects[0]).await?;

        // Assert
        let updated_projects = get_final_projects(&context.pool).await?;

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

    fn assert_dates_are_equal_to_spigot_resource(final_project: &FinalProject, spigot_resource: &SpigotResource) {
        assert_that(&final_project.release_date).is_equal_to(spigot_resource.release_date);
        assert_that(&final_project.update_date).is_equal_to(spigot_resource.update_date);
    }

    fn assert_dates_are_equal_to_hangar_project(final_project: &FinalProject, hangar_project: &HangarProject) {
        assert_that(&final_project.release_date).is_equal_to(hangar_project.created_at);
        assert_that(&final_project.update_date).is_equal_to(hangar_project.last_updated);
    }

    fn assert_spigot_fields_are_equal(final_project: &FinalProject, spigot_author: &SpigotAuthor, spigot_resource: &SpigotResource) {
        assert_that(&final_project.spigot_id).is_some().is_equal_to(spigot_resource.id);
        assert_that(&final_project.spigot_name).is_equal_to(&spigot_resource.parsed_name);
        assert_that(&final_project.spigot_author).is_some().is_equal_to(&spigot_author.name);
        assert_that(&final_project.spigot_tag).is_some().is_equal_to(&spigot_resource.tag);
    }

    fn assert_hangar_fields_are_equal(final_project: &FinalProject, hangar_project: &HangarProject) {
        assert_that(&final_project.hangar_slug).is_some().is_equal_to(&hangar_project.slug);
        assert_that(&final_project.hangar_name).is_some().is_equal_to(&hangar_project.name);
        assert_that(&final_project.hangar_owner).is_some().is_equal_to(&hangar_project.owner);
        assert_that(&final_project.hangar_description).is_some().is_equal_to(&hangar_project.description);
    }

    fn assert_spigot_fields_are_none(final_project: &FinalProject) {
        assert_that(&final_project.spigot_id).is_none();
        assert_that(&final_project.spigot_name).is_none();
        assert_that(&final_project.spigot_author).is_none();
        assert_that(&final_project.spigot_tag).is_none();
    }

    fn assert_hangar_fields_are_none(final_project: &FinalProject) {
        assert_that(&final_project.hangar_slug).is_none();
        assert_that(&final_project.hangar_name).is_none();
        assert_that(&final_project.hangar_owner).is_none();
        assert_that(&final_project.hangar_description).is_none();
    }
}