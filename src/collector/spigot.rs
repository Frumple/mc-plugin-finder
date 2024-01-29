use anyhow::Result;
use deadpool_postgres::Object;
use governor::{Quota, RateLimiter};
use governor::clock::QuantaClock;
use governor::state::{InMemoryState, NotKeyed};
use nonzero_ext::*;
use reqwest::Client;
use std::num::NonZeroU32;

mod author;

const SPIGOT_USER_AGENT: &str = "analysis";
const SPIGOT_RATE_LIMIT_PER_SECOND: NonZeroU32 = nonzero!(2u32);

const SPIGOT_BASE_URL: &str = "https://api.spiget.org/v2";

#[derive(Debug)]
pub struct SpigotClient {
    api_client: Client,
    db_client: Object,
    rate_limiter: RateLimiter<NotKeyed, InMemoryState, QuantaClock>
}

impl SpigotClient {
    pub fn new(db_client: Object) -> Result<SpigotClient> {
        let api_client = reqwest::Client::builder()
            .user_agent(SPIGOT_USER_AGENT)
            .build()?;

        let quota = Quota::per_second(SPIGOT_RATE_LIMIT_PER_SECOND);
        let rate_limiter = RateLimiter::direct(quota);

        Ok(SpigotClient { api_client, db_client, rate_limiter })
    }
}
