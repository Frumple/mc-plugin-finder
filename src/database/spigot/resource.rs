use crate::database::cornucopia::queries::spigot_resource::{self, UpsertSpigotResourceParams, SpigotResourceEntity};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Client;
use time::OffsetDateTime;

pub struct SpigotResource {
    pub id: i32,
    pub name: String,
    pub tag: String,
    pub slug: String,
    pub release_date: OffsetDateTime,
    pub update_date: OffsetDateTime,
    pub author_id: i32,
    pub version_id: i32,
    pub version_name: Option<String>,
    pub premium: Option<bool>,
    pub source_code_link: Option<String>
}

impl From<SpigotResource> for UpsertSpigotResourceParams<String, String, String, String, String> {
    fn from(resource: SpigotResource) -> Self {
        UpsertSpigotResourceParams {
            id: resource.id,
            name: resource.name,
            tag: resource.tag,
            slug: resource.slug,
            release_date: resource.release_date,
            update_date: resource.update_date,
            author_id: resource.author_id,
            version_id: resource.version_id,
            version_name: resource.version_name,
            premium: resource.premium,
            source_code_link: resource.source_code_link
        }
    }
}

impl From<SpigotResourceEntity> for SpigotResource {
    fn from(entity: SpigotResourceEntity) -> Self {
        SpigotResource {
            id: entity.id,
            name: entity.name,
            tag: entity.tag,
            slug: entity.slug,
            release_date: entity.release_date,
            update_date: entity.update_date,
            author_id: entity.author_id,
            version_id: entity.version_id,
            version_name: entity.version_name,
            premium: entity.premium,
            source_code_link: entity.source_code_link
        }
    }
}

pub async fn upsert_spigot_resource(db_client: &Client, resource: SpigotResource) -> Result<()> {
    spigot_resource::upsert_spigot_resource()
        .params(db_client, &resource.into())
        .await?;

    Ok(())
}

pub async fn get_spigot_resources(db_client: &Client) -> Result<Vec<SpigotResource>> {
    let entities = spigot_resource::get_spigot_resources()
        .bind(db_client)
        .all()
        .await?;

    let resources = entities.into_iter().map(|x| x.into()).collect();

    Ok(resources)
}

pub async fn get_latest_spigot_resource_update_date(db_client: &Client) -> Result<OffsetDateTime> {
    let date = spigot_resource::get_latest_spigot_resource_update_date()
        .bind(db_client)
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

    #[tokio::test]
    #[named]
    async fn should_get_latest_spigot_resource_update_date() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let author = SpigotAuthor {
            id: 1,
            name: "author-1".to_string()
        };

        insert_spigot_author(&context.client, author).await?;

        let resources = vec![
            SpigotResource {
                id: 1,
                name: "resource-1".to_string(),
                tag: "resource-1-tag".to_string(),
                slug: "foo.1".to_string(),
                release_date: OffsetDateTime::from_unix_timestamp(1577865600)?,
                update_date: OffsetDateTime::from_unix_timestamp(1609488000)?,
                author_id: 1,
                version_id: 1,
                version_name: None,
                premium: Some(false),
                source_code_link: Some("https://github.com/Frumple/foo".to_string())
            },
            SpigotResource {
                id: 2,
                name: "resource-2".to_string(),
                tag: "resource-2-tag".to_string(),
                slug: "bar.2".to_string(),
                release_date: OffsetDateTime::from_unix_timestamp(1577865600)?,
                update_date: OffsetDateTime::from_unix_timestamp(1641024000)?,
                author_id: 1,
                version_id: 1,
                version_name: None,
                premium: Some(false),
                source_code_link: Some("https://github.com/Frumple/bar".to_string())
            },
            SpigotResource {
                id: 3,
                name: "resource-3".to_string(),
                tag: "resource-3-tag".to_string(),
                slug: "baz.3".to_string(),
                release_date: OffsetDateTime::from_unix_timestamp(1577865600)?,
                update_date: OffsetDateTime::from_unix_timestamp(1672560000)?,
                author_id: 1,
                version_id: 1,
                version_name: None,
                premium: Some(false),
                source_code_link: Some("https://github.com/Frumple/baz".to_string())
            }
        ];

        for resource in resources {
            upsert_spigot_resource(&context.client, resource).await?;
        }

        // Act
        let latest_update_date = get_latest_spigot_resource_update_date(&context.client).await?;

        // Assert
        assert_eq!(latest_update_date, OffsetDateTime::from_unix_timestamp(1672560000)?);

        // Teardown
        context.drop().await?;

        Ok(())
    }

}