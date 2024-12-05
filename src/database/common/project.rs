use crate::database::cornucopia::queries::common_project::{self, CommonProjectEntity, CommonProjectSearchResultEntity, SearchCommonProjectsParams, UpsertCommonProjectParams};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use std::str::FromStr;
use thiserror::Error;
use time::OffsetDateTime;
use time::macros::datetime;
use tracing::{info, instrument};

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProject {
    pub id: Option<i32>,

    pub spigot_id: Option<i32>,
    pub spigot_slug: Option<String>,
    pub spigot_name: Option<String>,
    pub spigot_description: Option<String>,
    pub spigot_author: Option<String>,
    pub spigot_version: Option<String>,
    pub spigot_premium: Option<bool>,

    pub modrinth_id: Option<String>,
    pub modrinth_slug: Option<String>,
    pub modrinth_name: Option<String>,
    pub modrinth_description: Option<String>,
    pub modrinth_author: Option<String>,
    pub modrinth_version: Option<String>,

    pub hangar_slug: Option<String>,
    pub hangar_name: Option<String>,
    pub hangar_description: Option<String>,
    pub hangar_author: Option<String>,
    pub hangar_version: Option<String>
}

impl From<CommonProject> for UpsertCommonProjectParams<String, String, String, String, String, String, String, String, String, String, String> {
    fn from(project: CommonProject) -> Self {
        UpsertCommonProjectParams {
            id: project.id.map(i64::from),

            spigot_id: project.spigot_id,
            spigot_name: project.spigot_name,
            spigot_description: project.spigot_description,
            spigot_author: project.spigot_author,

            modrinth_id: project.modrinth_id,
            modrinth_name: project.modrinth_name,
            modrinth_description: project.modrinth_description,
            modrinth_author: project.modrinth_author,

            hangar_slug: project.hangar_slug,
            hangar_name: project.hangar_name,
            hangar_description: project.hangar_description,
            hangar_author: project.hangar_author
        }
    }
}

