use crate::database::cornucopia::queries::spigot_author::{self, InsertSpigotAuthorParams, SpigotAuthorEntity};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use thiserror::Error;
use tracing::instrument;

#[derive(Clone, Debug, PartialEq)]
pub struct SpigotAuthor {
    pub id: i32,
    pub name: String
}

impl From<SpigotAuthor> for InsertSpigotAuthorParams<String> {
    fn from(author: SpigotAuthor) -> Self {
        InsertSpigotAuthorParams {
            id: author.id,
            name: author.name
        }
    }
}

impl From<SpigotAuthorEntity> for SpigotAuthor {
    fn from(entity: SpigotAuthorEntity) -> Self {
        SpigotAuthor {
            id: entity.id,
            name: entity.name
        }
    }
}

#[derive(Debug, Error)]
enum SpigotAuthorError {
    #[error("Skipping author ID {author_id}: Database query failed: {source}")]
    DatabaseQueryFailed {
        author_id: i32,
        source: anyhow::Error
    }
}

#[instrument(
    level = "debug",
    skip(db_pool)
)]
pub async fn insert_spigot_author(db_pool: &Pool, author: &SpigotAuthor) -> Result<()> {
    let db_client = db_pool.get().await?;

    let db_result = spigot_author::insert_spigot_author()
        .params(&db_client, &author.clone().into())
        .await;

    match db_result {
        Ok(_) => Ok(()),
        Err(err) => Err(
            SpigotAuthorError::DatabaseQueryFailed {
                author_id: author.id,
                source: err.into()
            }.into()
        )
    }
}

pub async fn get_spigot_authors(db_pool: &Pool) -> Result<Vec<SpigotAuthor>> {
    let db_client = db_pool.get().await?;

    let authors = spigot_author::get_spigot_authors()
        .bind(&db_client)
        .all()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect();

    Ok(authors)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::database::test::DatabaseTestContext;

    use ::function_name::named;
    use speculoos::prelude::*;

    #[tokio::test]
    #[named]
    async fn should_insert_spigot_author_into_db() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let author = &create_test_authors()[0];

        // Act
        insert_spigot_author(&context.pool, author).await?;

        // Assert
        let retrieved_authors = get_spigot_authors(&context.pool).await?;
        let retrieved_author = &retrieved_authors[0];

        assert_that(&retrieved_authors).has_length(1);
        assert_that(&retrieved_author).is_equal_to(author);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_not_insert_author_with_duplicate_id_into_db() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let old_author = &create_test_authors()[0];
        let mut new_author = old_author.clone();

        // Act
        insert_spigot_author(&context.pool, old_author).await?;

        new_author.name = "carl".to_string();

        let result = insert_spigot_author(&context.pool, &new_author).await;

        // Assert

        // Verify no error occurs
        assert_that(&result).is_ok();

        // Verify author has not been changed
        let retrieved_authors = get_spigot_authors(&context.pool).await?;
        let retrieved_author = &retrieved_authors[0];

        assert_that(&retrieved_authors).has_length(1);
        assert_that(&retrieved_author).is_equal_to(old_author);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    pub async fn populate_test_spigot_author(db_pool: &Pool) -> Result<SpigotAuthor> {
        let author = &create_test_authors()[0];
        insert_spigot_author(db_pool, author).await?;
        Ok(author.clone())
    }

    pub async fn populate_test_spigot_authors(db_pool: &Pool) -> Result<Vec<SpigotAuthor>> {
        let authors = create_test_authors();
        for author in &authors {
            insert_spigot_author(db_pool, author).await?;
        }
        Ok(authors)
    }

    fn create_test_authors() -> Vec<SpigotAuthor> {
        vec![
            SpigotAuthor {
                id: 1,
                name: "alice".to_string()
            },
            SpigotAuthor {
                id: 2,
                name: "bob".to_string()
            },
            SpigotAuthor {
                id: 3,
                name: "eve".to_string()
            }
        ]
    }
}