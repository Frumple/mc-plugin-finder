use crate::HttpServer;

use anyhow::Result;
use governor::{Quota, RateLimiter};
use governor::clock::QuantaClock;
use governor::state::{InMemoryState, NotKeyed};
use nonzero_ext::*;
use reqwest::Client;
use std::num::NonZeroU32;
use url::Url;

mod author;
mod resource;
mod version;

const SPIGOT_BASE_URL: &str = "https://api.spiget.org/v2/";

const SPIGOT_USER_AGENT: &str = "mc-plugin-finder (contact@mcpluginfinder.com)";
const SPIGOT_RATE_LIMIT_PER_SECOND: NonZeroU32 = nonzero!(5u32);

#[derive(Debug)]
pub struct SpigotServer;
impl HttpServer for SpigotServer {
    async fn new() -> Self {
        Self
    }

    fn base_url(&self) -> Url {
        Url::parse(SPIGOT_BASE_URL)
          .expect("Spigot base URL could not be parsed")
    }
}

#[derive(Debug)]
pub struct SpigotClient<T> {
    api_client: Client,
    rate_limiter: RateLimiter<NotKeyed, InMemoryState, QuantaClock>,
    http_server: T
}

impl<T> SpigotClient<T> {
    pub fn new(http_server: T) -> Result<SpigotClient<T>> {
        let api_client = reqwest::Client::builder()
            .user_agent(SPIGOT_USER_AGENT)
            .build()?;

        let quota = Quota::per_second(SPIGOT_RATE_LIMIT_PER_SECOND);
        let rate_limiter = RateLimiter::direct(quota);

        Ok(Self { api_client, rate_limiter, http_server })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wiremock::MockServer;

    #[derive(Debug)]
    pub struct SpigotTestServer {
        mock_server: MockServer
    }

    impl SpigotTestServer {
        pub fn mock(&self) -> &MockServer {
            &self.mock_server
        }
    }

    impl HttpServer for SpigotTestServer {
        async fn new() -> Self {
            Self {
                mock_server: MockServer::start().await
            }
        }

        fn base_url(&self) -> Url {
            Url::parse(&self.mock_server.uri())
                .expect("Spigot mock server base URL could not be parsed")
        }
    }
}
