use log::debug;
use reqwest::{header::HeaderMap, Client, Method, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use super::{error::TonApiError, models::ApiResponse, query_params::QueryParams};

pub struct BaseRestApiClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl BaseRestApiClient {
    pub fn new(client: Client, base_url: String, api_key: Option<String>) -> Self {
        Self {
            client,
            base_url,
            api_key,
        }
    }

    pub async fn get<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        endpoint: String,
        params: Option<QueryParams>,
        headers: Option<HeaderMap>,
    ) -> Result<T, TonApiError> {
        let request_builder = self.build_request(Method::GET, endpoint, params, headers)?;
        self.send_request(request_builder).await
    }

    pub async fn post_json<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        endpoint: String,
        params: Option<QueryParams>,
        body: Option<impl Serialize>,
        headers: Option<HeaderMap>,
    ) -> Result<T, TonApiError> {
        let request_builder = self.build_request(Method::POST, endpoint, params, headers)?;
        let request_builder = if let Some(body) = body {
            request_builder.json(&body)
        } else {
            request_builder
        };
        self.send_request(request_builder).await
    }

    pub async fn put_bytes<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        endpoint: String,
        params: Option<QueryParams>,
        body: Option<Vec<u8>>,
        headers: Option<HeaderMap>,
    ) -> Result<T, TonApiError> {
        let request_builder = self.build_request(Method::PUT, endpoint, params, headers)?;
        let request_builder = if let Some(body) = body {
            request_builder.json(&body)
        } else {
            request_builder
        };
        self.send_request(request_builder).await
    }

    fn build_request(
        &self,
        method: Method,
        endpoint: String,
        params: Option<QueryParams>,
        headers: Option<HeaderMap>,
    ) -> Result<RequestBuilder, TonApiError> {
        let headers = headers.unwrap_or_default();
        let query_params = params.unwrap_or_default();

        let url = format!("{}{}", self.base_url, endpoint);
        let url_with_params = reqwest::Url::parse_with_params(&url, query_params)?;
        let mut request_builder = match method {
            Method::GET => self.client.get(url_with_params).headers(headers),
            Method::POST => self.client.post(url_with_params).headers(headers),
            Method::PUT => self.client.put(url_with_params).headers(headers),
            _ => unimplemented!(),
        };
        debug!("Request after processing: {:?}", request_builder);

        if let Some(ref api_key) = self.api_key {
            request_builder = request_builder.bearer_auth(api_key);
        }

        Ok(request_builder)
    }

    async fn send_request<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        request_builder: RequestBuilder,
    ) -> Result<T, TonApiError> {
        let response = request_builder.send().await?;
        debug!("Received response: {:?}", response);

        let status = response.status();

        if let Some(length) = response.content_length() {
            if status.is_success() && length == 0 {
                return Ok(serde_json::from_value(serde_json::Value::Null)
                    .expect("Can't serialize null value"));
            }
        }

        let response_text = response.text().await?;
        debug!("Response text: {}", response_text);

        let response_body: ApiResponse<T> = serde_json::from_str(&response_text)?;
        debug!("Response body: {:?}", response_body);

        match response_body {
            ApiResponse::Success { result } => Ok(result),
            ApiResponse::Error { error } => match status {
                StatusCode::TOO_MANY_REQUESTS => Err(TonApiError::RateLimitExceeded),
                _ => Err(TonApiError::ApiError {
                    code: status.as_u16(),
                    message: error,
                }),
            },
        }
    }
}
