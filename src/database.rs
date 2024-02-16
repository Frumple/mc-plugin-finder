use deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use tokio_postgres::NoTls;

pub struct Database {
    user: String,
    password: String,
    host: String,
    port: u16,
    dbname: String
}

impl Database {
    pub fn new(user: String, password: String, host: String, port: u16, dbname: String) -> Self {
        Self {
            user,
            password,
            host,
            port,
            dbname
        }
    }

    pub async fn create_pool(&self) -> Result<Pool, CreatePoolError> {
        let mut config = Config::new();
        config.user = Some(self.user.clone());
        config.password = Some(self.password.clone());
        config.host = Some(self.host.clone());
        config.port = Some(self.port);
        config.dbname = Some(self.dbname.clone());
        config.create_pool(Some(Runtime::Tokio1), NoTls)
    }
}