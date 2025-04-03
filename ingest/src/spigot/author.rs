use crate::HttpServer;
use crate::spigot::SpigotClient;
use mc_plugin_finder::database::ingest_log::{IngestLog, IngestLogAction, IngestLogRepository, IngestLogItem, insert_ingest_log};
use mc_plugin_finder::database::spigot::author::{SpigotAuthor, insert_spigot_author};


use anyhow::Result;
use deadpool_postgres::Pool;
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;
use time::OffsetDateTime;
use tracing::{info, warn, instrument};

const SPIGOT_AUTHORS_REQUEST_FIELDS: &str = "id,name";
const SPIGOT_AUTHORS_REQUESTS_AHEAD: usize = 2;
const SPIGOT_AUTHORS_CONCURRENT_FUTURES: usize = 10;

#[derive(Clone, Debug, Serialize)]
struct GetSpigotAuthorsRequest {
    size: u32,
    page: u32,
    sort: String,
    fields: String
}

impl GetSpigotAuthorsRequest {
    fn create_populate_request() -> Self {
        Self {
            size: 100,
            page: 1,
            sort: "+id".to_string(),
            fields: SPIGOT_AUTHORS_REQUEST_FIELDS.to_string()
        }
    }
}

impl RequestAhead for GetSpigotAuthorsRequest {
    fn next_request(&self) -> Self {
        Self {
            size: self.size,
            page: self.page + 1,
            sort: self.sort.clone(),
            fields: self.fields.clone()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct GetSpigotAuthorsResponse {
    headers: GetSpigotAuthorsResponseHeaders,
    authors: Vec<IncomingSpigotAuthor>
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct GetSpigotAuthorsResponseHeaders {
    x_page_index: u32,
    x_page_count: u32
}

impl GetSpigotAuthorsResponse {
    fn more_authors_available(&self) -> bool {
        self.headers.x_page_index <= self.headers.x_page_count
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingSpigotAuthor {
    id: i32,
    name: String
}

impl From<IncomingSpigotAuthor> for SpigotAuthor {
    fn from(author: IncomingSpigotAuthor) -> Self {
        SpigotAuthor {
            id: author.id,
            name: author.name
        }
    }
}

#[derive(Debug, Error)]
enum GetSpigotAuthorsError {
    #[error("Could not get Spigot authors {request:?}: Received unexpected status code {status_code}")]
    UnexpectedStatusCode {
        request: GetSpigotAuthorsRequest,
        status_code: u16
    }
}

impl<T> SpigotClient<T> where T: HttpServer + Send + Sync {
    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn populate_spigot_authors(&self, db_pool: &Pool) -> Result<()> {
        info!("Populating Spigot authors...");

        let request = GetSpigotAuthorsRequest::create_populate_request();
        let count = Arc::new(AtomicU32::new(0));
        let date_started = OffsetDateTime::now_utc();

        let result = self
            .pages_ahead(SPIGOT_AUTHORS_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(SPIGOT_AUTHORS_CONCURRENT_FUTURES, |incoming_author| process_incoming_author(incoming_author, db_pool, &count))
            .await;

        let date_finished = OffsetDateTime::now_utc();
        let items_processed = count.load(Ordering::Relaxed);

        let ingest_log = IngestLog {
            action: IngestLogAction::Populate,
            repository: IngestLogRepository::Spigot,
            item: IngestLogItem::Author,
            date_started,
            date_finished,
            items_processed: items_processed.try_into()?,
            success: result.is_ok()
        };
        insert_ingest_log(db_pool, &ingest_log).await?;

        info!("Spigot authors populated: {}", items_processed);

        result
    }

    #[instrument(
        skip(self)
    )]
    async fn get_authors_from_api(&self, request: GetSpigotAuthorsRequest) -> Result<GetSpigotAuthorsResponse> {
        self.rate_limiter.until_ready().await;

        let url = self.http_server.base_url().join("authors")?;

        let raw_response = self.api_client.get(url)
            .query(&request)
            .send()
            .await?;

        let status = raw_response.status();
        if status == StatusCode::OK {
            let raw_headers = raw_response.headers();
            let headers = GetSpigotAuthorsResponseHeaders {
                // TODO: Convert from string to int using serde_aux::field_attributes::deserialize_number_from_string
                x_page_index: raw_headers["x-page-index"].to_str()?.parse::<u32>()?,
                x_page_count: raw_headers["x-page-count"].to_str()?.parse::<u32>()?,
            };

            let authors: Vec<IncomingSpigotAuthor> = raw_response.json().await?;

            let response = GetSpigotAuthorsResponse {
                headers,
                authors,
            };

            Ok(response)
        } else {
            Err(
                GetSpigotAuthorsError::UnexpectedStatusCode {
                    request,
                    status_code: status.into()
                }.into()
            )
        }
    }
}

impl<T> PageTurner<GetSpigotAuthorsRequest> for SpigotClient<T> where T: HttpServer + Send + Sync {
    type PageItems = Vec<IncomingSpigotAuthor>;
    type PageError = anyhow::Error;

    async fn turn_page(&self, mut request: GetSpigotAuthorsRequest) -> TurnedPageResult<Self, GetSpigotAuthorsRequest> {
        let response = self.get_authors_from_api(request.clone()).await?;

        if response.more_authors_available() {
            request.page += 1;
            Ok(TurnedPage::next(response.authors, request))
        } else {
            Ok(TurnedPage::last(response.authors))
        }
    }
}

async fn process_incoming_author(incoming_author: IncomingSpigotAuthor, db_pool: &Pool, count: &Arc<AtomicU32>) -> Result<()> {
    let db_result = insert_spigot_author(db_pool, &incoming_author.into()).await;

    match db_result {
        Ok(_) => {
            count.fetch_add(1, Ordering::Relaxed);
        }
        Err(err) => warn!("{}", err)
    }

    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::spigot::test::SpigotTestServer;

    use speculoos::prelude::*;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[tokio::test]
    async fn should_get_authors_from_api() -> Result<()> {
        // Arrange
        let spigot_server = SpigotTestServer::new().await;

        let request = GetSpigotAuthorsRequest::create_populate_request();

        let expected_response = GetSpigotAuthorsResponse {
            headers: GetSpigotAuthorsResponseHeaders {
                x_page_index: 1,
                x_page_count: 10
            },
            authors: create_test_authors()
        };

        let response_template = ResponseTemplate::new(200)
            .append_header("x-page-index", expected_response.headers.x_page_index.to_string().as_str())
            .append_header("x-page-count", expected_response.headers.x_page_count.to_string().as_str())
            .set_body_json(expected_response.authors.clone());

        Mock::given(method("GET"))
            .and(path("/authors"))
            .and(query_param("size", request.size.to_string().as_str()))
            .and(query_param("page", expected_response.headers.x_page_index.to_string().as_str()))
            .and(query_param("sort", request.sort.as_str()))
            .and(query_param("fields", SPIGOT_AUTHORS_REQUEST_FIELDS))
            .respond_with(response_template)
            .mount(spigot_server.mock())
            .await;

        // Act
        let spigot_client = SpigotClient::new(spigot_server)?;
        let response = spigot_client.get_authors_from_api(request).await;

        // Assert
        assert_that(&response).is_ok().is_equal_to(expected_response);

        Ok(())
    }

    pub fn create_test_authors() -> Vec<IncomingSpigotAuthor> {
        vec![
            IncomingSpigotAuthor {
                id: 1,
                name: "author-1".to_string()
            },
            IncomingSpigotAuthor {
                id: 2,
                name: "author-2".to_string()
            }
        ]
    }
}