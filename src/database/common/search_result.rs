use crate::database::cornucopia::queries::search_result::{self, SearchResultEntity, SearchProjectsParams};
use crate::database::source_repository::SourceRepository;

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use std::str::FromStr;
use time::OffsetDateTime;
use tracing::instrument;

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

impl From<SearchParams> for SearchProjectsParams<String, String> {
    fn from(params: SearchParams) -> Self {
        SearchProjectsParams {
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
pub struct SearchResult {
    pub full_count: i64,

    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub latest_minecraft_version: Option<String>,
    pub downloads: i32,
    pub likes_and_stars: i32,
    pub follows_and_watchers: i32,

    pub spigot: Option<SearchResultSpigot>,
    pub modrinth: Option<SearchResultModrinth>,
    pub hangar: Option<SearchResultHangar>,
    pub source_repository: Option<SourceRepository>
}

impl From<SearchResultEntity> for SearchResult {
    fn from(entity: SearchResultEntity) -> Self {
        let spigot = entity.spigot_id.map(|_| SearchResultSpigot {
            id: entity.spigot_id.expect("Spigot id should not be None"),
            slug: entity.spigot_slug.expect("Spigot slug should not be None"),
            name: entity.spigot_name,
            description: entity.spigot_description.expect("Spigot description should not be None"),
            author: entity.spigot_author.expect("Spigot author should not be None"),
            version: entity.spigot_version,
            premium: entity.spigot_premium.expect("Spigot premium should not be None"),
            abandoned: entity.spigot_abandoned.expect("Spigot abandoned should not be None"),
            icon_data: entity.spigot_icon_data
        });

        let modrinth = entity.modrinth_id.clone().map(|_| SearchResultModrinth {
            id: entity.modrinth_id.expect("Modrinth id should not be None"),
            slug: entity.modrinth_slug.expect("Modrinth slug should not be None"),
            name: entity.modrinth_name.expect("Modrinth name should not be None"),
            description: entity.modrinth_description.expect("Modrinth description should not be None"),
            author: entity.modrinth_author.expect("Modrinth author should not be None"),
            version: entity.modrinth_version,
            icon_url: entity.modrinth_icon_url
        });

        let hangar = entity.hangar_slug.clone().map(|_| SearchResultHangar {
            slug: entity.hangar_slug.expect("Hangar slug should not be None"),
            name: entity.hangar_name.expect("Hangar name should not be None"),
            description: entity.hangar_description.expect("Hangar description should not be None"),
            author: entity.hangar_author.expect("Hangar author should not be None"),
            version: entity.hangar_version,
            avatar_url: entity.hangar_avatar_url.expect("Hangar avatar url should not be None")
        });

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

        SearchResult {
            full_count: entity.full_count,

            date_created: entity.date_created,
            date_updated: entity.date_updated,
            latest_minecraft_version: entity.latest_minecraft_version,
            downloads: entity.downloads,
            likes_and_stars: entity.likes_and_stars,
            follows_and_watchers: entity.follows_and_watchers,

            spigot,
            modrinth,
            hangar,
            source_repository
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchResultSpigot {
    pub id: i32,
    pub slug: String,
    pub name: Option<String>,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub premium: bool,
    pub abandoned: bool,
    pub icon_data: Option<String>
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchResultModrinth {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchResultHangar {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub avatar_url: String
}

#[instrument(
    level = "info",
    skip(db_pool)
)]
pub async fn search_projects(db_pool: &Pool, params: &SearchParams) -> Result<Vec<SearchResult>> {
    let db_client = db_pool.get().await?;

    let entities = search_result::search_projects()
        .params(&db_client, &params.clone().into())
        .all()
        .await?;

    let projects = entities.into_iter().map(|x| x.into()).collect();

    Ok(projects)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::database::spigot::author::SpigotAuthor;
    use crate::database::spigot::resource::SpigotResource;
    use crate::database::spigot::resource::test::populate_test_spigot_authors_and_resources;

    use crate::database::modrinth::project::ModrinthProject;
    use crate::database::modrinth::project::test::populate_test_modrinth_projects;

    use crate::database::hangar::project::HangarProject;
    use crate::database::hangar::project::test::populate_test_hangar_projects;

    use crate::database::common::project::refresh_common_projects;

    use crate::database::test::DatabaseTestContext;

    use ::function_name::named;
    use speculoos::prelude::*;
    use time::macros::datetime;

    #[tokio::test]
    #[named]
    async fn should_search_spigot_resources() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_authors, spigot_resources) = populate_test_spigot_authors_and_resources(&context.pool).await?;
        let _modrinth_projects = populate_test_modrinth_projects(&context.pool).await?;
        let _hangar_projects = populate_test_hangar_projects(&context.pool).await?;

        refresh_common_projects(&context.pool).await?;

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_spigot_resource(&search_results[0], &spigot_resources[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), None, None);
        assert_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_spigot_resource(&search_results[0], &spigot_resources[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), None, None);
        assert_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            spigot: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_spigot_resource(&search_results[0], &spigot_resources[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), None, None);
        assert_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);

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

        refresh_common_projects(&context.pool).await?;

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            modrinth: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_modrinth_project(&search_results[0], &modrinth_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, Some(&modrinth_projects[0]), None);
        assert_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            modrinth: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_modrinth_project(&search_results[0], &modrinth_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, Some(&modrinth_projects[0]), None);
        assert_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            modrinth: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_modrinth_project(&search_results[0], &modrinth_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, Some(&modrinth_projects[0]), None);
        assert_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);

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

        refresh_common_projects(&context.pool).await?;

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            hangar: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, None, Some(&hangar_projects[0]));
        assert_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            hangar: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, None, Some(&hangar_projects[0]));
        assert_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            hangar: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, None, Some(&hangar_projects[0]));
        assert_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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

