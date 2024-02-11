use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const API_BASE_URL: &str = "https://www.googleapis.com/customsearch/v1";

#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    link: String,
    title: String,
    snippet: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResponse {
    items: Vec<SearchResult>,
    // Add other relevant fields from the response if needed
}

#[derive(Debug)]
pub enum GSearchError {
    RequestError(reqwest::Error),
    ApiResponseError(reqwest::StatusCode),
    JsonError(serde_json::Error), // Add JsonError variant for better error handling
}

pub struct GSearch {
    api_key: String,
    cx: String,
}

impl GSearch {
    pub fn new(api_key: &str, cx: &str) -> Self {
        GSearch {
            api_key: api_key.to_string(),
            cx: cx.to_string(),
        }
    }
    pub fn search(&self, query: &str) -> Result<SearchResponse, GSearchError> {
        let url = format!(
            "{}?key={}&cx={}&q={}",
            API_BASE_URL, self.api_key, self.cx, query
        );

        let response = Client::new()
            .get(&url)
            .send()
            .map_err(GSearchError::RequestError)?;

        if response.status().is_success() {
            let json_str = response.text().map_err(GSearchError::RequestError)?;
            let result: SearchResponse =
                serde_json::from_str(&json_str).map_err(GSearchError::JsonError)?;
            Ok(result)
        } else {
            Err(GSearchError::ApiResponseError(response.status()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let api_key = "AIzaSyAaXEsNq7G4GOjO2RSdGTWsCQnQK3GmPvM";
        let cx: &str = "f5900ec03a27c4e6c";
        let gsearch = GSearch::new(api_key, cx);

        let query = "lectures";
        let result = gsearch.search(query);

        assert!(result.is_ok());
    }
}
