use url::Url;

#[derive(Debug, PartialEq)]
pub struct SourceRepository {
    pub host: String,
    pub owner: String,
    pub name: String
}

pub fn extract_source_repository_from_url(url: &str) -> Option<SourceRepository> {
    let parse_result = Url::parse(url);

    if let Ok(parsed_url) = parse_result {
        if let Some(host) = parsed_url.host_str() {
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

    None
}

fn is_source_repository_host(host: &str) -> bool {
    matches!(host, "github.com" | "gitlab.com" | "bitbucket.org")
}

#[cfg(test)]
mod test {
    use super::*;

    use speculoos::prelude::*;

    #[test]
    fn should_extract_source_repository_from_github_url() {
        let expected_repo = SourceRepository {
            host: "github.com".to_string(),
            owner: "Frumple".to_string(),
            name: "foo".to_string()
        };

        let url = "https://github.com/Frumple/foo";
        let repo = extract_source_repository_from_url(url);
        assert_that(&repo).is_some().is_equal_to(expected_repo);
    }

    #[test]
    fn should_extract_source_repository_from_gitlab_url() {
        let expected_repo = SourceRepository {
            host: "gitlab.com".to_string(),
            owner: "Frumple".to_string(),
            name: "bar".to_string()
        };

        let url = "https://gitlab.com/Frumple/bar";
        let repo = extract_source_repository_from_url(url);
        assert_that(&repo).is_some().is_equal_to(expected_repo);
    }

    #[test]
    fn should_extract_source_repository_from_bitbucket_url() {
        let expected_repo = SourceRepository {
            host: "bitbucket.org".to_string(),
            owner: "Frumple".to_string(),
            name: "baz".to_string()
        };

        let url = "https://bitbucket.org/Frumple/baz";
        let repo = extract_source_repository_from_url(url);
        assert_that(&repo).is_some().is_equal_to(expected_repo);
    }

    #[test]
    fn should_extract_repository_from_url_with_trailing_slash() {
        let expected_repo = SourceRepository {
            host: "github.com".to_string(),
            owner: "Frumple".to_string(),
            name: "foo".to_string()
        };

        let url = "https://github.com/Frumple/foo/";
        let repo = extract_source_repository_from_url(url);
        assert_that(&repo).is_some().is_equal_to(expected_repo);
    }

    #[test]
    fn should_extract_repository_from_url_with_trailing_path() {
        let expected_repo = SourceRepository {
            host: "github.com".to_string(),
            owner: "Frumple".to_string(),
            name: "foo".to_string()
        };

        let url = "https://github.com/Frumple/foo/wiki";
        let repo = extract_source_repository_from_url(url);
        assert_that(&repo).is_some().is_equal_to(expected_repo);
    }

    #[test]
    fn should_not_extract_source_repository_from_invalid_github_url() {
        let url = "https://github.com";
        let repo = extract_source_repository_from_url(url);
        assert_that(&repo).is_none();
    }

    #[test]
    fn should_not_extract_source_repository_from_other_url() {
        let url = "https://pastebin.com/AAAAAAAA";
        let repo = extract_source_repository_from_url(url);
        assert_that(&repo).is_none();
    }
}