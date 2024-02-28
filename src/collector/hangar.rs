use crate::collector::HttpServer;

use anyhow::Result;
use governor::{Quota, RateLimiter};
use governor::clock::QuantaClock;
use governor::state::{InMemoryState, NotKeyed};
use nonzero_ext::*;
use reqwest::Client;
use std::num::NonZeroU32;
use url::Url;

mod project;

const HANGAR_USER_AGENT: &str = "mc-plugin-finder";
const HANGAR_RATE_LIMIT_PER_SECOND: NonZeroU32 = nonzero!(2u32);

#[derive(Debug)]
pub struct HangarServer;
impl HttpServer for HangarServer {
    async fn new() -> Self {
        Self
    }

    fn base_url(&self) -> Url {
        // TODO: Switch to Hangar production when ready
        // Url::parse("https://hangar.papermc.io/api/v1/")
        Url::parse("https://hangar.papermc.dev/api/v1/")
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