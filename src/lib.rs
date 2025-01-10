pub mod entity;
pub mod memories;
pub mod personality;
pub mod relationship;
pub mod soul;
pub mod value;
pub mod voice;

use reqwest::{
    header::{self, ACCEPT, CONTENT_TYPE},
    Error, Response,
};
pub use soul::Soul;
use std::fmt::format;

use serde::Serialize;

#[derive(Debug)]
pub struct Soulgraph {
    client: reqwest::Client,
    base_url: String,
}

#[derive(Default)]
pub struct SoulgraphBuilder {
    key: Option<String>,
    url: Option<String>,
}

impl SoulgraphBuilder {
    pub fn new() -> SoulgraphBuilder {
        SoulgraphBuilder {
            key: None,
            url: None,
        }
    }

    pub fn api_key(mut self, key: &str) -> SoulgraphBuilder {
        self.key = Some(key.to_owned());
        self
    }

    pub fn base_url(mut self, url: &str) -> SoulgraphBuilder {
        self.url = Some(url.to_owned());
        self
    }

    pub fn build(self) -> Soulgraph {
        match (self.key, self.url) {
            (Some(key), Some(url)) => Soulgraph::new(key.as_str(), url.as_str()),
            (None, _) => panic!("Missing api key"),
            (_, None) => panic!("Missing base url"),
        }
    }
}

impl Soulgraph {
    pub fn builder() -> SoulgraphBuilder {
        SoulgraphBuilder::default()
    }

    pub fn new(api_key: &str, base_url: &str) -> Soulgraph {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-API-KEY", api_key.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(ACCEPT, "application/json".parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to construct http client");

        Soulgraph {
            client,
            base_url: base_url.to_owned(),
        }
    }

    pub async fn get(self, endpoint: &str) -> Result<Response, Error> {
        let url = format(format_args!("{}{}", self.base_url, endpoint));
        self.client.get(url).send().await
    }

    pub async fn post<T: Serialize>(self, endpoint: &str, json: &T) -> Result<Response, Error> {
        let url = format(format_args!("{}{}", self.base_url, endpoint));
        self.client.post(url).json(json).send().await
    }

    pub async fn patch<T: Serialize>(self, endpoint: &str, json: &T) -> Result<Response, Error> {
        let url = format(format_args!("{}{}", self.base_url, endpoint));
        self.client.patch(url).json(json).send().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_new() {
        let builder = SoulgraphBuilder::new();
        assert!(builder.key.is_none());
        assert!(builder.url.is_none());
    }

    #[test]
    fn test_builder_with_api_key() {
        let builder = SoulgraphBuilder::new().api_key("test-key");
        assert_eq!(builder.key, Some("test-key".to_string()));
    }

    #[test]
    fn test_builder_with_base_url() {
        let builder = SoulgraphBuilder::new().base_url("http://test.com");
        assert_eq!(builder.url, Some("http://test.com".to_string()));
    }

    #[test]
    fn test_builder_complete() {
        let soulgraph = SoulgraphBuilder::new()
            .api_key("test-key")
            .base_url("http://test.com")
            .build();

        assert_eq!(soulgraph.base_url, "http://test.com");
    }

    #[test]
    #[should_panic(expected = "Missing api key")]
    fn test_builder_missing_key() {
        SoulgraphBuilder::new().base_url("http://test.com").build();
    }

    #[test]
    #[should_panic(expected = "Missing base url")]
    fn test_builder_missing_url() {
        SoulgraphBuilder::new().api_key("test-key").build();
    }
}
