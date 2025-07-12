use reqwest::{Client, Response};
use std::time::Duration;
use tracing::{debug, error};

use crate::error::{AppResult, AppError};

/// HTTP client wrapper with common functionality
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new() -> AppResult<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Free-Deep-Research-System/1.0.0")
            .build()
            .map_err(|e| AppError::network(e.to_string()))?;
        
        Ok(Self { client })
    }
    
    /// Make a GET request
    pub async fn get(&self, url: &str) -> AppResult<Response> {
        debug!("Making GET request to: {}", url);
        
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::network(e.to_string()))?;
        
        Ok(response)
    }
    
    /// Make a POST request with JSON body
    pub async fn post_json<T: serde::Serialize>(&self, url: &str, body: &T) -> AppResult<Response> {
        debug!("Making POST request to: {}", url);
        
        let response = self.client
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(|e| AppError::network(e.to_string()))?;
        
        Ok(response)
    }
    
    /// Make a request with custom headers
    pub async fn request_with_headers(
        &self,
        method: reqwest::Method,
        url: &str,
        headers: &[(&str, &str)],
    ) -> AppResult<Response> {
        debug!("Making {} request to: {}", method, url);
        
        let mut request = self.client.request(method, url);
        
        for (key, value) in headers {
            request = request.header(*key, *value);
        }
        
        let response = request
            .send()
            .await
            .map_err(|e| AppError::network(e.to_string()))?;
        
        Ok(response)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}
