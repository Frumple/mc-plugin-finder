use anyhow::Result;
use constcat::concat;
use futures::stream::TryStreamExt;
use governor::{Quota, RateLimiter};
use governor::clock::QuantaClock;
use governor::state::{InMemoryState, NotKeyed};
use nonzero_ext::*;
use page_turner::prelude::*;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::num::NonZeroU32;

const SPIGOT_USER_AGENT: &str = "analysis";
const SPIGOT_RATE_LIMIT_PER_SECOND: NonZeroU32 = nonzero!(1u32);

const SPIGOT_BASE_URL: &str = "https://api.spiget.org/v2";
const SPIGOT_AUTHORS_URL: &str = concat!(SPIGOT_BASE_URL, "/authors");

const SPIGOT_AUTHORS_REQUEST_SIZE: u32 = 1000;
const SPIGOT_AUTHORS_REQUEST_FIELDS: &str = "id,name";
const SPIGOT_AUTHORS_REQUESTS_AHEAD: usize = 2;

#[derive(Clone, Debug, Serialize)]
struct GetSpigotAuthorsRequest {
    headers: GetSpigotAuthorsRequestHeaders,
}

impl RequestAhead for GetSpigotAuthorsRequest {
    fn next_request(&self) -> Self {
        Self {
            headers: GetSpigotAuthorsRequestHeaders {
                size: self.headers.size,
                page: self.headers.page + 1,
                fields: self.headers.fields.clone()
            }
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct GetSpigotAuthorsRequestHeaders {
    size: u32,
    page: u32,
    fields: String
}

#[derive(Debug)]
struct SpigotGetAuthorsResponse {
    headers: SpigotGetAuthorsResponseHeaders,
    authors: Vec<SpigotAuthor>
}

#[derive(Debug)]
struct SpigotGetAuthorsResponseHeaders {
    x_page_index: u32,
    x_page_count: u32
}

impl SpigotGetAuthorsResponse {
    fn more_authors_available(&self) -> bool {
        self.headers.x_page_index <= self.headers.x_page_count
    }
}

#[derive(Deserialize, Debug)]
pub struct SpigotAuthor {
  id: i32,
  name: String
}

#[derive(Debug)]
pub struct SpigotClient {
    client: Client,
    rate_limiter: RateLimiter<NotKeyed, InMemoryState, QuantaClock>
}

impl SpigotClient {
    pub fn new() -> Result<SpigotClient> {
        let client = reqwest::Client::builder()
            .user_agent(SPIGOT_USER_AGENT)
            .build()?;

        let quota = Quota::per_second(SPIGOT_RATE_LIMIT_PER_SECOND);
        let rate_limiter = RateLimiter::direct(quota);

        Ok(SpigotClient { client, rate_limiter })
    }

    pub async fn get_spigot_authors(&self) -> Result<Vec<SpigotAuthor>> {
        let get_authors_request = GetSpigotAuthorsRequest {
            headers: GetSpigotAuthorsRequestHeaders {
                size: SPIGOT_AUTHORS_REQUEST_SIZE,
                page: 1,
                fields: SPIGOT_AUTHORS_REQUEST_FIELDS.to_string()
            }
        };

        self
            .pages_ahead(SPIGOT_AUTHORS_REQUESTS_AHEAD, Limit::None, get_authors_request)
            .items()
            .try_collect()
            .await
    }

    async fn get_authors(&self, request: GetSpigotAuthorsRequest) -> Result<SpigotGetAuthorsResponse> {
        self.rate_limiter.until_ready().await;

        let raw_response = self.client.get(SPIGOT_AUTHORS_URL)
            .query(&request.headers)
            .send()
            .await?;

        let raw_headers = raw_response.headers();
        let headers = SpigotGetAuthorsResponseHeaders {
            x_page_index: raw_headers["x-page-index"].to_str()?.parse::<u32>()?,
            x_page_count: raw_headers["x-page-count"].to_str()?.parse::<u32>()?,
        };

        let authors: Vec<SpigotAuthor> = raw_response.json().await?;

        let response = SpigotGetAuthorsResponse {
            authors,
            headers
        };

        Ok(response)
    }
}

impl PageTurner<GetSpigotAuthorsRequest> for SpigotClient {
    type PageItems = Vec<SpigotAuthor>;
    type PageError = anyhow::Error;

    async fn turn_page(&self, mut request: GetSpigotAuthorsRequest) -> TurnedPageResult<Self, GetSpigotAuthorsRequest> {
        println!("Start: {:?}", request);
        let response = self.get_authors(request.clone()).await?;
        println!("End: {:?}", request);

        if response.more_authors_available() {
            request.headers.page += 1;
            Ok(TurnedPage::next(response.authors, request))
        } else {
            Ok(TurnedPage::last(response.authors))
        }
    }
}