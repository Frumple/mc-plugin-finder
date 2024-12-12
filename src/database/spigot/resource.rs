use crate::database::source_repository::SourceRepository;
use crate::database::cornucopia::queries::spigot_resource::{self, SpigotResourceEntity, UpsertSpigotResourceParams};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use thiserror::Error;
use time::OffsetDateTime;
use tracing::instrument;

#[derive(Clone, Debug, PartialEq)]
pub struct SpigotResource {
    pub id: i32,
    pub name: String,
    pub parsed_name: Option<String>,
    pub description: String,
    pub slug: String,
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub latest_minecraft_version: Option<String>,
    pub downloads: i32,
    pub likes: i32,
    pub author_id: i32,
    pub version_id: i32,
    pub version_name: Option<String>,
    pub premium: bool,
    pub icon_url: Option<String>,
    pub icon_data: Option<String>,
    pub source_url: Option<String>,
    pub source_repository: Option<SourceRepository>
}

impl From<SpigotResource> for UpsertSpigotResourceParams<String, String, String, String, String, String, String, String, String, String, String, String> {
    fn from(resource: SpigotResource) -> Self {
        let mut source_repository_host = None;
        let mut source_repository_owner = None;
        let mut source_repository_name = None;

        if let Some(repo) = resource.source_repository {
            source_repository_host = Some(repo.host);
            source_repository_owner = Some(repo.owner);
            source_repository_name = Some(repo.name);
        }

        UpsertSpigotResourceParams {
            id: resource.id,
            name: resource.name,
            parsed_name: resource.parsed_name,
            description: resource.description,
            slug: resource.slug,
            icon_url: resource.icon_url,
            icon_data: resource.icon_data,
            date_created: resource.date_created,
            date_updated: resource.date_updated,
            latest_minecraft_version: resource.latest_minecraft_version,
            downloads: resource.downloads,
            likes: resource.likes,
            author_id: resource.author_id,
            version_id: resource.version_id,
            version_name: resource.version_name,
            premium: resource.premium,
            source_url: resource.source_url,
            source_repository_host,
            source_repository_owner,
            source_repository_name
        }
    }
}

impl From<SpigotResourceEntity> for SpigotResource {
    fn from(entity: SpigotResourceEntity) -> Self {
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

        SpigotResource {
            id: entity.id,
            name: entity.name,
            parsed_name: entity.parsed_name,
            description: entity.description,
            slug: entity.slug,
            date_created: entity.date_created,
            date_updated: entity.date_updated,
            latest_minecraft_version: entity.latest_minecraft_version,
            downloads: entity.downloads,
            likes: entity.likes,
            author_id: entity.author_id,
            version_id: entity.version_id,
            version_name: entity.version_name,
            premium: entity.premium,
            icon_url: entity.icon_url,
            icon_data: entity.icon_data,
            source_url: entity.source_url,
            source_repository
        }
    }
}

#[derive(Debug, Error)]
enum SpigotResourceError {
    #[error("Skipping resource ID {resource_id}: Database query failed: {source}")]
    DatabaseQueryFailed {
        resource_id: i32,
        source: anyhow::Error
    }
}

#[instrument(
    level = "debug",
    skip(db_pool)
)]
pub async fn upsert_spigot_resource(db_pool: &Pool, resource: &SpigotResource) -> Result<()> {
    let db_client = db_pool.get().await?;

    let db_result = spigot_resource::upsert_spigot_resource()
        .params(&db_client, &resource.clone().into())
        .await;

    match db_result {
        Ok(_) => Ok(()),
        Err(err) => Err(
            SpigotResourceError::DatabaseQueryFailed {
                resource_id: resource.id,
                source: err.into()
            }.into()
        )
    }
}

pub async fn get_spigot_resources(db_pool: &Pool) -> Result<Vec<SpigotResource>> {
    let db_client = db_pool.get().await?;

    let entities = spigot_resource::get_spigot_resources()
        .bind(&db_client)
        .all()
        .await?;

    let resources = entities.into_iter().map(|x| x.into()).collect();

    Ok(resources)
}

