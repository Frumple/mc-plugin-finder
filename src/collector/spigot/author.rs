use crate::collector::spigot::{SPIGOT_BASE_URL, SpigotClient};
use crate::cornucopia::queries::spigot_author::{insert_spigot_author, get_highest_spigot_author_id};

use anyhow::Result;
use constcat::concat;
use futures::{future, TryFutureExt};
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use serde::{Serialize, Deserialize};
use std::cell::Cell;
use std::rc::Rc;

const SPIGOT_AUTHORS_URL: &str = concat!(SPIGOT_BASE_URL, "/authors");

const SPIGOT_AUTHORS_REQUEST_FIELDS: &str = "id,name";
const SPIGOT_POPULATE_ALL_AUTHORS_REQUESTS_AHEAD: usize = 2;

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
                sort: self.headers.sort.clone(),
                fields: self.headers.fields.clone()
            }
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct GetSpigotAuthorsRequestHeaders {
    size: u32,
    page: u32,
    sort: String,
    fields: String
}

#[derive(Debug)]
struct GetSpigotAuthorsResponse {
    headers: SpigotGetAuthorsResponseHeaders,
    authors: Vec<SpigotAuthor>
}

#[derive(Debug)]
struct SpigotGetAuthorsResponseHeaders {
    x_page_index: u32,
    x_page_count: u32
}

impl GetSpigotAuthorsResponse {
    fn more_authors_available(&self) -> bool {
        self.headers.x_page_index <= self.headers.x_page_count
    }
}

#[derive(Debug, Deserialize)]
pub struct SpigotAuthor {
    id: i32,
    name: String
}

impl SpigotClient {
    pub async fn populate_all_spigot_authors(&self) -> Result<u32> {
        let request = GetSpigotAuthorsRequest {
            headers: GetSpigotAuthorsRequestHeaders {
                size: 1000,
                page: 1,
                sort: "+id".to_string(),
                fields: SPIGOT_AUTHORS_REQUEST_FIELDS.to_string()
            }
        };

        let count_rc: Rc<Cell<u32>> = Rc::new(Cell::new(0));

        let result = self
            .pages_ahead(SPIGOT_POPULATE_ALL_AUTHORS_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(None, |author| {
                let count_rc_clone = count_rc.clone();
                async move {
                    let db_result = insert_spigot_author()
                        .bind(&self.db_client, &author.id, &author.name)
                        .map_ok(|_ok: u64| ())
                        .map_err(|err: tokio_postgres::Error| anyhow::Error::new(err))
                        .await;

                    if db_result.is_ok() {
                        count_rc_clone.set(count_rc_clone.get() + 1);
                    }

                    db_result
                }
            })
            .await;

        let count = count_rc.get();

        match result {
            Ok(()) => Ok(count),
            Err(err) => Err(err)
        }
    }

    pub async fn populate_new_spigot_authors(&self) -> Result<u32> {
        let highest_author_id = get_highest_spigot_author_id()
            .bind(&self.db_client)
            .one()
            .await?;

        println!("Highest id: {:?}", highest_author_id);

        let get_authors_request = GetSpigotAuthorsRequest {
            headers: GetSpigotAuthorsRequestHeaders {
                size: 100,
                page: 1,
                sort: "-id".to_string(),
                fields: SPIGOT_AUTHORS_REQUEST_FIELDS.to_string()
            }
        };

        let count_rc: Rc<Cell<u32>> = Rc::new(Cell::new(0));

        let result = self
            .pages(get_authors_request)
            .items()
            .try_take_while(|x| future::ready(Ok(x.id > highest_author_id)))
            .try_for_each(|author| {
                let count_rc_clone = count_rc.clone();
                async move {
                    let db_result = insert_spigot_author()
                        .bind(&self.db_client, &author.id, &author.name)
                        .map_ok(|_ok: u64| ())
                        .map_err(|err: tokio_postgres::Error| anyhow::Error::new(err))
                        .await;

                    if db_result.is_ok() {
                        count_rc_clone.set(count_rc_clone.get() + 1);
                    }

                    db_result
                }
            })
            .await;

        let count = count_rc.get();

        match result {
            Ok(()) => Ok(count),
            Err(err) => Err(err)
        }
    }

    async fn get_authors(&self, request: GetSpigotAuthorsRequest) -> Result<GetSpigotAuthorsResponse> {
        self.rate_limiter.until_ready().await;

        let raw_response = self.api_client.get(SPIGOT_AUTHORS_URL)
            .query(&request.headers)
            .send()
            .await?;

        let raw_headers = raw_response.headers();
        let headers = SpigotGetAuthorsResponseHeaders {
            // TODO: Convert from string to int using serde_aux::field_attributes::deserialize_number_from_string
            x_page_index: raw_headers["x-page-index"].to_str()?.parse::<u32>()?,
            x_page_count: raw_headers["x-page-count"].to_str()?.parse::<u32>()?,
        };

        let authors: Vec<SpigotAuthor> = raw_response.json().await?;

        let response = GetSpigotAuthorsResponse {
            headers,
            authors,
        };

        Ok(response)
    }
}

// TODO: Can this be expressed once instead of for both authors and resources?
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