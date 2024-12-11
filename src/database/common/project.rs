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
    LatestMinecraftVersion,
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
            SearchParamsSort::LatestMinecraftVersion => "latest_minecraft_version".to_string(),
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
            "date_created"             => Ok(Self::DateCreated),
            "date_updated"             => Ok(Self::DateUpdated),
            "latest_minecraft_version" => Ok(Self::LatestMinecraftVersion),
            "downloads"                => Ok(Self::Downloads),
            "likes_and_stars"          => Ok(Self::LikesAndStars),
            "follows_and_watchers"     => Ok(Self::FollowsAndWatchers),
            _                          => Err(())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProjectSearchResult {
    pub full_count: i64,

    pub id: i32,
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub latest_minecraft_version: Option<String>,
    pub downloads: i32,
    pub likes_and_stars: i32,
    pub follows_and_watchers: i32,

    pub spigot: Option<CommonProjectSearchResultSpigot>,
    pub modrinth: Option<CommonProjectSearchResultModrinth>,
    pub hangar: Option<CommonProjectSearchResultHangar>,

    pub source_repository_host: Option<String>,
    pub source_repository_owner: Option<String>,
    pub source_repository_name: Option<String>
}

impl From<CommonProjectSearchResultEntity> for CommonProjectSearchResult {
    fn from(entity: CommonProjectSearchResultEntity) -> Self {
        let spigot = entity.spigot_id.map(|_| CommonProjectSearchResultSpigot {
            id: entity.spigot_id.expect("Spigot id should not be None"),
            slug: entity.spigot_slug.expect("Spigot slug should not be None"),
            name: entity.spigot_name,
            description: entity.spigot_description.expect("Spigot description should not be None"),
            author: entity.spigot_author.expect("Spigot author should not be None"),
            version: entity.spigot_version,
            premium: entity.spigot_premium.expect("Spigot premium should not be None"),
            icon_data: entity.spigot_icon_data
        });

        let modrinth = entity.modrinth_id.clone().map(|_| CommonProjectSearchResultModrinth {
            id: entity.modrinth_id.expect("Modrinth id should not be None"),
            slug: entity.modrinth_slug.expect("Modrinth slug should not be None"),
            name: entity.modrinth_name.expect("Modrinth name should not be None"),
            description: entity.modrinth_description.expect("Modrinth description should not be None"),
            author: entity.modrinth_author.expect("Modrinth author should not be None"),
            version: entity.modrinth_version,
            icon_url: entity.modrinth_icon_url
        });

        let hangar = entity.hangar_slug.clone().map(|_| CommonProjectSearchResultHangar {
            slug: entity.hangar_slug.expect("Hangar slug should not be None"),
            name: entity.hangar_name.expect("Hangar name should not be None"),
            description: entity.hangar_description.expect("Hangar description should not be None"),
            author: entity.hangar_author.expect("Hangar author should not be None"),
            version: entity.hangar_version,
            avatar_url: entity.hangar_avatar_url.expect("Hangar avatar url should not be None")
        });

        CommonProjectSearchResult {
            full_count: entity.full_count,

            id: entity.id,
            date_created: entity.date_created,
            date_updated: entity.date_updated,
            latest_minecraft_version: entity.latest_minecraft_version,
            downloads: entity.downloads,
            likes_and_stars: entity.likes_and_stars,
            follows_and_watchers: entity.follows_and_watchers,

            spigot,
            modrinth,
            hangar,

            source_repository_host: entity.source_repository_host,
            source_repository_owner: entity.source_repository_owner,
            source_repository_name: entity.source_repository_name
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProjectSearchResultSpigot {
    pub id: i32,
    pub slug: String,
    pub name: Option<String>,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub premium: bool,
    pub icon_data: Option<String>
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProjectSearchResultModrinth {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProjectSearchResultHangar {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub avatar_url: String
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
        assert_search_result_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);

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
        assert_search_result_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);

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
        assert_search_result_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);

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
        assert_search_result_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);

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
        assert_search_result_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);

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
        assert_search_result_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);

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
        assert_search_result_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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
        assert_search_result_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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
        assert_search_result_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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
        assert_search_result_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_search_result_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);

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
        assert_search_result_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_search_result_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);

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
        assert_search_result_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_search_result_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);

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
        assert_search_result_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_search_result_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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
        assert_search_result_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_search_result_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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
        assert_search_result_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_search_result_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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
        assert_search_result_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);
        assert_search_result_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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
        assert_search_result_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);
        assert_search_result_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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
        assert_search_result_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);
        assert_search_result_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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
        assert_search_result_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_search_result_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);
        assert_search_result_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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
        assert_search_result_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_search_result_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);
        assert_search_result_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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
        assert_search_result_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_search_result_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);
        assert_search_result_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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

        // Act 3 - Sort by latest Minecraft version order
        let params = SearchParams {
            hangar: true,
            name: true,
            sort: SearchParamsSort::LatestMinecraftVersion,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 3 - Verify results are in latest Minecraft version order
        assert_that(&search_results).has_length(3);
        assert_that(&search_results[0].latest_minecraft_version).is_some().is_equal_to("1.21.4".to_string());
        assert_that(&search_results[1].latest_minecraft_version).is_some().is_equal_to("1.16.5".to_string());
        assert_that(&search_results[2].latest_minecraft_version).is_some().is_equal_to("1.8".to_string());

        // Act 4 - Sort by downloads order
        let params = SearchParams {
            hangar: true,
            name: true,
            sort: SearchParamsSort::Downloads,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 4 - Verify results are in downloads order
        assert_that(&search_results).has_length(3);
        assert_that(&search_results[0].downloads).is_equal_to(300);
        assert_that(&search_results[1].downloads).is_equal_to(200);
        assert_that(&search_results[2].downloads).is_equal_to(100);

        // Act 5 - Sort by likes and stars order
        let params = SearchParams {
            hangar: true,
            name: true,
            sort: SearchParamsSort::LikesAndStars,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 5 - Verify results are in likes and stars order
        assert_that(&search_results).has_length(3);
        assert_that(&search_results[0].likes_and_stars).is_equal_to(300);
        assert_that(&search_results[1].likes_and_stars).is_equal_to(200);
        assert_that(&search_results[2].likes_and_stars).is_equal_to(100);

        // Act 6 - Sort by follows and watchers order
        let params = SearchParams {
            hangar: true,
            name: true,
            sort: SearchParamsSort::FollowsAndWatchers,
            ..Default::default()
        };
        let search_results = search_common_projects(&context.pool, &params).await?;

        // Assert 6 - Verify results are in follows and watchers order
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

    fn assert_search_result_spigot_fields_are_equal(spigot: &Option<CommonProjectSearchResultSpigot>, spigot_author: &SpigotAuthor, spigot_resource: &SpigotResource) {
        assert_that(spigot).is_some();

        let s = spigot.as_ref().unwrap();
        assert_that(&s.id).is_equal_to(spigot_resource.id);
        assert_that(&s.slug).is_equal_to(&spigot_resource.slug);
        assert_that(&s.name).is_equal_to(&spigot_resource.parsed_name);
        assert_that(&s.description).is_equal_to(&spigot_resource.description);
        assert_that(&s.author).is_equal_to(&spigot_author.name);
        assert_that(&s.version).is_equal_to(&spigot_resource.version_name);
        assert_that(&s.premium).is_equal_to(&spigot_resource.premium);
        assert_that(&s.icon_data).is_equal_to(&spigot_resource.icon_data);
    }

    fn assert_search_result_modrinth_fields_are_equal(modrinth: &Option<CommonProjectSearchResultModrinth>, modrinth_project: &ModrinthProject) {
        assert_that(modrinth).is_some();

        let m = modrinth.as_ref().unwrap();
        assert_that(&m.id).is_equal_to(&modrinth_project.id);
        assert_that(&m.slug).is_equal_to(&modrinth_project.slug);
        assert_that(&m.name).is_equal_to(&modrinth_project.name);
        assert_that(&m.description).is_equal_to(&modrinth_project.description);
        assert_that(&m.author).is_equal_to(&modrinth_project.author);
        assert_that(&m.version).is_equal_to(&modrinth_project.version_name);
        assert_that(&m.icon_url).is_equal_to(&modrinth_project.icon_url);
    }

    fn assert_search_result_hangar_fields_are_equal(hangar: &Option<CommonProjectSearchResultHangar>, hangar_project: &HangarProject) {
        assert_that(hangar).is_some();

        let h = hangar.as_ref().unwrap();
        assert_that(&h.slug).is_equal_to(&hangar_project.slug);
        assert_that(&h.name).is_equal_to(&hangar_project.name);
        assert_that(&h.description).is_equal_to(&hangar_project.description);
        assert_that(&h.author).is_equal_to(&hangar_project.author);
        assert_that(&h.version).is_equal_to(&hangar_project.version_name);
        assert_that(&h.avatar_url).is_equal_to(&hangar_project.avatar_url);

    }
}