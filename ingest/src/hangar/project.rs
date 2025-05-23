use crate::HttpServer;
use crate::hangar::HangarClient;
use crate::hangar::version::{IncomingHangarVersion, apply_incoming_hangar_version_to_hangar_project};
use mc_plugin_finder::database::ingest_log::{IngestLog, IngestLogAction, IngestLogRepository, IngestLogItem, insert_ingest_log};
use mc_plugin_finder::database::hangar::project::{HangarProject, upsert_hangar_project};
use mc_plugin_finder::database::source_repository::{SourceRepository, extract_source_repository_from_url};

use anyhow::Result;
use deadpool_postgres::Pool;
use futures::future;
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;
use time::format_description::well_known::Rfc3339;
use tracing::{info, warn, instrument};

const HANGAR_PROJECTS_REQUESTS_AHEAD: usize = 2;
const HANGAR_PROJECTS_CONCURRENT_FUTURES: usize = 10;

#[derive(Clone, Debug, Serialize)]
struct GetHangarProjectsRequest {
    limit: u32,
    offset: u32,
    sort: String
}

impl GetHangarProjectsRequest {
    fn create_request() -> Self {
        Self {
            limit: 25,
            offset: 0,
            // Get projects in reverse recently updated order.
            // "-updated" is not documented in the official Hangar v1 docs.
            sort: "-updated".to_string()
        }
    }
}

