use crate::database::cornucopia::queries::spigot_resource::{self, UpsertSpigotResourceParams, SpigotResourceEntity};

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
    pub tag: String,
    pub slug: String,
    pub release_date: OffsetDateTime,
    pub update_date: OffsetDateTime,
    pub author_id: i32,
    pub version_id: i32,
    pub version_name: Option<String>,
    pub premium: Option<bool>,
    pub source_code_link: Option<String>,
    pub source_repository_host: Option<String>,
    pub source_repository_owner: Option<String>,
    pub source_repository_name: Option<String>
}

impl From<SpigotResource> for UpsertSpigotResourceParams<String, String, String, String, String, String, String, String, String> {
    fn from(resource: SpigotResource) -> Self {
        UpsertSpigotResourceParams {
            id: resource.id,
            name: resource.name,
            parsed_name: resource.parsed_name,
            tag: resource.tag,
            slug: resource.slug,
            release_date: resource.release_date,
            update_date: resource.update_date,
            author_id: resource.author_id,
            version_id: resource.version_id,
            version_name: resource.version_name,
            premium: resource.premium,
            source_code_link: resource.source_code_link,
            source_repository_host: resource.source_repository_host,
            source_repository_owner: resource.source_repository_owner,
            source_repository_name: resource.source_repository_name
        }
    }
}

impl From<SpigotResourceEntity> for SpigotResource {
    fn from(entity: SpigotResourceEntity) -> Self {
        SpigotResource {
            id: entity.id,
            name: entity.name,
            parsed_name: entity.parsed_name,
            tag: entity.tag,
            slug: entity.slug,
            release_date: entity.release_date,
            update_date: entity.update_date,
            author_id: entity.author_id,
            version_id: entity.version_id,
            version_name: entity.version_name,
            premium: entity.premium,
            source_code_link: entity.source_code_link,
            source_repository_host: entity.source_repository_host,
            source_repository_owner: entity.source_repository_owner,
            source_repository_name: entity.source_repository_name
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
pub async fn upsert_spigot_resource(db_pool: &Pool, resource: SpigotResource) -> Result<()> {
    let db_client = db_pool.get().await?;
    let resource_id = resource.id;

    let db_result = spigot_resource::upsert_spigot_resource()
        .params(&db_client, &resource.into())
        .await;

    match db_result {
        Ok(_) => Ok(()),
        Err(err) => Err(
            SpigotResourceError::DatabaseQueryFailed {
                resource_id,
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
mod test {
    use super::*;
    use crate::database::spigot::author::{SpigotAuthor, insert_spigot_author};
    use crate::database::spigot::resource::{upsert_spigot_resource, get_latest_spigot_resource_update_date};
    use crate::test::DatabaseTestContext;

    use ::function_name::named;
    use speculoos::prelude::*;
    use time::macros::datetime;

    #[tokio::test]
    #[named]
    async fn should_insert_spigot_resource() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let author = create_test_author();
        insert_spigot_author(&context.pool, author).await?;

        let resource = &create_test_resources()[0];

        // Act
        upsert_spigot_resource(&context.pool, resource.clone()).await?;

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
        let author = create_test_author();
        insert_spigot_author(&context.pool, author).await?;

        let resource = &create_test_resources()[0];
        upsert_spigot_resource(&context.pool, resource.clone()).await?;

        let updated_resource = SpigotResource {
            id: 1,
            name: "resource-1-updated".to_string(),
            parsed_name: Some("resource-1-updated".to_string()),
            tag: "resource-1-tag-updated".to_string(),
            slug: "foo-updated.1".to_string(),
            release_date: datetime!(2020-01-01 0:00 UTC),
            update_date: datetime!(2021-07-01 0:00 UTC),
            author_id: 1,
            version_id: 2,
            version_name: None,
            premium: Some(true),
            source_code_link: Some("https://github.com/Frumple/foo-updated".to_string()),
            source_repository_host: Some("github.com".to_string()),
            source_repository_owner: Some("Frumple".to_string()),
            source_repository_name: Some("foo-updated".to_string())
        };

        // Act
        upsert_spigot_resource(&context.pool, updated_resource.clone()).await?;

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
        let resource = &create_test_resources()[0];

        // Act
        let result = upsert_spigot_resource(&context.pool, resource.clone()).await;
        let error = result.unwrap_err();

        // Assert
        assert!(matches!(error.downcast_ref::<SpigotResourceError>(), Some(SpigotResourceError::DatabaseQueryFailed { .. })));

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
        let author = create_test_author();

        insert_spigot_author(&context.pool, author).await?;

        let resources = create_test_resources();
        for resource in resources {
            upsert_spigot_resource(&context.pool, resource).await?;
        }

        // Act
        let latest_update_date = get_latest_spigot_resource_update_date(&context.pool).await?;

        // Assert
        assert_that(&latest_update_date).is_equal_to(datetime!(2023-01-01 0:00 UTC));

        // Teardown
        context.drop().await?;

        Ok(())
    }

    fn create_test_author() -> SpigotAuthor {
        SpigotAuthor {
            id: 1,
            name: "author-1".to_string()
        }
    }

    fn create_test_resources() -> Vec<SpigotResource> {
        vec![
            SpigotResource {
                id: 1,
                name: "resource-1".to_string(),
                parsed_name: Some("resource-1".to_string()),
                tag: "resource-1-tag".to_string(),
                slug: "foo.1".to_string(),
                release_date: datetime!(2020-01-01 0:00 UTC),
                update_date: datetime!(2021-01-01 0:00 UTC),
                author_id: 1,
                version_id: 1,
                version_name: None,
                premium: Some(false),
                source_code_link: Some("https://github.com/Frumple/foo".to_string()),
                source_repository_host: Some("github.com".to_string()),
                source_repository_owner: Some("Frumple".to_string()),
                source_repository_name: Some("foo".to_string())
            },
            SpigotResource {
                id: 2,
                name: "resource-2".to_string(),
                parsed_name: Some("resource-2".to_string()),
                tag: "resource-2-tag".to_string(),
                slug: "bar.2".to_string(),
                release_date: datetime!(2020-01-01 0:00 UTC),
                update_date: datetime!(2022-01-01 0:00 UTC),
                author_id: 1,
                version_id: 1,
                version_name: None,
                premium: Some(false),
                source_code_link: Some("https://gitlab.com/Frumple/bar".to_string()),
                source_repository_host: Some("gitlab.com".to_string()),
                source_repository_owner: Some("Frumple".to_string()),
                source_repository_name: Some("bar".to_string())
            },
            SpigotResource {
                id: 3,
                name: "resource-3".to_string(),
                parsed_name: Some("resource-3".to_string()),
                tag: "resource-3-tag".to_string(),
                slug: "baz.3".to_string(),
                release_date: datetime!(2020-01-01 0:00 UTC),
                update_date: datetime!(2023-01-01 0:00 UTC),
                author_id: 1,
                version_id: 1,
                version_name: None,
                premium: Some(false),
                source_code_link: Some("https://bitbucket.org/Frumple/baz".to_string()),
                source_repository_host: Some("bitbucket.org".to_string()),
                source_repository_owner: Some("Frumple".to_string()),
                source_repository_name: Some("baz".to_string())
            }
        ]
    }
}