impl From<CommonProjectEntity> for CommonProject {
    fn from(entity: CommonProjectEntity) -> Self {
        CommonProject {
            id: entity.id,

            spigot_id: entity.spigot_id,
            spigot_slug: entity.spigot_slug,
            spigot_name: entity.spigot_name,
            spigot_description: entity.spigot_description,
            spigot_author: entity.spigot_author,
            spigot_version: entity.spigot_version,
            spigot_premium: entity.spigot_premium,

            modrinth_id: entity.modrinth_id,
            modrinth_slug: entity.modrinth_slug,
            modrinth_name: entity.modrinth_name,
            modrinth_description: entity.modrinth_description,
            modrinth_author: entity.modrinth_author,
            modrinth_version: entity.modrinth_version,

            hangar_slug: entity.hangar_slug,
            hangar_name: entity.hangar_name,
            hangar_description: entity.hangar_description,
            hangar_author: entity.hangar_author,
            hangar_version: entity.hangar_version
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchParams {
    pub query: String,
    pub spigot: bool,
    pub modrinth: bool,
    pub hangar: bool,
    pub name: bool,
    pub description: bool,
    pub author: bool,
    pub sort: SearchParamsSort,
    pub limit: i64,
    pub offset: i64
}

impl Default for SearchParams {
    fn default() -> Self {
        SearchParams {
            query: String::default(),
            spigot: bool::default(),
            modrinth: bool::default(),
            hangar: bool::default(),
            name: bool::default(),
            description: bool::default(),
            author: bool::default(),
            sort: SearchParamsSort::default(),
            limit: 25,
            offset: i64::default()
        }
    }
}

impl From<SearchParams> for SearchCommonProjectsParams<String, String> {
    fn from(params: SearchParams) -> Self {
        SearchCommonProjectsParams {
            // Add SQL wildcard characters to both sides of the query string
            query: format!("%{0}%", params.query),
            spigot: params.spigot,
            modrinth: params.modrinth,
            hangar: params.hangar,
            name: params.name,
            description: params.description,
            author: params.author,
            sort: params.sort.into(),
            limit: params.limit,
            offset: params.offset
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum SearchParamsSort {
    DateCreated,
    DateUpdated,
    #[default]
    Downloads,
    LikesAndStars,
    FollowsAndWatchers,
}

// TODO: Re-implement this into a bidirection one-to-one String/Enum mapping
impl From<SearchParamsSort> for String {
    fn from(sort: SearchParamsSort) -> Self {
        match sort {
            SearchParamsSort::DateCreated => "date_created".to_string(),
            SearchParamsSort::DateUpdated => "date_updated".to_string(),
            SearchParamsSort::Downloads => "downloads".to_string(),
            SearchParamsSort::LikesAndStars => "likes_and_stars".to_string(),
            SearchParamsSort::FollowsAndWatchers => "follows_and_watchers".to_string()
        }
    }
}

impl FromStr for SearchParamsSort {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "date_created"         => Ok(Self::DateCreated),
            "date_updated"         => Ok(Self::DateUpdated),
            "downloads"            => Ok(Self::Downloads),
            "likes_and_stars"      => Ok(Self::LikesAndStars),
            "follows_and_watchers" => Ok(Self::FollowsAndWatchers),
            _                      => Err(())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProjectSearchResult {
    pub full_count: i64,

    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub latest_minecraft_version: Option<String>,
    pub downloads: i32,
    pub likes_and_stars: i32,
    pub follows_and_watchers: i32,

    pub project: CommonProject,

    pub source_repository_host: Option<String>,
    pub source_repository_owner: Option<String>,
    pub source_repository_name: Option<String>
}

impl From<CommonProjectSearchResultEntity> for CommonProjectSearchResult {
    fn from(entity: CommonProjectSearchResultEntity) -> Self {
        CommonProjectSearchResult {
            full_count: entity.full_count,

            date_created: entity.date_created,
            date_updated: entity.date_updated,
            latest_minecraft_version: entity.latest_minecraft_version,
            downloads: entity.downloads,
            likes_and_stars: entity.likes_and_stars,
            follows_and_watchers: entity.follows_and_watchers,

            project: CommonProject {
                id: entity.id,

                spigot_id: entity.spigot_id,
                spigot_slug: entity.spigot_slug,
                spigot_name: entity.spigot_name,
                spigot_description: entity.spigot_description,
                spigot_author: entity.spigot_author,
                spigot_version: entity.spigot_version,
                spigot_premium: entity.spigot_premium,

                modrinth_id: entity.modrinth_id,
                modrinth_slug: entity.modrinth_slug,
                modrinth_name: entity.modrinth_name,
                modrinth_description: entity.modrinth_description,
                modrinth_author: entity.modrinth_author,
                modrinth_version: entity.modrinth_version,

                hangar_slug: entity.hangar_slug,
                hangar_name: entity.hangar_name,
                hangar_description: entity.hangar_description,
                hangar_author: entity.hangar_author,
                hangar_version: entity.hangar_version,
            },

            source_repository_host: entity.source_repository_host,
            source_repository_owner: entity.source_repository_owner,
            source_repository_name: entity.source_repository_name
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

#[instrument(
    level = "info",
    skip(db_pool)
)]
pub async fn search_common_projects(db_pool: &Pool, params: &SearchParams) -> Result<Vec<CommonProjectSearchResult>> {
    let db_client = db_pool.get().await?;

    let entities = common_project::search_common_projects()
        .params(&db_client, &params.clone().into())
        .all()
        .await?;

    let projects = entities.into_iter().map(|x| x.into()).collect();

    Ok(projects)
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
    use crate::database::hangar::project::test::{populate_test_hangar_project, populate_test_hangar_projects};

    use crate::database::modrinth::project::{ModrinthProject, upsert_modrinth_project};
    use crate::database::modrinth::project::test::{populate_test_modrinth_project, populate_test_modrinth_projects};

    use crate::database::spigot::author::SpigotAuthor;
    use crate::database::spigot::resource::{SpigotResource, upsert_spigot_resource};
    use crate::database::spigot::resource::test::{populate_test_spigot_author_and_resource, populate_test_spigot_authors_and_resources};
    use crate::database::spigot::test::SPIGOT_BASE64_TEST_ICON_DATA;

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
            source_url: Some("https://gitlab.com/Frumple/bar".to_string()),
            source_repository_host: Some("gitlab.com".to_string()),
            source_repository_owner: Some("Frumple".to_string()),
            source_repository_name: Some("bar".to_string())
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

    #[tokio::test]
    #[named]
    async fn should_search_spigot_resources() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_authors, spigot_resources) = populate_test_spigot_authors_and_resources(&context.pool).await?;
        let _modrinth_projects = populate_test_modrinth_projects(&context.pool).await?;
        let _hangar_projects = populate_test_hangar_projects(&context.pool).await?;

        let merged_projects = get_merged_common_projects(&context.pool, None).await?;
        for merged_project in merged_projects {
            upsert_common_project(&context.pool, &merged_project).await?;
        }

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_spigot_resource(&search_results[0], &spigot_resources[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), None, None);
        assert_spigot_fields_are_equal(&search_results[0].project, &spigot_authors[0], &spigot_resources[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_spigot_resource(&search_results[0], &spigot_resources[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), None, None);
        assert_spigot_fields_are_equal(&search_results[0].project, &spigot_authors[0], &spigot_resources[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            spigot: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_spigot_resource(&search_results[0], &spigot_resources[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), None, None);
        assert_spigot_fields_are_equal(&search_results[0].project, &spigot_authors[0], &spigot_resources[0]);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_search_modrinth_projects() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (_spigot_authors, _spigot_resources) = populate_test_spigot_authors_and_resources(&context.pool).await?;
        let modrinth_projects = populate_test_modrinth_projects(&context.pool).await?;
        let _hangar_projects = populate_test_hangar_projects(&context.pool).await?;

        let merged_projects = get_merged_common_projects(&context.pool, None).await?;
        for merged_project in merged_projects {
            upsert_common_project(&context.pool, &merged_project).await?;
        }

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            modrinth: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_modrinth_project(&search_results[0], &modrinth_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, Some(&modrinth_projects[0]), None);
        assert_modrinth_fields_are_equal(&search_results[0].project, &modrinth_projects[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            modrinth: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_modrinth_project(&search_results[0], &modrinth_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, Some(&modrinth_projects[0]), None);
        assert_modrinth_fields_are_equal(&search_results[0].project, &modrinth_projects[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            modrinth: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_modrinth_project(&search_results[0], &modrinth_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, Some(&modrinth_projects[0]), None);
        assert_modrinth_fields_are_equal(&search_results[0].project, &modrinth_projects[0]);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_search_hangar_projects() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (_spigot_authors, _spigot_resources) = populate_test_spigot_authors_and_resources(&context.pool).await?;
        let _modrinth_projects = populate_test_modrinth_projects(&context.pool).await?;
        let hangar_projects = populate_test_hangar_projects(&context.pool).await?;

        let merged_projects = get_merged_common_projects(&context.pool, None).await?;
        for merged_project in merged_projects {
            upsert_common_project(&context.pool, &merged_project).await?;
        }

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            hangar: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, None, Some(&hangar_projects[0]));
        assert_hangar_fields_are_equal(&search_results[0].project, &hangar_projects[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            hangar: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, None, Some(&hangar_projects[0]));
        assert_hangar_fields_are_equal(&search_results[0].project, &hangar_projects[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            hangar: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, None, Some(&hangar_projects[0]));
        assert_hangar_fields_are_equal(&search_results[0].project, &hangar_projects[0]);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_search_spigot_resources_and_modrinth_projects() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_authors, spigot_resources) = populate_test_spigot_authors_and_resources(&context.pool).await?;
        let modrinth_projects = populate_test_modrinth_projects(&context.pool).await?;
        let _hangar_projects = populate_test_hangar_projects(&context.pool).await?;

        let merged_projects = get_merged_common_projects(&context.pool, None).await?;
        for merged_project in merged_projects {
            upsert_common_project(&context.pool, &merged_project).await?;
        }

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            modrinth: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_modrinth_project(&search_results[0], &modrinth_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), Some(&modrinth_projects[0]), None);
        assert_spigot_fields_are_equal(&search_results[0].project, &spigot_authors[0], &spigot_resources[0]);
        assert_modrinth_fields_are_equal(&search_results[0].project, &modrinth_projects[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            modrinth: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_modrinth_project(&search_results[0], &modrinth_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), Some(&modrinth_projects[0]), None);
        assert_spigot_fields_are_equal(&search_results[0].project, &spigot_authors[0], &spigot_resources[0]);
        assert_modrinth_fields_are_equal(&search_results[0].project, &modrinth_projects[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            spigot: true,
            modrinth: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_modrinth_project(&search_results[0], &modrinth_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), Some(&modrinth_projects[0]), None);
        assert_spigot_fields_are_equal(&search_results[0].project, &spigot_authors[0], &spigot_resources[0]);
        assert_modrinth_fields_are_equal(&search_results[0].project, &modrinth_projects[0]);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_search_spigot_resources_and_hangar_projects() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_authors, spigot_resources) = populate_test_spigot_authors_and_resources(&context.pool).await?;
        let _modrinth_projects = populate_test_modrinth_projects(&context.pool).await?;
        let hangar_projects = populate_test_hangar_projects(&context.pool).await?;

        let merged_projects = get_merged_common_projects(&context.pool, None).await?;
        for merged_project in merged_projects {
            upsert_common_project(&context.pool, &merged_project).await?;
        }

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            hangar: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), None, Some(&hangar_projects[0]));
        assert_spigot_fields_are_equal(&search_results[0].project, &spigot_authors[0], &spigot_resources[0]);
        assert_hangar_fields_are_equal(&search_results[0].project, &hangar_projects[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            hangar: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), None, Some(&hangar_projects[0]));
        assert_spigot_fields_are_equal(&search_results[0].project, &spigot_authors[0], &spigot_resources[0]);
        assert_hangar_fields_are_equal(&search_results[0].project, &hangar_projects[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            spigot: true,
            hangar: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), None, Some(&hangar_projects[0]));
        assert_spigot_fields_are_equal(&search_results[0].project, &spigot_authors[0], &spigot_resources[0]);
        assert_hangar_fields_are_equal(&search_results[0].project, &hangar_projects[0]);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_search_modrinth_projects_and_hangar_projects() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (_spigot_authors, _spigot_resources) = populate_test_spigot_authors_and_resources(&context.pool).await?;
        let modrinth_projects = populate_test_modrinth_projects(&context.pool).await?;
        let hangar_projects = populate_test_hangar_projects(&context.pool).await?;

        let merged_projects = get_merged_common_projects(&context.pool, None).await?;
        for merged_project in merged_projects {
            upsert_common_project(&context.pool, &merged_project).await?;
        }

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            modrinth: true,
            hangar: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, Some(&modrinth_projects[0]), Some(&hangar_projects[0]));
        assert_modrinth_fields_are_equal(&search_results[0].project, &modrinth_projects[0]);
        assert_hangar_fields_are_equal(&search_results[0].project, &hangar_projects[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            modrinth: true,
            hangar: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, Some(&modrinth_projects[0]), Some(&hangar_projects[0]));
        assert_modrinth_fields_are_equal(&search_results[0].project, &modrinth_projects[0]);
        assert_hangar_fields_are_equal(&search_results[0].project, &hangar_projects[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            modrinth: true,
            hangar: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, Some(&modrinth_projects[0]), Some(&hangar_projects[0]));
        assert_modrinth_fields_are_equal(&search_results[0].project, &modrinth_projects[0]);
        assert_hangar_fields_are_equal(&search_results[0].project, &hangar_projects[0]);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_search_all_resources_and_projects() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_authors, spigot_resources) = populate_test_spigot_authors_and_resources(&context.pool).await?;
        let modrinth_projects = populate_test_modrinth_projects(&context.pool).await?;
        let hangar_projects = populate_test_hangar_projects(&context.pool).await?;

        let merged_projects = get_merged_common_projects(&context.pool, None).await?;
        for merged_project in merged_projects {
            upsert_common_project(&context.pool, &merged_project).await?;
        }

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            modrinth: true,
            hangar: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), Some(&modrinth_projects[0]), Some(&hangar_projects[0]));
        assert_spigot_fields_are_equal(&search_results[0].project, &spigot_authors[0], &spigot_resources[0]);
        assert_modrinth_fields_are_equal(&search_results[0].project, &modrinth_projects[0]);
        assert_hangar_fields_are_equal(&search_results[0].project, &hangar_projects[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            modrinth: true,
            hangar: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), Some(&modrinth_projects[0]), Some(&hangar_projects[0]));
        assert_spigot_fields_are_equal(&search_results[0].project, &spigot_authors[0], &spigot_resources[0]);
        assert_modrinth_fields_are_equal(&search_results[0].project, &modrinth_projects[0]);
        assert_hangar_fields_are_equal(&search_results[0].project, &hangar_projects[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            spigot: true,
            modrinth: true,
            hangar: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), Some(&modrinth_projects[0]), Some(&hangar_projects[0]));
        assert_spigot_fields_are_equal(&search_results[0].project, &spigot_authors[0], &spigot_resources[0]);
        assert_modrinth_fields_are_equal(&search_results[0].project, &modrinth_projects[0]);
        assert_hangar_fields_are_equal(&search_results[0].project, &hangar_projects[0]);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_return_search_results_in_correct_order() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let _hangar_projects = populate_test_hangar_projects(&context.pool).await?;

        let merged_projects = get_merged_common_projects(&context.pool, None).await?;
        for merged_project in merged_projects {
            upsert_common_project(&context.pool, &merged_project).await?;
        }

        // Act 1 - Sort by date_created order
        let params = SearchParams {
            hangar: true,
            name: true,
            sort: SearchParamsSort::DateCreated,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 1 - Verify results are in date_created order
        assert_that(&search_results).has_length(3);
        assert_that(&search_results[0].date_created).is_equal_to(datetime!(2022-01-03 0:00 UTC));
        assert_that(&search_results[1].date_created).is_equal_to(datetime!(2022-01-02 0:00 UTC));
        assert_that(&search_results[2].date_created).is_equal_to(datetime!(2022-01-01 0:00 UTC));

        // Act 2 - Sort by date_updated order
        let params = SearchParams {
            hangar: true,
            name: true,
            sort: SearchParamsSort::DateUpdated,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 2 - Verify results are in date_updated order
        assert_that(&search_results).has_length(3);
        assert_that(&search_results[0].date_updated).is_equal_to(datetime!(2022-02-03 0:00 UTC));
        assert_that(&search_results[1].date_updated).is_equal_to(datetime!(2022-02-02 0:00 UTC));
        assert_that(&search_results[2].date_updated).is_equal_to(datetime!(2022-02-01 0:00 UTC));

        // Act 3 - Sort by downloads order
        let params = SearchParams {
            hangar: true,
            name: true,
            sort: SearchParamsSort::Downloads,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 3 - Verify results are in downloads order
        assert_that(&search_results).has_length(3);
        assert_that(&search_results[0].downloads).is_equal_to(300);
        assert_that(&search_results[1].downloads).is_equal_to(200);
        assert_that(&search_results[2].downloads).is_equal_to(100);

        // Act 4 - Sort by likes and stars order
        let params = SearchParams {
            hangar: true,
            name: true,
            sort: SearchParamsSort::LikesAndStars,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 4 - Verify results are in likes and stars order
        assert_that(&search_results).has_length(3);
        assert_that(&search_results[0].likes_and_stars).is_equal_to(300);
        assert_that(&search_results[1].likes_and_stars).is_equal_to(200);
        assert_that(&search_results[2].likes_and_stars).is_equal_to(100);

        // Act 5 - Sort by follows and watchers order
        let params = SearchParams {
            hangar: true,
            name: true,
            sort: SearchParamsSort::FollowsAndWatchers,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 5 - Verify results are in follows and watchers order
        assert_that(&search_results).has_length(3);
        assert_that(&search_results[0].follows_and_watchers).is_equal_to(300);
        assert_that(&search_results[1].follows_and_watchers).is_equal_to(200);
        assert_that(&search_results[2].follows_and_watchers).is_equal_to(100);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    fn assert_dates_are_equal_to_spigot_resource(search_result: &CommonProjectSearchResult, spigot_resource: &SpigotResource) {
        assert_that(&search_result.date_created).is_equal_to(spigot_resource.date_created);
        assert_that(&search_result.date_updated).is_equal_to(spigot_resource.date_updated);
    }

    fn assert_dates_are_equal_to_modrinth_project(search_result: &CommonProjectSearchResult, modrinth_project: &ModrinthProject) {
        assert_that(&search_result.date_created).is_equal_to(modrinth_project.date_created);
        assert_that(&search_result.date_updated).is_equal_to(modrinth_project.date_updated);
    }

    fn assert_dates_are_equal_to_hangar_project(search_result: &CommonProjectSearchResult, hangar_project: &HangarProject) {
        assert_that(&search_result.date_created).is_equal_to(hangar_project.date_created);
        assert_that(&search_result.date_updated).is_equal_to(hangar_project.date_updated);
    }

    fn assert_stats_are_equal_to_projects(search_result: &CommonProjectSearchResult, spigot_resource: Option<&SpigotResource>, modrinth_project: Option<&ModrinthProject>, hangar_project: Option<&HangarProject>) {
        let mut expected_downloads = 0;
        let mut expected_likes_and_stars = 0;
        let mut expected_follows_and_watchers = 0;

        if let Some(resource) = spigot_resource {
            expected_downloads += resource.downloads;
            expected_likes_and_stars += resource.likes;
        }

        if let Some(project) = modrinth_project {
            expected_downloads += project.downloads;
            expected_follows_and_watchers += project.follows;
        }

        if let Some(project) = hangar_project {
            expected_downloads += project.downloads;
            expected_likes_and_stars += project.stars;
            expected_follows_and_watchers += project.watchers;
        }

        assert_that(&search_result.downloads).is_equal_to(expected_downloads);
        assert_that(&search_result.likes_and_stars).is_equal_to(expected_likes_and_stars);
        assert_that(&search_result.follows_and_watchers).is_equal_to(expected_follows_and_watchers);
    }

    fn assert_spigot_fields_are_equal(common_project: &CommonProject, spigot_author: &SpigotAuthor, spigot_resource: &SpigotResource) {
        assert_that(&common_project.spigot_id).is_some().is_equal_to(spigot_resource.id);
        assert_that(&common_project.spigot_name).is_equal_to(&spigot_resource.parsed_name);
        assert_that(&common_project.spigot_description).is_some().is_equal_to(&spigot_resource.description);
        assert_that(&common_project.spigot_author).is_some().is_equal_to(&spigot_author.name);
    }

    fn assert_modrinth_fields_are_equal(common_project: &CommonProject, modrinth_project: &ModrinthProject) {
        assert_that(&common_project.modrinth_id).is_some().is_equal_to(&modrinth_project.id);
        assert_that(&common_project.modrinth_name).is_some().is_equal_to(&modrinth_project.name);
        assert_that(&common_project.modrinth_description).is_some().is_equal_to(&modrinth_project.description);
        assert_that(&common_project.modrinth_author).is_some().is_equal_to(&modrinth_project.author);
    }

    fn assert_hangar_fields_are_equal(common_project: &CommonProject, hangar_project: &HangarProject) {
        assert_that(&common_project.hangar_slug).is_some().is_equal_to(&hangar_project.slug);
        assert_that(&common_project.hangar_name).is_some().is_equal_to(&hangar_project.name);
        assert_that(&common_project.hangar_description).is_some().is_equal_to(&hangar_project.description);
        assert_that(&common_project.hangar_author).is_some().is_equal_to(&hangar_project.author);
    }

    fn assert_spigot_fields_are_none(common_project: &CommonProject) {
        assert_that(&common_project.spigot_id).is_none();
        assert_that(&common_project.spigot_name).is_none();
        assert_that(&common_project.spigot_description).is_none();
        assert_that(&common_project.spigot_author).is_none();
    }

    fn assert_modrinth_fields_are_none(common_project: &CommonProject) {
        assert_that(&common_project.modrinth_id).is_none();
        assert_that(&common_project.modrinth_name).is_none();
        assert_that(&common_project.modrinth_description).is_none();
        assert_that(&common_project.modrinth_author).is_none();
    }

    fn assert_hangar_fields_are_none(common_project: &CommonProject) {
        assert_that(&common_project.hangar_slug).is_none();
        assert_that(&common_project.hangar_name).is_none();
        assert_that(&common_project.hangar_description).is_none();
        assert_that(&common_project.hangar_author).is_none();
    }
}