        refresh_common_projects(&context.pool).await?;

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            modrinth: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_modrinth_project(&search_results[0], &modrinth_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), Some(&modrinth_projects[0]), None);
        assert_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            modrinth: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_modrinth_project(&search_results[0], &modrinth_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), Some(&modrinth_projects[0]), None);
        assert_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            spigot: true,
            modrinth: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_modrinth_project(&search_results[0], &modrinth_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), Some(&modrinth_projects[0]), None);
        assert_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);

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

        refresh_common_projects(&context.pool).await?;

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            hangar: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), None, Some(&hangar_projects[0]));
        assert_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            hangar: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), None, Some(&hangar_projects[0]));
        assert_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            spigot: true,
            hangar: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), None, Some(&hangar_projects[0]));
        assert_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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

        refresh_common_projects(&context.pool).await?;

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            modrinth: true,
            hangar: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, Some(&modrinth_projects[0]), Some(&hangar_projects[0]));
        assert_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);
        assert_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            modrinth: true,
            hangar: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, Some(&modrinth_projects[0]), Some(&hangar_projects[0]));
        assert_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);
        assert_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            modrinth: true,
            hangar: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], None, Some(&modrinth_projects[0]), Some(&hangar_projects[0]));
        assert_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);
        assert_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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

        refresh_common_projects(&context.pool).await?;

        // Act 1 - Search by name
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            modrinth: true,
            hangar: true,
            name: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 1 - Verify search by name
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), Some(&modrinth_projects[0]), Some(&hangar_projects[0]));
        assert_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);
        assert_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

        // Act 2 - Search by description
        let params = SearchParams {
            query: "foo".to_string(),
            spigot: true,
            modrinth: true,
            hangar: true,
            description: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 2 - Verify search by description
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), Some(&modrinth_projects[0]), Some(&hangar_projects[0]));
        assert_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);
        assert_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

        // Act 3 - Search by author
        let params = SearchParams {
            query: "alice".to_string(),
            spigot: true,
            modrinth: true,
            hangar: true,
            author: true,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 3 - Verify search by author
        assert_that(&search_results).has_length(1);
        assert_dates_are_equal_to_hangar_project(&search_results[0], &hangar_projects[0]);
        assert_stats_are_equal_to_projects(&search_results[0], Some(&spigot_resources[0]), Some(&modrinth_projects[0]), Some(&hangar_projects[0]));
        assert_spigot_fields_are_equal(&search_results[0].spigot, &spigot_authors[0], &spigot_resources[0]);
        assert_modrinth_fields_are_equal(&search_results[0].modrinth, &modrinth_projects[0]);
        assert_hangar_fields_are_equal(&search_results[0].hangar, &hangar_projects[0]);

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

        refresh_common_projects(&context.pool).await?;

        // Act 1 - Sort by date_created order
        let params = SearchParams {
            hangar: true,
            name: true,
            sort: SearchParamsSort::DateCreated,
            ..Default::default()
        };
        let search_results = search_projects(&context.pool, &params).await?;

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
        let search_results = search_projects(&context.pool, &params).await?;

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
        let search_results = search_projects(&context.pool, &params).await?;

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
        let search_results = search_projects(&context.pool, &params).await?;

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
        let search_results = search_projects(&context.pool, &params).await?;

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
        let search_results = search_projects(&context.pool, &params).await?;

        // Assert 6 - Verify results are in follows and watchers order
        assert_that(&search_results).has_length(3);
        assert_that(&search_results[0].follows_and_watchers).is_equal_to(300);
        assert_that(&search_results[1].follows_and_watchers).is_equal_to(200);
        assert_that(&search_results[2].follows_and_watchers).is_equal_to(100);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    fn assert_dates_are_equal_to_spigot_resource(search_result: &SearchResult, spigot_resource: &SpigotResource) {
        assert_that(&search_result.date_created).is_equal_to(spigot_resource.date_created);
        assert_that(&search_result.date_updated).is_equal_to(spigot_resource.date_updated);
    }

    fn assert_dates_are_equal_to_modrinth_project(search_result: &SearchResult, modrinth_project: &ModrinthProject) {
        assert_that(&search_result.date_created).is_equal_to(modrinth_project.date_created);
        assert_that(&search_result.date_updated).is_equal_to(modrinth_project.date_updated);
    }

    fn assert_dates_are_equal_to_hangar_project(search_result: &SearchResult, hangar_project: &HangarProject) {
        assert_that(&search_result.date_created).is_equal_to(hangar_project.date_created);
        assert_that(&search_result.date_updated).is_equal_to(hangar_project.date_updated);
    }

    fn assert_stats_are_equal_to_projects(search_result: &SearchResult, spigot_resource: Option<&SpigotResource>, modrinth_project: Option<&ModrinthProject>, hangar_project: Option<&HangarProject>) {
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

    fn assert_spigot_fields_are_equal(spigot: &Option<SearchResultSpigot>, spigot_author: &SpigotAuthor, spigot_resource: &SpigotResource) {
        assert_that(spigot).is_some();

        let s = spigot.as_ref().unwrap();
        assert_that(&s.id).is_equal_to(spigot_resource.id);
        assert_that(&s.slug).is_equal_to(&spigot_resource.slug);
        assert_that(&s.name).is_equal_to(&spigot_resource.parsed_name);
        assert_that(&s.description).is_equal_to(&spigot_resource.description);
        assert_that(&s.author).is_equal_to(&spigot_author.name);
        assert_that(&s.version).is_equal_to(&spigot_resource.version_name);
        assert_that(&s.premium).is_equal_to(spigot_resource.premium);
        assert_that(&s.icon_data).is_equal_to(&spigot_resource.icon_data);
    }

    fn assert_modrinth_fields_are_equal(modrinth: &Option<SearchResultModrinth>, modrinth_project: &ModrinthProject) {
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

    fn assert_hangar_fields_are_equal(hangar: &Option<SearchResultHangar>, hangar_project: &HangarProject) {
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