mod cornucopia;
pub mod common;
pub mod hangar;
pub mod modrinth;
pub mod source_repository;
pub mod spigot;

use crate::config::{get_config_string, get_config_int};

use anyhow::Result;
use deadpool_postgres::{CreatePoolError, Pool, Runtime};
use tokio_postgres::NoTls;

pub struct Database {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String
}

impl Database {
    pub async fn create_pool(&self) -> Result<Pool, CreatePoolError> {
        self.create_custom_pool(&self.db_name).await
    }

    pub async fn create_custom_pool(&self, db_name: &str) -> Result<Pool, CreatePoolError> {
        let config = deadpool_postgres::Config {
            user: Some(self.user.clone()),
            password: Some(self.password.clone()),
            host: Some(self.host.clone()),
            port: Some(self.port),
            dbname: Some(db_name.to_string()),
            ..Default::default()
        };
        config.create_pool(Some(Runtime::Tokio1), NoTls)
    }
}

pub fn get_db() -> Database {
    Database {
       user: get_config_string("db.user"),
       password: get_config_string("db.password"),
       host: get_config_string("db.host"),
       port: u16::try_from(get_config_int("db.port")).expect("could not convert db.port to u16"),
       db_name: get_config_string("db.name"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use deadpool_postgres::Pool;
    use std::fs::read_to_string;

    const DEFAULT_POSTGRES_DB_NAME: &str = "postgres";

    pub struct DatabaseTestContext {
        db_name: String,
        base_pool: Pool,
        pub pool: Pool
    }

    impl DatabaseTestContext {
        pub async fn new(db_name: &str) -> Self {
            dotenvy::dotenv().expect("could not load environment variables from .env file");

            let db = get_db();

            // Tests should always connect to the default database before creating/dropping other databases
            let base_pool = db.create_custom_pool(DEFAULT_POSTGRES_DB_NAME)
                .await
                .expect("could not create database pool");

            Self::drop_database(&base_pool, db_name)
                .await
                .expect("could not drop database before re-creating it");
            Self::create_database(&base_pool, db_name)
                .await
                .expect("could not create database");

            let pool = db.create_custom_pool(db_name)
                .await
                .expect("could not create database pool");

            Self::run_migration(&pool)
                .await
                .expect("could not run migration");

            Self {
                db_name: db_name.to_string(),
                base_pool,
                pool
            }
        }

        async fn create_database(pool: &Pool, db_name: &str) -> Result<()> {
            let client = pool.get().await?;

            let statement = format!("CREATE DATABASE {};", db_name);
            client.execute(&statement, &[]).await?;

            Ok(())
        }

        async fn run_migration(pool: &Pool) -> Result<()> {
            let client = pool.get().await?;

            let schema_text = read_to_string("schema.sql")?
               .parse::<String>()?;

            client.batch_execute(schema_text.as_str()).await?;

            Ok(())
        }

        async fn drop_database(pool: &Pool, db_name: &str) -> Result<()> {
            let client = pool.get().await?;

            let disconnect_users_statement =
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1;";
            client.execute(disconnect_users_statement, &[&db_name]).await?;

            let drop_database_statement = format!("DROP DATABASE IF EXISTS {};", db_name);
            client.execute(&drop_database_statement, &[]).await?;

            Ok(())
        }

        // TODO: When async drop trait is implemented in Rust, use that instead
        pub async fn drop(&self) -> Result<()> {
            Self::drop_database(&self.base_pool, &self.db_name).await?;

            Ok(())
        }
    }
}