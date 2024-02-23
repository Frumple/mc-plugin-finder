use crate::database::cornucopia::queries::spigot_author::{self, InsertSpigotAuthorParams, SpigotAuthorEntity};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Client;

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

pub async fn insert_spigot_author(db_client: &Client, author: SpigotAuthor) -> Result<()> {
    spigot_author::insert_spigot_author()
        .params(db_client, &author.into())
        .await?;

    Ok(())
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

    #[tokio::test]
    #[named]
    async fn should_get_highest_spigot_author_id() -> Result<()> {
        // Setup
        let context = DatabaseTestContext::new(function_name!()).await;

        // Arrange
        let authors = vec![
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
        ];

        for author in authors {
            insert_spigot_author(&context.client, author).await?;
        }

        // Act
        let highest_id = get_highest_spigot_author_id(&context.client).await?;

        // Assert
        assert_eq!(highest_id, 3);

        // Teardown
        context.drop().await?;

        Ok(())
    }
}