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

const HANGAR_IS_LIVE: bool = true;
const HANGAR_DEV_BASE_URL: &str = "https://hangar.papermc.dev/api/v1/";
const HANGAR_LIVE_BASE_URL: &str = "https://hangar.papermc.io/api/v1/";

const HANGAR_USER_AGENT: &str = concat!("Frumple/mc-plugin-finder/", env!("CARGO_PKG_VERSION"), " (contact@mcpluginfinder.com)");
const HANGAR_RATE_LIMIT_PER_SECOND: NonZeroU32 = nonzero!(4u32);

#[derive(Debug)]
pub struct HangarServer;

impl HttpServer for HangarServer {
    async fn new() -> Self {
        Self
    }

    fn base_url(&self) -> Url {
        let url = if HANGAR_IS_LIVE { HANGAR_LIVE_BASE_URL } else { HANGAR_DEV_BASE_URL };
        Url::parse(url)
          .expect("Hangar base URL could not be parsed")
    }
}

#[derive(Debug)]
pub struct HangarClient<T> {
    api_client: Client,
    rate_limiter: RateLimiter<NotKeyed, InMemoryState, QuantaClock>,
    http_server: T
}

impl<T> HangarClient<T> {
    pub fn new(http_server: T) -> Result<HangarClient<T>> {
        let api_client = reqwest::Client::builder()
            .user_agent(HANGAR_USER_AGENT)
            .build()?;

        let quota = Quota::per_second(HANGAR_RATE_LIMIT_PER_SECOND);
        let rate_limiter = RateLimiter::direct(quota);

        Ok(Self { api_client, rate_limiter, http_server })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wiremock::MockServer;

    #[derive(Debug)]
    pub struct HangarTestServer {
        mock_server: MockServer
    }

    impl HangarTestServer {
        pub fn mock(&self) -> &MockServer {
            &self.mock_server
        }
    }

    impl HttpServer for HangarTestServer {
        async fn new() -> Self {
            Self {
                mock_server: MockServer::start().await
            }
        }

        fn base_url(&self) -> Url {
            Url::parse(&self.mock_server.uri())
                .expect("Hangar mock server base URL could not be parsed")
        }
    }
}