pub async fn get_latest_spigot_resource_update_date(db_pool: &Pool) -> Result<OffsetDateTime> {
    let db_client = db_pool.get().await?;

    let date: OffsetDateTime = spigot_resource::get_latest_spigot_resource_update_date()
        .bind(&db_client)
        .one()
        .await?;

    Ok(date)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::database::spigot::author::SpigotAuthor;
    use crate::database::spigot::author::test::{populate_test_spigot_author, populate_test_spigot_authors};
    use crate::database::spigot::test::SPIGOT_BASE64_TEST_ICON_DATA;
    use crate::database::test::DatabaseTestContext;

    use ::function_name::named;
    use speculoos::prelude::*;
    use time::macros::datetime;

    #[tokio::test]
    #[named]
    async fn should_insert_spigot_resource() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let _authors = populate_test_spigot_authors(&context.pool).await?;
        let resource = &create_test_spigot_resources()[0];

        // Act
        upsert_spigot_resource(&context.pool, resource).await?;

        // Assert
        let retrieved_resources = get_spigot_resources(&context.pool).await?;
        let retrieved_resource = &retrieved_resources[0];

        assert_that(&retrieved_resources).has_length(1);
        assert_that(&retrieved_resource).is_equal_to(resource);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_update_spigot_resource() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let _authors = populate_test_spigot_authors(&context.pool).await?;

        let resource = &create_test_spigot_resources()[0];
        upsert_spigot_resource(&context.pool, resource).await?;

        let updated_resource = SpigotResource {
            id: 1,
            name: "foo-updated".to_string(),
            parsed_name: Some("foo-updated".to_string()),
            description: "foo-description-updated".to_string(),
            slug: "foo-updated.1".to_string(),
            date_created: datetime!(2020-01-01 0:00 UTC),
            date_updated: datetime!(2021-07-01 0:00 UTC),
            latest_minecraft_version: Some("1.22".to_string()),
            downloads: 101,
            likes: 201,
            author_id: 1,
            version_id: 2,
            version_name: None,
            premium: true,
            icon_url: Some("data/resource_icons/1/1.jpg".to_string()),
            icon_data: Some(SPIGOT_BASE64_TEST_ICON_DATA.to_string()),
            source_url: Some("https://github.com/alice/foo-updated".to_string()),
            source_repository: Some(SourceRepository {
                host: "github.com".to_string(),
                owner: "alice".to_string(),
                name: "foo-updated".to_string()
            })
        };

        // Act
        upsert_spigot_resource(&context.pool, &updated_resource).await?;

        // Assert
        let retrieved_resources = get_spigot_resources(&context.pool).await?;
        let retrieved_resource = &retrieved_resources[0];

        assert_that(&retrieved_resources).has_length(1);
        assert_that(&retrieved_resource).is_equal_to(&updated_resource);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_not_insert_resource_with_nonexistent_author() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let resource = &create_test_spigot_resources()[0];

        // Act
        let result = upsert_spigot_resource(&context.pool, resource).await;

        // Assert
        assert_that(&result).is_err();

        let error = result.unwrap_err();
        let downcast_error = error.downcast_ref::<SpigotResourceError>().unwrap();

        #[allow(irrefutable_let_patterns)]
        if let SpigotResourceError::DatabaseQueryFailed { resource_id, source: _ } = downcast_error {
            assert_that(&resource_id).is_equal_to(&resource.id);
        } else {
            panic!("expected error to be DatabaseQueryFailed, but was {}", downcast_error);
        }

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_get_latest_spigot_resource_update_date() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let _authors: Vec<SpigotAuthor> = populate_test_spigot_authors(&context.pool).await?;

        let resources = create_test_spigot_resources();
        for resource in resources {
            upsert_spigot_resource(&context.pool, &resource).await?;
        }

        // Act
        let latest_update_date = get_latest_spigot_resource_update_date(&context.pool).await?;

        // Assert
        assert_that(&latest_update_date).is_equal_to(datetime!(2020-02-03 0:00 UTC));

        // Teardown
        context.drop().await?;

        Ok(())
    }

    pub async fn populate_test_spigot_author_and_resource(db_pool: &Pool) -> Result<(SpigotAuthor, SpigotResource)> {
        let author = populate_test_spigot_author(db_pool).await?;

        let resource = &create_test_spigot_resources()[0];
        upsert_spigot_resource(db_pool, resource).await?;
        Ok((author.clone(), resource.clone()))
    }

    pub async fn populate_test_spigot_authors_and_resources(db_pool: &Pool) -> Result<(Vec<SpigotAuthor>, Vec<SpigotResource>)> {
        let authors = populate_test_spigot_authors(db_pool).await?;

        let resources = create_test_spigot_resources();
        for resource in &resources {
            upsert_spigot_resource(db_pool, resource).await?;
        }
        Ok((authors, resources))
    }

    fn create_test_spigot_resources() -> Vec<SpigotResource> {
        vec![
            SpigotResource {
                id: 1,
                name: "foo-spigot".to_string(),
                parsed_name: Some("foo-spigot".to_string()),
                description: "foo-spigot-description".to_string(),
                slug: "foo.1".to_string(),
                date_created: datetime!(2020-01-01 0:00 UTC),
                date_updated: datetime!(2020-02-03 0:00 UTC),
                latest_minecraft_version: Some("1.21".to_string()),
                downloads: 100,
                likes: 200,
                author_id: 1,
                version_id: 1,
                version_name: None,
                premium: false,
                icon_url: Some("data/resource_icons/1/1.jpg".to_string()),
                icon_data: Some(SPIGOT_BASE64_TEST_ICON_DATA.to_string()),
                source_url: Some("https://github.com/alice/foo".to_string()),
                source_repository: Some(SourceRepository {
                    host: "github.com".to_string(),
                    owner: "alice".to_string(),
                    name: "foo".to_string()
                })
            },
            SpigotResource {
                id: 2,
                name: "bar-spigot".to_string(),
                parsed_name: Some("bar-spigot".to_string()),
                description: "bar-spigot-description".to_string(),
                slug: "bar.2".to_string(),
                date_created: datetime!(2020-01-02 0:00 UTC),
                date_updated: datetime!(2020-02-02 0:00 UTC),
                latest_minecraft_version: Some("1.8".to_string()),
                downloads: 300,
                likes: 100,
                author_id: 2,
                version_id: 1,
                version_name: None,
                premium: false,
                icon_url: Some("data/resource_icons/2/2.jpg".to_string()),
                icon_data: Some(SPIGOT_BASE64_TEST_ICON_DATA.to_string()),
                source_url: Some("https://gitlab.com/bob/bar".to_string()),
                source_repository: Some(SourceRepository {
                    host: "gitlab.com".to_string(),
                    owner: "bob".to_string(),
                    name: "bar".to_string()
                })
            },
            SpigotResource {
                id: 3,
                name: "baz-spigot".to_string(),
                parsed_name: Some("baz-spigot".to_string()),
                description: "baz-spigot-description".to_string(),
                slug: "baz.3".to_string(),
                date_created: datetime!(2020-01-03 0:00 UTC),
                date_updated: datetime!(2020-02-01 0:00 UTC),
                latest_minecraft_version: Some("1.16".to_string()),
                downloads: 200,
                likes: 300,
                author_id: 3,
                version_id: 1,
                version_name: None,
                premium: false,
                icon_url: Some("data/resource_icons/3/3.jpg".to_string()),
                icon_data: Some(SPIGOT_BASE64_TEST_ICON_DATA.to_string()),
                source_url: Some("https://bitbucket.org/eve/baz".to_string()),
                source_repository: Some(SourceRepository {
                    host: "bitbucket.org".to_string(),
                    owner: "eve".to_string(),
                    name: "baz".to_string()
                })
            }
        ]
    }
}