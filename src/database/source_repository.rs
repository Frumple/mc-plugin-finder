use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceRepository {
    pub host: String,
    pub owner: String,
    pub name: String
}

impl SourceRepository {
    pub fn url(&self) -> String {
        format!("https://{}/{}/{}", self.host, self.owner, self.name)
    }
}

pub fn extract_source_repository_from_url(url: &str) -> Option<SourceRepository> {
    let parse_result = Url::parse(url);

    if let Ok(parsed_url) = parse_result {
        if let Some(host_str) = parsed_url.host_str() {
            if let Some(host) = remove_www_from_host(host_str) {
                if is_source_repository_host(host) {
                    if let Some(path_segments) = parsed_url.path_segments().map(|x| x.collect::<Vec<_>>()) {
                        if path_segments.len() >= 2 {
                            let source_repository = SourceRepository {
                                host: host.to_string(),
                                owner: path_segments[0].to_string(),
                                name: path_segments[1].to_string()
                            };

                            return Some(source_repository)
                        }
                    }
                }
            }
        }
    }

    None
}

fn remove_www_from_host(host: &str) -> Option<&str> {
    if host.starts_with("www.") {
        return host.get(4..)
    }

    Some(host)
}

fn is_source_repository_host(host: &str) -> bool {
    matches!(host,
        "github.com" |
        "gitlab.com" |
        "bitbucket.org" |
        "codeberg.org"
    )
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::*;
    use speculoos::prelude::*;

    #[rstest]
    #[case::github_url("https://github.com/Frumple/foo", SourceRepository {host: "github.com".to_string(), owner: "Frumple".to_string(), name: "foo".to_string()})]
    #[case::gitlab_url("https://gitlab.com/Frumple/bar", SourceRepository {host: "gitlab.com".to_string(), owner: "Frumple".to_string(), name: "bar".to_string()})]
    #[case::bitbucket_url("https://bitbucket.org/Frumple/baz", SourceRepository {host: "bitbucket.org".to_string(), owner: "Frumple".to_string(), name: "baz".to_string()})]
    #[case::codeberg_url("https://codeberg.org/Frumple/qux", SourceRepository {host: "codeberg.org".to_string(), owner: "Frumple".to_string(), name: "qux".to_string()})]
    #[case::host_with_leading_www("https://www.github.com/Frumple/foo", SourceRepository {host: "github.com".to_string(), owner: "Frumple".to_string(), name: "foo".to_string()})]
    #[case::url_with_trailing_slash("https://github.com/Frumple/foo/", SourceRepository {host: "github.com".to_string(), owner: "Frumple".to_string(), name: "foo".to_string()})]
    #[case::url_with_trailing_path("https://github.com/Frumple/foo/wiki", SourceRepository {host: "github.com".to_string(), owner: "Frumple".to_string(), name: "foo".to_string()})]
    fn should_extract_source_repository_from_url(#[case] url: &str, #[case] expected_repo: SourceRepository) {
        let repo = extract_source_repository_from_url(url);
        assert_that(&repo).is_some().is_equal_to(expected_repo);
    }

    #[rstest]
    #[case::git_repository_url_without_owner_and_name("https://github.com")]
    #[case::git_repository_url_without_name("https://github.com/Frumple")]
    #[case::non_git_repository_url("https://pastebin.com/AAAAAAAA")]
    fn should_not_extract_source_repository(#[case] url: &str) {
        let repo = extract_source_repository_from_url(url);
        assert_that(&repo).is_none();
    }
}