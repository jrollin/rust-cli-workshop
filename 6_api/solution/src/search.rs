use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

use cli_table::{Cell, Table, TableDisplay};

#[derive(Debug, Deserialize)]
pub struct Article {
    pub title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub hits: Vec<Article>,
}

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Error when calling URL")]
    HTTPClient(#[from] reqwest::Error),
    #[error("Error status code")]
    HTTPStatus(StatusCode),
    #[error("Error parsing Json")]
    Parsing(String),
}

pub fn search_news(api_url: &str, keyword: &str) -> Result<SearchResult, SearchError> {
    let query = format!("{api_url}/api/v1/search?query={}", keyword);
    let http_response = reqwest::blocking::get(query)?;
    if !http_response.status().is_success() {
        return Err(SearchError::HTTPStatus(http_response.status()));
    }
    http_response
        .json::<SearchResult>()
        .map_err(|_| SearchError::Parsing("Parsing error".to_string()))
}

pub fn display_news(search_results: SearchResult) -> Result<TableDisplay, std::io::Error> {
    search_results
        .hits
        .into_iter()
        .map(|article| vec![article.title.cell()])
        .collect::<Vec<_>>()
        .table()
        .title(vec!["Title".cell()])
        .display()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_error() {
        let results = search_news("http://localhost:11111", "rust");
        assert!(matches!(results, Err(SearchError::HTTPClient(..))));
    }

    #[test]
    fn bad_response() {
        // Request a new server from the pool
        let mut server = mockito::Server::new();

        // Use one of these addresses to configure your client
        // let host = server.host_with_port();
        let url = server.url();

        // Create a mock
        let mock = server
            .mock("GET", "/api/v1/search?query=rust")
            .with_status(400)
            .create();

        let results = search_news(url.as_str(), "rust");
        assert!(matches!(results, Err(SearchError::HTTPStatus(..))));
        // You can use `Mock::assert` to verify that your mock was called
        mock.assert();
    }
    #[test]
    fn bad_parsing() {
        // Request a new server from the pool
        let mut server = mockito::Server::new();

        // Use one of these addresses to configure your client
        // let host = server.host_with_port();
        let url = server.url();

        // Create a mock
        let mock = server
            .mock("GET", "/api/v1/search?query=rust")
            .with_status(200)
            .with_body("not_json")
            .create();

        let results = search_news(url.as_str(), "rust");
        assert!(matches!(results, Err(SearchError::Parsing(..))));
        // You can use `Mock::assert` to verify that your mock was called
        mock.assert();
    }
}
