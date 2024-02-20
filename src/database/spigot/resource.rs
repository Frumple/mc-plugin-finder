use crate::database::cornucopia::queries::spigot_resource::{self, UpsertSpigotResourceParams, SpigotResourceEntity};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Object;
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

pub async fn upsert_spigot_resource(db_client: &Object, resource: SpigotResource) -> Result<()> {
    spigot_resource::upsert_spigot_resource()
        .params(db_client, &resource.into())
        .await?;

    Ok(())
}

pub async fn get_spigot_resources(db_client: &Object) -> Result<Vec<SpigotResource>> {
    let entities = spigot_resource::get_spigot_resources()
        .bind(db_client)
        .all()
        .await?;

    let resources = entities.into_iter().map(|x| x.into()).collect();

    Ok(resources)
}

pub async fn get_latest_spigot_resource_update_date(db_client: &Object) -> Result<OffsetDateTime> {
    let date = spigot_resource::get_latest_spigot_resource_update_date()
        .bind(db_client)
        .one()
        .await?;

    Ok(date)
}