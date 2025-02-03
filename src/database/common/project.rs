use crate::database::cornucopia::queries::common_project::{self, CommonProjectEntity};
use crate::database::ingest_log::{IngestLog, IngestLogAction, IngestLogRepository, IngestLogItem, insert_ingest_log};

use anyhow::Result;
use deadpool_postgres::Pool;
use time::OffsetDateTime;
use tracing::{info, instrument};

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProject {
    pub spigot: Option<CommonProjectSpigot>,
    pub modrinth: Option<CommonProjectModrinth>,
    pub hangar: Option<CommonProjectHangar>
}

impl From<CommonProjectEntity> for CommonProject {
    fn from(entity: CommonProjectEntity) -> Self {
        let spigot = entity.spigot_id.map(|_| CommonProjectSpigot {
            id: entity.spigot_id.unwrap(),
            slug: entity.spigot_slug.unwrap(),
            name: entity.spigot_name,
            description: entity.spigot_description.unwrap(),
            author: entity.spigot_author.unwrap(),
            version: entity.spigot_version,
            premium: entity.spigot_premium.unwrap(),
            abandoned: entity.spigot_abandoned.unwrap(),
            icon_data: entity.spigot_icon_data,
            date_created: entity.spigot_date_created.unwrap(),
            date_updated: entity.spigot_date_updated.unwrap(),
            latest_minecraft_version: entity.spigot_latest_minecraft_version,
            downloads: entity.spigot_downloads.unwrap(),
            likes: entity.spigot_likes.unwrap()
        });

        let modrinth = entity.modrinth_id.clone().map(|_| CommonProjectModrinth {
            id: entity.modrinth_id.unwrap(),
            slug: entity.modrinth_slug.unwrap(),
            name: entity.modrinth_name.unwrap(),
            description: entity.modrinth_description.unwrap(),
            author: entity.modrinth_author.unwrap(),
            version: entity.modrinth_version,
            icon_url: entity.modrinth_icon_url,
            date_created: entity.modrinth_date_created.unwrap(),
            date_updated: entity.modrinth_date_updated.unwrap(),
            latest_minecraft_version: entity.modrinth_latest_minecraft_version,
            downloads: entity.modrinth_downloads.unwrap(),
            follows: entity.modrinth_follows.unwrap()
        });

        let hangar = entity.hangar_slug.clone().map(|_| CommonProjectHangar {
            slug: entity.hangar_slug.unwrap(),
            name: entity.hangar_name.unwrap(),
            description: entity.hangar_description.unwrap(),
            author: entity.hangar_author.unwrap(),
            version: entity.hangar_version,
            icon_url: entity.hangar_icon_url.unwrap(),
            date_created: entity.hangar_date_created.unwrap(),
            date_updated: entity.hangar_date_updated.unwrap(),
            latest_minecraft_version: entity.hangar_latest_minecraft_version,
            downloads: entity.hangar_downloads.unwrap(),
            stars: entity.hangar_stars.unwrap(),
            watchers: entity.hangar_watchers.unwrap()
        });

        CommonProject {
            spigot,
            modrinth,
            hangar
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProjectSpigot {
    pub id: i32,
    pub slug: String,
    pub name: Option<String>,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub premium: bool,
    pub abandoned: bool,
    pub icon_data: Option<String>,
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub latest_minecraft_version: Option<String>,
    pub downloads: i32,
    pub likes: i32
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProjectModrinth {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub icon_url: Option<String>,
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub latest_minecraft_version: Option<String>,
    pub downloads: i32,
    pub follows: i32
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommonProjectHangar {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub icon_url: String,
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub latest_minecraft_version: Option<String>,
    pub downloads: i32,
    pub stars: i32,
    pub watchers: i32
}

#[instrument(
    level = "info",
    skip(db_pool)
)]
pub async fn refresh_common_projects(db_pool: &Pool) -> Result<()> {
    let db_client = db_pool.get().await?;
    let date_started = OffsetDateTime::now_utc();

    common_project::refresh_common_projects()
        .bind(&db_client)
        .await?;

    let date_finished = OffsetDateTime::now_utc();

    let items_processed = common_project::get_common_project_count()
        .bind(&db_client)
        .one()
        .await?;

        let ingest_log = IngestLog {
            action: IngestLogAction::Refresh,
            repository: IngestLogRepository::Common,
            item: IngestLogItem::Project,
            date_started,
            date_finished,
            items_processed: items_processed.try_into()?
        };
        insert_ingest_log(db_pool, &ingest_log).await?;

    info!("Common projects refreshed: {}", items_processed);

    Ok(())
}

// Used for tests only.
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

    use crate::database::modrinth::project::{ModrinthProject, upsert_modrinth_project};
    use crate::database::modrinth::project::test::populate_test_modrinth_project;

    use crate::database::hangar::project::{HangarProject, upsert_hangar_project};
    use crate::database::hangar::project::test::populate_test_hangar_project;

    use crate::database::test::DatabaseTestContext;

    use ::function_name::named;
    use speculoos::prelude::*;
    use time::macros::datetime;

    #[tokio::test]
    #[named]
    async fn should_insert_and_update_spigot_resource() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_author, mut spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;

        // Act 1 - Refresh common projects
        refresh_common_projects(&context.pool).await?;

        // Assert 1 - Verify project in common projects
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];

        assert_spigot_fields_are_equal(inserted_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_none(inserted_project);
        assert_hangar_fields_are_none(inserted_project);

        // Act 2 - Update project
        spigot_resource.parsed_name = Some("foo-updated".to_string());
        spigot_resource.description = "foo-updated-description".to_string();
        spigot_resource.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        refresh_common_projects(&context.pool).await?;

        // Assert 2 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_none(updated_project);
        assert_hangar_fields_are_none(updated_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_insert_and_update_modrinth_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let mut modrinth_project = populate_test_modrinth_project(&context.pool).await?;

        // Act 1 - Refresh common projects
        refresh_common_projects(&context.pool).await?;

        // Assert 1 - Verify project in common projects
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];

        assert_spigot_fields_are_none(inserted_project);
        assert_modrinth_fields_are_equal(inserted_project, &modrinth_project);
        assert_hangar_fields_are_none(inserted_project);

        // Act 2 - Update project
        modrinth_project.name = "foo-updated".to_string();
        modrinth_project.description = "foo-updated-description".to_string();
        modrinth_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_modrinth_project(&context.pool, &modrinth_project).await?;

        refresh_common_projects(&context.pool).await?;

        // Assert 2 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];

        assert_spigot_fields_are_none(updated_project);
        assert_modrinth_fields_are_equal(updated_project, &modrinth_project);
        assert_hangar_fields_are_none(updated_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_insert_and_update_hangar_project() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let mut hangar_project = populate_test_hangar_project(&context.pool).await?;

        // Act 1 - Refresh common projects
        refresh_common_projects(&context.pool).await?;

        // Assert 1 - Verify project in common projects
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];

        assert_spigot_fields_are_none(inserted_project);
        assert_modrinth_fields_are_none(inserted_project);
        assert_hangar_fields_are_equal(inserted_project, &hangar_project);

        // Act 2 - Update project
        hangar_project.name = "foo-updated".to_string();
        hangar_project.description = "foo-updated-description".to_string();
        hangar_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        refresh_common_projects(&context.pool).await?;

        // Assert 2 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];

        assert_spigot_fields_are_none(updated_project);
        assert_modrinth_fields_are_none(updated_project);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_insert_and_update_spigot_and_modrinth() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_author, mut spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;
        let mut modrinth_project = populate_test_modrinth_project(&context.pool).await?;

        // Act 1 - Refresh common projects
        refresh_common_projects(&context.pool).await?;

        // Assert 1 - Verify project in common projects
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];

        assert_spigot_fields_are_equal(inserted_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_equal(inserted_project, &modrinth_project);
        assert_hangar_fields_are_none(inserted_project);

        // Act 2 - Update project
        spigot_resource.parsed_name = Some("foo-updated".to_string());
        spigot_resource.description = "foo-updated-description".to_string();
        spigot_resource.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        modrinth_project.name = "foo-updated".to_string();
        modrinth_project.description = "foo-updated-description".to_string();
        modrinth_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_modrinth_project(&context.pool, &modrinth_project).await?;

        refresh_common_projects(&context.pool).await?;

        // Assert 2 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_equal(updated_project, &modrinth_project);
        assert_hangar_fields_are_none(updated_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_insert_and_update_spigot_and_hangar() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_author, mut spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;
        let mut hangar_project = populate_test_hangar_project(&context.pool).await?;

        // Act 1 - Refresh common projects
        refresh_common_projects(&context.pool).await?;

        // Assert 1 - Verify project in common projects
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];

        assert_spigot_fields_are_equal(inserted_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_none(inserted_project);
        assert_hangar_fields_are_equal(inserted_project, &hangar_project);

        // Act 2 - Update project
        spigot_resource.parsed_name = Some("foo-updated".to_string());
        spigot_resource.description = "foo-updated-description".to_string();
        spigot_resource.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_spigot_resource(&context.pool, &spigot_resource).await?;

        hangar_project.name = "foo-updated".to_string();
        hangar_project.description = "foo-updated-description".to_string();
        hangar_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        refresh_common_projects(&context.pool).await?;

        // Assert 2 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_none(updated_project);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_insert_and_update_modrinth_and_hangar() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let mut modrinth_project = populate_test_modrinth_project(&context.pool).await?;
        let mut hangar_project = populate_test_hangar_project(&context.pool).await?;

        // Act 1 - Refresh common projects
        refresh_common_projects(&context.pool).await?;

        // Assert 1 - Verify project in common projects
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];

        assert_spigot_fields_are_none(inserted_project);
        assert_modrinth_fields_are_equal(inserted_project, &modrinth_project);
        assert_hangar_fields_are_equal(inserted_project, &hangar_project);

        // Act 2 - Update project
        modrinth_project.name = "foo-updated".to_string();
        modrinth_project.description = "foo-updated-description".to_string();
        modrinth_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_modrinth_project(&context.pool, &modrinth_project).await?;

        hangar_project.name = "foo-updated".to_string();
        hangar_project.description = "foo-updated-description".to_string();
        hangar_project.date_updated = datetime!(2023-07-01 0:00 UTC);
        upsert_hangar_project(&context.pool, &hangar_project).await?;

        refresh_common_projects(&context.pool).await?;

        // Assert 2 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];

        assert_spigot_fields_are_none(updated_project);
        assert_modrinth_fields_are_equal(updated_project, &modrinth_project);
        assert_hangar_fields_are_equal(updated_project, &hangar_project);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_insert_and_update_spigot_modrinth_and_hangar() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let (spigot_author, mut spigot_resource) = populate_test_spigot_author_and_resource(&context.pool).await?;
        let mut modrinth_project = populate_test_modrinth_project(&context.pool).await?;
        let mut hangar_project = populate_test_hangar_project(&context.pool).await?;

        // Act 1 - Refresh common projects
        refresh_common_projects(&context.pool).await?;

        // Assert 1 - Verify project in common projects
        let inserted_projects = get_common_projects(&context.pool).await?;

        assert_that(&inserted_projects).has_length(1);

        let inserted_project = &inserted_projects[0];

        assert_spigot_fields_are_equal(inserted_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_equal(inserted_project, &modrinth_project);
        assert_hangar_fields_are_equal(inserted_project, &hangar_project);

        // Act 2 - Update project
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

        refresh_common_projects(&context.pool).await?;

        // Assert 2 - Verify project was updated
        let updated_projects = get_common_projects(&context.pool).await?;

        assert_that(&updated_projects).has_length(1);

        let updated_project = &updated_projects[0];

        assert_spigot_fields_are_equal(updated_project, &spigot_author, &spigot_resource);
        assert_modrinth_fields_are_equal(updated_project, &modrinth_project);
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