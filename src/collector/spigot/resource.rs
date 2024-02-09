use crate::collector::HttpServer;
use crate::collector::spigot::SpigotClient;
use crate::cornucopia::queries::spigot_resource::{InsertSpigotResourceParams, insert_spigot_resource};

use anyhow::Result;
use cornucopia_async::Params;
use deadpool_postgres::Object;
use futures::future;
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use regex::Regex;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

const SPIGOT_RESOURCES_REQUEST_FIELDS: &str = "id,name,tag,releaseDate,updateDate,file,author,version,premium,sourceCodeLink";
const SPIGOT_POPULATE_RESOURCES_REQUESTS_AHEAD: usize = 2;

#[derive(Clone, Debug, Serialize)]
struct GetSpigotResourcesRequest {
    size: u32,
    page: u32,
    sort: String,
    fields: String
}

impl GetSpigotResourcesRequest {
    fn create_populate_request() -> Self {
        Self {
            size: 1000,
            page: 1,
            sort: "+id".to_string(),
            fields: SPIGOT_RESOURCES_REQUEST_FIELDS.to_string()
        }
    }
}

impl RequestAhead for GetSpigotResourcesRequest {
    fn next_request(&self) -> Self {
        Self {
            size: self.size,
            page: self.page + 1,
            sort: self.sort.clone(),
            fields: self.fields.clone()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct GetSpigotResourcesRequestHeaders {

}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct GetSpigotResourcesResponse {
    headers: GetSpigotResourcesResponseHeaders,
    resources: Vec<SpigotResource>
}

impl GetSpigotResourcesResponse {
    fn more_resources_available(&self) -> bool {
        self.headers.x_page_index <= self.headers.x_page_count
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct GetSpigotResourcesResponseHeaders {
    x_page_index: u32,
    x_page_count: u32
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpigotResource {
    id: i32,
    name: String,
    tag: String,
    release_date: i64,
    update_date: i64,
    file: Option<SpigotResourceNestedFile>,
    author: SpigotResourceNestedAuthor,
    version: SpigotResourceNestedVersion,
    premium: Option<bool>,
    source_code_link: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpigotResourceNestedFile {
    url: String
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpigotResourceNestedAuthor {
    id: i32
}


#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

impl<T> SpigotClient<T> where T: HttpServer + Send + Sync {
    pub async fn populate_spigot_resources(&self, db_client: &Object) -> Result<u32> {
        let request = GetSpigotResourcesRequest::create_populate_request();

        let count_rc: Rc<Cell<u32>> = Rc::new(Cell::new(0));

        let result = self
            .pages_ahead(SPIGOT_POPULATE_RESOURCES_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(None, |resource| {
                let count_rc_clone = count_rc.clone();
                async move {
                    // let latest_resource_version_request = GetLatestResourceVersionRequest { resource: resource.id };
                    // let latest_resource_version_name = self.get_latest_resource_version_name(latest_resource_version_request).await?;

                    if let Some(file) = resource.file {
                        if let Some(slug) = extract_slug_from_file_download_url(&file.url) {
                            let params = InsertSpigotResourceParams {
                                id: resource.id,
                                name: resource.name,
                                tag: resource.tag,
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
                                .params(db_client, &params)
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

        let url = self.http_server.base_url().join("resources")?;
        let raw_response = self.api_client.get(url)
            .query(&request)
            .send()
            .await?;

        let raw_headers = raw_response.headers();
        let headers = GetSpigotResourcesResponseHeaders {
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

impl<T> PageTurner<GetSpigotResourcesRequest> for SpigotClient<T> where T: HttpServer + Send + Sync {
    type PageItems = Vec<SpigotResource>;
    type PageError = anyhow::Error;

  async fn turn_page(&self, mut request: GetSpigotResourcesRequest) -> TurnedPageResult<Self, GetSpigotResourcesRequest> {
        println!("API Start: {:?}", request);
        let start = Instant::now();
        let response = self.get_resources(request.clone()).await?;
        let duration = start.elapsed();
        println!("API End: {:?} in {:?}", request, duration);

        if response.more_resources_available() {
            request.page += 1;
            Ok(TurnedPage::next(response.resources, request))
        } else {
            Ok(TurnedPage::last(response.resources))
        }
    }
}

fn extract_slug_from_file_download_url(url: &str) -> Option<String> {
    let re = Regex::new(r"resources/(\S+\.\d+)/download.*").unwrap();
    let caps = re.captures(url)?;
    Some(String::from(&caps[1]))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::collector::spigot::test::SpigotTestServer;

    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[test]
    fn should_extract_slug_from_file_download_url() {
        let url = "resources/luckperms.28140/download?version=511529";
        let slug = extract_slug_from_file_download_url(url);
        assert_eq!(slug, Some("luckperms.28140".to_string()));
    }

    #[test]
    fn should_not_extract_slug_if_file_download_url_has_no_name() {
        let url = "resources/40087/download?version=156669";
        let slug = extract_slug_from_file_download_url(url);
        assert_eq!(slug, None);
    }

    #[tokio::test]
    async fn should_get_resources() -> Result<()> {
        let spigot_server = SpigotTestServer::new().await;

        let request = GetSpigotResourcesRequest::create_populate_request();

        let expected_response = GetSpigotResourcesResponse {
            headers: GetSpigotResourcesResponseHeaders {
                x_page_index: 1,
                x_page_count: 10
            },
            resources: vec![
                SpigotResource {
                    id: 2000,
                    name: "testresource-2000".to_string(),
                    tag: "testresource-2000-tag".to_string(),
                    release_date: OffsetDateTime::now_utc().unix_timestamp(),
                    update_date: OffsetDateTime::now_utc().unix_timestamp(),
                    file: Some(SpigotResourceNestedFile {
                        url: "resources/luckperms.28140/download?version=511529".to_string()
                    }),
                    author: SpigotResourceNestedAuthor {
                        id: 1000
                    },
                    version: SpigotResourceNestedVersion {
                        id: 511529
                    },
                    premium: Some(false),
                    source_code_link: Some("https://github.com/lucko/LuckPerms".to_string())
                },
                SpigotResource {
                    id: 2001,
                    name: "testresource-2001".to_string(),
                    tag: "testresource-2001-tag".to_string(),
                    release_date: OffsetDateTime::now_utc().unix_timestamp(),
                    update_date: OffsetDateTime::now_utc().unix_timestamp(),
                    file: Some(SpigotResourceNestedFile {
                        url: "resources/essentialsx.9089/download?version=50842".to_string()
                    }),
                    author: SpigotResourceNestedAuthor {
                        id: 1001
                    },
                    version: SpigotResourceNestedVersion {
                        id: 50842
                    },
                    premium: Some(false),
                    source_code_link: Some("https://github.com/EssentialsX/Essentials".to_string())
                }
            ]
        };

        let response_template = ResponseTemplate::new(200)
            .append_header("x-page-index", expected_response.headers.x_page_index.to_string().as_str())
            .append_header("x-page-count", expected_response.headers.x_page_count.to_string().as_str())
            .set_body_json(expected_response.resources.clone());

            Mock::given(method("GET"))
            .and(path("/resources"))
            .and(query_param("size", request.size.to_string().as_str()))
            .and(query_param("page", expected_response.headers.x_page_index.to_string().as_str()))
            .and(query_param("sort", request.sort.as_str()))
            .and(query_param("fields", SPIGOT_RESOURCES_REQUEST_FIELDS))
            .respond_with(response_template)
            .mount(spigot_server.mock())
            .await;

        let spigot_client = SpigotClient::new(spigot_server)?;
        let response = spigot_client.get_resources(request).await?;

        assert_eq!(response, expected_response);

        Ok(())
    }
}