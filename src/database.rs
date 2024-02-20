mod cornucopia;
pub mod spigot;

use anyhow::Result;
use deadpool_postgres::{Config, CreatePoolError, Object, Pool, Runtime};
use tokio_postgres::NoTls;

pub struct DatabaseCredentials {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String
}

pub struct DatabaseManager {
    pool: Pool
}

impl DatabaseManager {
    pub async fn new(credentials: DatabaseCredentials) -> Result<Self> {
        let pool = DatabaseManager::create_pool(credentials).await?;

        let manager = DatabaseManager {
            pool
        };

        Ok(manager)
    }

    async fn create_pool(credentials: DatabaseCredentials) -> Result<Pool, CreatePoolError> {
        let mut config = Config::new();
        config.user = Some(credentials.user.clone());
        config.password = Some(credentials.password.clone());
        config.host = Some(credentials.host.clone());
        config.port = Some(credentials.port);
        config.dbname = Some(credentials.dbname.clone());
        config.create_pool(Some(Runtime::Tokio1), NoTls)
    }

    pub async fn get_client(&self) -> Result<Object> {
        let client = self.pool.get().await?;
        Ok(client)
    }
}