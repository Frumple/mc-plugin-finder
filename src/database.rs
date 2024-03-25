mod cornucopia;
pub mod common;
pub mod hangar;
pub mod modrinth;
pub mod spigot;

use anyhow::Result;
use deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use tokio_postgres::NoTls;

pub struct Database {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

impl Database {
    pub async fn create_pool(&self, db_name: &str) -> Result<Pool, CreatePoolError> {
        let mut config = Config::new();
        config.user = Some(self.user.clone());
        config.password = Some(self.password.clone());
        config.host = Some(self.host.clone());
        config.port = Some(self.port);
        config.dbname = Some(db_name.to_string());
        config.create_pool(Some(Runtime::Tokio1), NoTls)
    }
}