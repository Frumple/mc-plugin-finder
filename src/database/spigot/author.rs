use crate::database::cornucopia::queries::spigot_author::{self, InsertSpigotAuthorParams, SpigotAuthorEntity};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Client;
use thiserror::Error;

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

pub async fn insert_spigot_author(db_client: &Client, author: SpigotAuthor) -> Result<()> {
    let author_id = author.id;

    let db_result = spigot_author::insert_spigot_author()
        .params(db_client, &author.into())
        .await;

    match db_result {
        Ok(_) => Ok(()),
        Err(err) => Err(
            SpigotAuthorError::DatabaseQueryFailed {
                author_id,
                source: err.into()
            }.into()
        )
    }
}

pub async fn get_spigot_authors(db_client: &Client) -> Result<Vec<SpigotAuthor>> {
    let entities = spigot_author::get_spigot_authors()
        .bind(db_client)
        .all()
        .await?;

    let authors = entities.into_iter().map(|x| x.into()).collect();

    Ok(authors)
}

pub async fn get_highest_spigot_author_id(db_client: &Client) -> Result<i32> {
    let id = spigot_author::get_highest_spigot_author_id()
        .bind(db_client)
        .one()
        .await?;

    Ok(id)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::database::spigot::author::{insert_spigot_author, get_highest_spigot_author_id};
    use crate::test::DatabaseTestContext;

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
        insert_spigot_author(&context.client, author.clone()).await?;

        // Assert
        let retrieved_authors = get_spigot_authors(&context.client).await?;
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
        let author = &create_test_authors()[0];

        // Act
        insert_spigot_author(&context.client, author.clone()).await?;

        let result = insert_spigot_author(&context.client, author.clone()).await;
        let error = result.unwrap_err();

        // Assert
        assert!(matches!(error.downcast_ref::<SpigotAuthorError>(), Some(SpigotAuthorError::DatabaseQueryFailed{ .. })));

        let retrieved_authors = get_spigot_authors(&context.client).await?;
        let retrieved_author = &retrieved_authors[0];

        assert_that(&retrieved_authors).has_length(1);
        assert_that(&retrieved_author).is_equal_to(author);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    #[tokio::test]
    #[named]
    async fn should_get_highest_spigot_author_id() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let authors = create_test_authors();

        for author in authors {
            insert_spigot_author(&context.client, author).await?;
        }

        // Act
        let highest_id = get_highest_spigot_author_id(&context.client).await?;

        // Assert
        assert_that(&highest_id).is_equal_to(3);

        // Teardown
        context.drop().await?;

        Ok(())
    }

    fn create_test_authors() -> Vec<SpigotAuthor> {
        vec![
            SpigotAuthor {
                id: 1,
                name: "author-1".to_string()
            },
            SpigotAuthor {
                id: 2,
                name: "author-2".to_string()
            },
            SpigotAuthor {
                id: 3,
                name: "author-3".to_string()
            }
        ]
    }
}