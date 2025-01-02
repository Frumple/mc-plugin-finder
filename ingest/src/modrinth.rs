use crate::HttpServer;

use anyhow::Result;
use governor::{Quota, RateLimiter};
use governor::clock::QuantaClock;
use governor::state::{InMemoryState, NotKeyed};
use nonzero_ext::*;
use reqwest::Client;
use std::num::NonZeroU32;
use url::Url;

mod project;
mod version;

const MODRINTH_IS_LIVE: bool = true;
const MODRINTH_STAGING_BASE_URL: &str = "https://staging-api.modrinth.com/v2/";
const MODRINTH_LIVE_BASE_URL: &str = "https://api.modrinth.com/v2/";

const MODRINTH_USER_AGENT: &str = concat!("Frumple/mc-plugin-finder/", env!("CARGO_PKG_VERSION"), " (contact@mcpluginfinder.com)");
const MODRINTH_RATE_LIMIT_PER_SECOND: NonZeroU32 = nonzero!(4u32);

#[derive(Debug)]
pub struct ModrinthServer;

impl HttpServer for ModrinthServer {
    async fn new() -> Self {
        Self
    }

    fn base_url(&self) -> Url {
        let url = if MODRINTH_IS_LIVE { MODRINTH_LIVE_BASE_URL } else { MODRINTH_STAGING_BASE_URL };
        Url::parse(url)
          .expect("Modrinth base URL could not be parsed")
    }
}

#[derive(Debug)]
pub struct ModrinthClient<T> {
    api_client: Client,
    rate_limiter: RateLimiter<NotKeyed, InMemoryState, QuantaClock>,
    http_server: T
}

impl<T> ModrinthClient<T> {
    pub fn new(http_server: T) -> Result<ModrinthClient<T>> {
        let api_client = reqwest::Client::builder()
            .user_agent(MODRINTH_USER_AGENT)
            .build()?;

        let quota = Quota::per_second(MODRINTH_RATE_LIMIT_PER_SECOND);
        let rate_limiter = RateLimiter::direct(quota);

        Ok(Self { api_client, rate_limiter, http_server })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wiremock::MockServer;

    #[derive(Debug)]
    pub struct ModrinthTestServer {
        mock_server: MockServer
    }

    impl ModrinthTestServer {
        pub fn mock(&self) -> &MockServer {
            &self.mock_server
        }
    }

    impl HttpServer for ModrinthTestServer {
        async fn new() -> Self {
            Self {
                mock_server: MockServer::start().await
            }
        }

        fn base_url(&self) -> Url {
            Url::parse(&self.mock_server.uri())
                .expect("Modrinth mock server base URL could not be parsed")
        }
    }
}