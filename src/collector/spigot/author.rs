use crate::collector::HttpServer;
use crate::collector::spigot::SpigotClient;
use crate::cornucopia::queries::spigot_author::{InsertSpigotAuthorParams, insert_spigot_author, get_highest_spigot_author_id};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Object;
use futures::future;
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use serde::{Serialize, Deserialize};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

const SPIGOT_AUTHORS_REQUEST_FIELDS: &str = "id,name";
const SPIGOT_POPULATE_AUTHORS_REQUESTS_AHEAD: usize = 2;

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
            size: 1000,
            page: 1,
            sort: "+id".to_string(),
            fields: SPIGOT_AUTHORS_REQUEST_FIELDS.to_string()
        }
    }

    fn create_update_request() -> Self {
        Self {
            size: 100,
            page: 1,
            sort: "-id".to_string(),
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
    authors: Vec<SpigotAuthor>
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
pub struct SpigotAuthor {
    id: i32,
    name: String
}

impl<T> SpigotClient<T> where T: HttpServer + Send + Sync {
    pub async fn populate_spigot_authors(&self, db_client: &Object) -> Result<u32> {
        let request = GetSpigotAuthorsRequest::create_populate_request();

        let count_rc: Rc<Cell<u32>> = Rc::new(Cell::new(0));

        let result = self
            .pages_ahead(SPIGOT_POPULATE_AUTHORS_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(None, |author| {
                let count_rc_clone = count_rc.clone();
                async move {
                    let params = InsertSpigotAuthorParams {
                        id: author.id,
                        name: author.name
                    };

                    let db_result = insert_spigot_author()
                        .params(db_client, &params)
                        .await;

                    match db_result {
                        Ok(_) => count_rc_clone.set(count_rc_clone.get() + 1),
                        Err(err) => println!("Skipping author ID {}: Unable to add author to database: {}", author.id, err)
                    }
                    Ok(())
                }
            })
            .await;

        let count = count_rc.get();

        match result {
            Ok(()) => Ok(count),
            Err(err) => Err(err)
        }
    }

    pub async fn update_spigot_authors(&self, db_client: &Object) -> Result<u32> {
        let highest_author_id = get_highest_spigot_author_id()
            .bind(db_client)
            .one()
            .await?;

        println!("Highest id: {:?}", highest_author_id);

        let request = GetSpigotAuthorsRequest::create_update_request();

        let count_rc: Rc<Cell<u32>> = Rc::new(Cell::new(0));

        let result = self
            .pages(request)
            .items()
            .try_take_while(|x| future::ready(Ok(x.id > highest_author_id)))
            .try_for_each(|author| {
                let count_rc_clone = count_rc.clone();
                async move {
                    let params = InsertSpigotAuthorParams {
                        id: author.id,
                        name: author.name
                    };

                    let db_result = insert_spigot_author()
                        .params(db_client, &params)
                        .await;

                    match db_result {
                        Ok(_) => count_rc_clone.set(count_rc_clone.get() + 1),
                        Err(err) => println!("Skipping author ID {}: Unable to add author to database: {}", author.id, err)
                    }
                    Ok(())
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

        let url = self.http_server.base_url().join("authors")?;

        let raw_response = self.api_client.get(url)
            .query(&request)
            .send()
            .await?;

        let raw_headers = raw_response.headers();
        let headers = GetSpigotAuthorsResponseHeaders {
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

impl<T> PageTurner<GetSpigotAuthorsRequest> for SpigotClient<T> where T: HttpServer + Send + Sync {
    type PageItems = Vec<SpigotAuthor>;
    type PageError = anyhow::Error;

    async fn turn_page(&self, mut request: GetSpigotAuthorsRequest) -> TurnedPageResult<Self, GetSpigotAuthorsRequest> {
        println!("API Start: {:?}", request);
        let start = Instant::now();
        let response = self.get_authors(request.clone()).await?;
        let duration = start.elapsed();
        println!("API End: {:?} in {:?}", request, duration);

        if response.more_authors_available() {
            request.page += 1;
            Ok(TurnedPage::next(response.authors, request))
        } else {
            Ok(TurnedPage::last(response.authors))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::collector::spigot::test::SpigotTestServer;

    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[tokio::test]
    async fn should_get_authors() -> Result<()> {
        let spigot_server = SpigotTestServer::new().await;

        let request = GetSpigotAuthorsRequest::create_populate_request();

        let expected_response = GetSpigotAuthorsResponse {
            headers: GetSpigotAuthorsResponseHeaders {
                x_page_index: 1,
                x_page_count: 10
            },
            authors: vec![
                SpigotAuthor {
                    id: 1000,
                    name: "testuser-1000".to_string()
                },
                SpigotAuthor {
                    id: 1001,
                    name: "testuser-1001".to_string()
                }
            ]
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

        let spigot_client = SpigotClient::new(spigot_server)?;
        let response = spigot_client.get_authors(request).await?;

        assert_eq!(response, expected_response);

        Ok(())
    }
}