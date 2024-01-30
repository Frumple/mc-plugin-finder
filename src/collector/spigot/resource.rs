use crate::collector::spigot::{SPIGOT_BASE_URL, SpigotClient};
use crate::cornucopia::queries::spigot_resource::{insert_spigot_resource, InsertSpigotResourceParams};

use anyhow::{anyhow, Result};
use constcat::concat;
use cornucopia_async::Params;
use futures::{future, TryFutureExt};
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use regex::Regex;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use std::cell::Cell;
use std::rc::Rc;

const SPIGOT_RESOURCES_URL: &str = concat!(SPIGOT_BASE_URL, "/resources");

const SPIGOT_RESOURCES_REQUEST_FIELDS: &str = "id,name,releaseDate,updateDate,file,author,version,premium,sourceCodeLink";
const SPIGOT_POPULATE_ALL_RESOURCES_REQUESTS_AHEAD: usize = 2;

#[derive(Clone, Debug, Serialize)]
struct GetSpigotResourcesRequest {
    headers: GetSpigotResourcesRequestHeaders,
}

impl RequestAhead for GetSpigotResourcesRequest {
    fn next_request(&self) -> Self {
        Self {
            headers: GetSpigotResourcesRequestHeaders {
                size: self.headers.size,
                page: self.headers.page + 1,
                sort: self.headers.sort.clone(),
                fields: self.headers.fields.clone()
            }
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct GetSpigotResourcesRequestHeaders {
    size: u32,
    page: u32,
    sort: String,
    fields: String
}

#[derive(Debug)]
struct GetSpigotResourcesResponse {
    headers: SpigotGetResourcesResponseHeaders,
    resources: Vec<SpigotResource>
}

impl GetSpigotResourcesResponse {
    fn more_resources_available(&self) -> bool {
        self.headers.x_page_index <= self.headers.x_page_count
    }
}

#[derive(Debug)]
struct SpigotGetResourcesResponseHeaders {
    x_page_index: u32,
    x_page_count: u32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpigotResource {
    id: i32,
    name: String,
    release_date: i64,
    update_date: i64,
    file: Option<SpigotResourceNestedFile>,
    author: SpigotResourceNestedAuthor,
    version: SpigotResourceNestedVersion,
    premium: Option<bool>,
    source_code_link: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SpigotResourceNestedFile {
    url: String
}

#[derive(Debug, Deserialize)]
pub struct SpigotResourceNestedAuthor {
    id: i32
}


#[derive(Debug, Deserialize)]
pub struct SpigotResourceNestedVersion {
    id: i32
}

// #[derive(Clone, Debug, Serialize)]
// struct GetLatestResourceVersionRequest {
//     resource: i32
// }

// #[derive(Debug, Deserialize)]
// struct SpigotResourceVersion {
//     name: String
// }

impl SpigotClient {
    pub async fn populate_all_spigot_resources(&self) -> Result<u32> {
        let request = GetSpigotResourcesRequest {
            headers: GetSpigotResourcesRequestHeaders {
                size: 1000,
                page: 1,
                sort: "+id".to_string(),
                fields: SPIGOT_RESOURCES_REQUEST_FIELDS.to_string()
            }
        };

        let count_rc: Rc<Cell<u32>> = Rc::new(Cell::new(0));

        let result = self
            .pages_ahead(SPIGOT_POPULATE_ALL_RESOURCES_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(None, |resource| {
                let count_rc_clone = count_rc.clone();
                async move {
                    // let latest_resource_version_request = GetLatestResourceVersionRequest { resource: resource.id };
                    // let latest_resource_version_name = self.get_latest_resource_version_name(latest_resource_version_request).await?;

                    if let Some(file) = resource.file {
                        if let Some(slug) = parse_slug_from_file_download_url(&file.url) {
                            let params = InsertSpigotResourceParams {
                                id: resource.id,
                                name: resource.name,
                                slug,
                                release_date: OffsetDateTime::from_unix_timestamp(resource.release_date)?,
                                update_date: OffsetDateTime::from_unix_timestamp(resource.update_date)?,
                                author_id: resource.author.id,
                                version_id: resource.version.id,
                                version_name: None::<String>,
                                premium: resource.premium,
                                source_code_link: resource.source_code_link
                            };

                            let db_result = insert_spigot_resource()
                                .params(&self.db_client, &params)
                                // .map_ok(|_ok: u64| ())
                                // .map_err(|err: tokio_postgres::Error| anyhow::Error::new(err))
                                .await;

                            match db_result {
                                Ok(_) => count_rc_clone.set(count_rc_clone.get() + 1),
                                Err(err) => println!("Skipping resource ID {}: Unable to add resource to database: {}", resource.id, err)
                            }
                            Ok(())
                        } else {
                            println!("Skipping resource ID {}: Unable to parse slug from URL: {}", resource.id, file.url);
                            Ok(())
                        }
                    } else {
                        println!("Skipping resource ID {}: Resource has no file.", resource.id);
                        Ok(())
                    }
                }
            })
            .await;

        let count = count_rc.get();

        match result {
            Ok(()) => Ok(count),
            Err(err) => Err(err)
        }
    }

    async fn get_resources(&self, request: GetSpigotResourcesRequest) -> Result<GetSpigotResourcesResponse> {
        self.rate_limiter.until_ready().await;

        let raw_response = self.api_client.get(SPIGOT_RESOURCES_URL)
            .query(&request.headers)
            .send()
            .await?;

        let raw_headers = raw_response.headers();
        let headers = SpigotGetResourcesResponseHeaders {
            // TODO: Convert from string to int using serde_aux::field_attributes::deserialize_number_from_string
            x_page_index: raw_headers["x-page-index"].to_str()?.parse::<u32>()?,
            x_page_count: raw_headers["x-page-count"].to_str()?.parse::<u32>()?,
        };

        let resources: Vec<SpigotResource> = raw_response.json().await?;

        let response = GetSpigotResourcesResponse {
            headers,
            resources
        };

        Ok(response)
    }

    // async fn get_latest_resource_version_name(&self, request: GetLatestResourceVersionRequest) -> Result<String> {
    //     self.rate_limiter.until_ready().await;

    //     let resource_id = request.resource;
    //     let url = format!("{SPIGOT_BASE_URL}/resources/{resource_id}/versions/latest");

    //     let raw_response = self.api_client.get(url)
    //         .send()
    //         .await?;

    //     let version: SpigotResourceVersion = raw_response.json().await?;

    //     Ok(version.name)
    // }
}

// TODO: Can this be expressed once instead of for both authors and resources?
impl PageTurner<GetSpigotResourcesRequest> for SpigotClient {
    type PageItems = Vec<SpigotResource>;
    type PageError = anyhow::Error;

  async fn turn_page(&self, mut request: GetSpigotResourcesRequest) -> TurnedPageResult<Self, GetSpigotResourcesRequest> {
        println!("Start: {:?}", request);
        let response = self.get_resources(request.clone()).await?;
        println!("End: {:?}", request);

        if response.more_resources_available() {
            request.headers.page += 1;
            Ok(TurnedPage::next(response.resources, request))
        } else {
            Ok(TurnedPage::last(response.resources))
        }
    }
}

fn parse_slug_from_file_download_url(url: &str) -> Option<String> {
    let re = Regex::new(r"resources/(\S+\.\d+)/download.*").unwrap();
    let caps = re.captures(url)?;
    Some(String::from(&caps[1]))
}