impl RequestAhead for GetHangarProjectsRequest {
    fn next_request(&self) -> Self {
        Self {
            limit: self.limit,
            offset: self.offset + self.limit,
            sort: self.sort.clone()
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct GetHangarProjectsResponse {
    pagination: HangarResponsePagination,
    result: Vec<IncomingHangarProject>
}

impl GetHangarProjectsResponse {
    fn more_projects_available(&self) -> bool {
        self.pagination.offset + self.pagination.limit < self.pagination.count
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingHangarProject {
    name: String,
    description: String,
    namespace: IncomingHangarProjectNamespace,
    created_at: String,
    last_updated: String,
    stats: IncomingHangarProjectStats,
    visibility: String,
    avatar_url: String,
    settings: IncomingHangarProjectSettings
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingHangarProjectNamespace {
    owner: String,
    slug: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingHangarProjectStats {
    downloads: i32,
    stars: i32,
    watchers: i32
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingHangarProjectSettings {
    links: Vec<IncomingHangarProjectLinkGroup>,
    tags: Vec<String>,
    keywords: Vec<String>
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingHangarProjectLinkGroup {
    id: u32,
    #[serde(rename = "type")]
    r_type: String,
    title: Option<String>,
    links: Vec<IncomingHangarProjectLink>
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingHangarProjectLink {
    id: u32,
    name: String,
    url: Option<String>
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HangarResponsePagination {
    pub limit: u32,
    pub offset: u32,
    pub count: u32
}

#[derive(Debug, Error)]
enum GetHangarProjectsError {
    #[error("Could not get Hangar projects {request:?}: Received unexpected status code {status_code}")]
    UnexpectedStatusCode {
        request: GetHangarProjectsRequest,
        status_code: u16
    }
}

impl<T> HangarClient<T> where T: HttpServer + Send + Sync {
    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn populate_hangar_projects(&self, db_pool: &Pool) -> Result<()> {
        info!("Populating Hangar projects...");

        let request = GetHangarProjectsRequest::create_request();
        let count = Arc::new(AtomicU32::new(0));
        let date_started = OffsetDateTime::now_utc();

        let result= self
            .pages_ahead(HANGAR_PROJECTS_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(HANGAR_PROJECTS_CONCURRENT_FUTURES, |incoming_project| self.process_incoming_project(incoming_project, db_pool, &count, false))
            .await;

        let date_finished = OffsetDateTime::now_utc();
        let items_processed = count.load(Ordering::Relaxed);

        let ingest_log = IngestLog {
            action: IngestLogAction::Populate,
            repository: IngestLogRepository::Hangar,
            item: IngestLogItem::Project,
            date_started,
            date_finished,
            items_processed: items_processed.try_into()?,
            success: result.is_ok()
        };
        insert_ingest_log(db_pool, &ingest_log).await?;

        info!("Hangar projects populated: {}", items_processed);

        result
    }

    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn update_hangar_projects(&self, db_pool: &Pool, update_date_later_than: OffsetDateTime) -> Result<()> {
        info!("Updating Hangar projects since: {}", update_date_later_than);

        let request = GetHangarProjectsRequest::create_request();
        let count = Arc::new(AtomicU32::new(0));
        let date_started = OffsetDateTime::now_utc();

        let result = self
            .pages_ahead(HANGAR_PROJECTS_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_take_while(|x| future::ready(Ok(OffsetDateTime::parse(x.last_updated.as_str(), &Rfc3339).unwrap() > update_date_later_than)))
            .try_for_each_concurrent(HANGAR_PROJECTS_CONCURRENT_FUTURES, |incoming_project| self.process_incoming_project(incoming_project, db_pool, &count, true))
            .await;

        let date_finished = OffsetDateTime::now_utc();
        let items_processed = count.load(Ordering::Relaxed);

        let ingest_log = IngestLog {
            action: IngestLogAction::Update,
            repository: IngestLogRepository::Hangar,
            item: IngestLogItem::Project,
            date_started,
            date_finished,
            items_processed: items_processed.try_into()?,
            success: result.is_ok()
        };
        insert_ingest_log(db_pool, &ingest_log).await?;

        info!("Hangar projects updated: {}", items_processed);

        result
    }

    async fn process_incoming_project(&self, incoming_project: IncomingHangarProject, db_pool: &Pool, count: &Arc<AtomicU32>, get_version: bool) -> Result<()> {
        let mut incoming_version: Option<IncomingHangarVersion> = None;

        if get_version {
            let version_result = self.get_latest_hangar_project_version_from_api(&incoming_project.namespace.slug).await;

            match version_result {
                Ok(version) => incoming_version = Some(version),
                Err(err) => warn!("{}", err)
            }
        }

        let convert_result = convert_incoming_project(incoming_project, incoming_version).await;

        match convert_result {
            Ok(project) => {
                let db_result = upsert_hangar_project(db_pool, &project).await;

                match db_result {
                    Ok(_) => {
                        count.fetch_add(1, Ordering::Relaxed);
                    },
                    Err(err) => warn!("{}", err)
                }
            }
            Err(err) => warn!("{}", err)
        }

        Ok(())
    }

    #[instrument(
        skip(self)
    )]
    async fn get_projects_from_api(&self, request: GetHangarProjectsRequest) -> Result<GetHangarProjectsResponse> {
        self.rate_limiter.until_ready().await;

        let url = self.http_server.base_url().join("projects")?;
        let raw_response = self.api_client.get(url)
            .query(&request)
            .send()
            .await?;

        let status = raw_response.status();
        if status == StatusCode::OK {
            let response: GetHangarProjectsResponse = raw_response.json().await?;

            Ok(response)
        } else {
            Err(
                GetHangarProjectsError::UnexpectedStatusCode {
                    request,
                    status_code: status.into()
                }.into()
            )
        }
    }
}

impl<T> PageTurner<GetHangarProjectsRequest> for HangarClient<T> where T: HttpServer + Send + Sync {
    type PageItems = Vec<IncomingHangarProject>;
    type PageError = anyhow::Error;

    async fn turn_page(&self, mut request: GetHangarProjectsRequest) -> TurnedPageResult<Self, GetHangarProjectsRequest> {
        let response = self.get_projects_from_api(request.clone()).await?;

        if response.more_projects_available() {
            request.offset += request.limit;
            Ok(TurnedPage::next(response.result, request))
        } else {
            Ok(TurnedPage::last(response.result))
        }
    }
}

async fn convert_incoming_project(incoming_project: IncomingHangarProject, incoming_version: Option<IncomingHangarVersion>) -> Result<HangarProject> {
    let source_code_link = find_source_code_link(incoming_project.settings);

    let mut project = HangarProject {
        slug: incoming_project.namespace.slug,
        author: incoming_project.namespace.owner,
        name: incoming_project.name,
        description: incoming_project.description,
        date_created: OffsetDateTime::parse(&incoming_project.created_at, &Rfc3339)?,
        date_updated: OffsetDateTime::parse(&incoming_project.last_updated, &Rfc3339)?,
        latest_minecraft_version: None,
        downloads: incoming_project.stats.downloads,
        stars: incoming_project.stats.stars,
        watchers: incoming_project.stats.watchers,
        visibility: incoming_project.visibility,
        icon_url: incoming_project.avatar_url,
        version_name: None,
        source_url: source_code_link.clone(),
        source_repository: None
    };

    if let Some(version) = incoming_version {
        apply_incoming_hangar_version_to_hangar_project(&mut project, &version);
    }

    if let Some(url) = source_code_link {
        let option_repo = extract_source_repository_from_url(url.as_str());

        if let Some(repo) = option_repo {
            project.source_repository = Some(SourceRepository {
                host: repo.host,
                owner: repo.owner,
                name: repo.name,
                id: None
            });
        }
    }

    Ok(project)
}

fn find_source_code_link(settings: IncomingHangarProjectSettings) -> Option<String> {
    for link_group in settings.links {
        for link in link_group.links {
            if link.name.contains("Source") {
                return link.url
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hangar::test::HangarTestServer;
    use crate::hangar::version::test::create_test_version;

    use speculoos::prelude::*;
    use time::macros::datetime;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[tokio::test]
    async fn should_get_projects_from_api() -> Result<()> {
        // Arrange
        let hangar_server = HangarTestServer::new().await;

        let request = GetHangarProjectsRequest::create_request();

        let expected_response = GetHangarProjectsResponse {
            pagination: HangarResponsePagination {
                limit: 25,
                offset: 50,
                count: 100
            },
            result: create_test_projects()
        };

        let response_template = ResponseTemplate::new(200)
            .set_body_json(expected_response.clone());

        Mock::given(method("GET"))
            .and(path("/projects"))
            .and(query_param("limit", request.limit.to_string().as_str()))
            .and(query_param("offset", request.offset.to_string().as_str()))
            .and(query_param("sort", request.sort.as_str()))
            .respond_with(response_template)
            .mount(hangar_server.mock())
            .await;

        // Act
        let hangar_client = HangarClient::new(hangar_server)?;
        let response = hangar_client.get_projects_from_api(request).await;

        // Assert
        assert_that(&response).is_ok().is_equal_to(expected_response);

        Ok(())
    }

    #[tokio::test]
    async fn should_process_incoming_project() -> Result<()> {
        // Arrange
        let incoming_project = create_test_projects()[0].clone();
        let incoming_version = create_test_version().clone();
        let version_name = "v1.2.3";

        // Act
        let project = convert_incoming_project(incoming_project, Some(incoming_version)).await?;

        // Assert
        let expected_project = HangarProject {
            slug: "foo".to_string(),
                author: "alice".to_string(),
                name: "foo-hangar".to_string(),
                description: "foo-hangar-description".to_string(),
                date_created: datetime!(2022-01-01 0:00 UTC),
                date_updated: datetime!(2022-02-03 0:00 UTC),
                latest_minecraft_version: Some("1.21.3".to_string()),
                downloads: 100,
                stars: 200,
                watchers: 200,
                visibility: "public".to_string(),
                icon_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
                version_name: Some(version_name.to_string()),
                source_url: Some("https://github.com/alice/foo".to_string()),
                source_repository: Some(SourceRepository {
                    host: "github.com".to_string(),
                    owner: "alice".to_string(),
                    name: "foo".to_string(),
                    id: None
                })
        };

        assert_that(&project).is_equal_to(expected_project);

        Ok(())
    }

    fn create_test_projects() -> Vec<IncomingHangarProject> {
        vec![
            IncomingHangarProject {
                name: "foo-hangar".to_string(),
                description: "foo-hangar-description".to_string(),
                namespace: IncomingHangarProjectNamespace {
                    owner: "alice".to_string(),
                    slug: "foo".to_string()
                },
                created_at: "2022-01-01T00:00:00Z".to_string(),
                last_updated: "2022-02-03T00:00:00Z".to_string(),
                stats: IncomingHangarProjectStats {
                    downloads: 100,
                    stars: 200,
                    watchers: 200
                },
                visibility: "public".to_string(),
                avatar_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
                settings: IncomingHangarProjectSettings {
                    links: create_test_project_links( SourceRepository { host: "github.com".to_string(), owner: "alice".to_string(), name: "foo".to_string(), id: None } ),
                    tags: vec!["ADDON".to_string(), "SUPPORTS_FOLIA".to_string()],
                    keywords: vec!["foo".to_string(), "fi".to_string()]
                }
            },
            IncomingHangarProject {
                name: "bar-hangar".to_string(),
                description: "bar-hangar-description".to_string(),
                namespace: IncomingHangarProjectNamespace {
                    owner: "bob".to_string(),
                    slug: "bar".to_string()
                },
                created_at: "2022-01-02T00:00:00Z".to_string(),
                last_updated: "2022-02-02T00:00:00Z".to_string(),
                stats: IncomingHangarProjectStats {
                    downloads: 300,
                    stars: 100,
                    watchers: 300
                },
                visibility: "public".to_string(),
                avatar_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
                settings: IncomingHangarProjectSettings {
                    links: create_test_project_links( SourceRepository { host: "gitlab.com".to_string(), owner: "bob".to_string(), name: "bar".to_string(), id: None } ),
                    tags: vec!["ADDON".to_string(), "SUPPORTS_FOLIA".to_string()],
                    keywords: vec!["foo".to_string(), "fi".to_string()]
                }
            },
        ]
    }

    fn create_test_project_links(repository: SourceRepository) -> Vec<IncomingHangarProjectLinkGroup> {
        vec![
            IncomingHangarProjectLinkGroup {
                id: 0,
                r_type: "top".to_string(),
                title: Some("top".to_string()),
                links: vec![
                    IncomingHangarProjectLink {
                        id: 1,
                        name: "Issues".to_string(),
                        url: Some(repository.url() + "/issues")
                    },
                    IncomingHangarProjectLink {
                        id: 2,
                        name: "Source".to_string(),
                        url: Some(repository.url())
                    },
                    IncomingHangarProjectLink {
                        id: 3,
                        name: "Support".to_string(),
                        url: Some(repository.url() + "/discussions")
                    },
                    IncomingHangarProjectLink {
                        id: 4,
                        name: "Wiki".to_string(),
                        url: Some(repository.url() + "/wiki")
                    }
                ]
            }
        ]
    }
}