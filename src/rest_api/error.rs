use reqwest::{header::InvalidHeaderValue, Error as ReqwestError};
use serde_json::Error as SerdeError;
use std::fmt::{self, Display, Formatter};
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum TonApiError {
    InvalidInput(InvalidInput),
    ProcessingError(ProcessingError),
    RateLimitExceeded,
    ApiError { code: u16, message: String },
}

#[derive(Debug)]
pub enum InvalidInput {
    HeaderValue(InvalidHeaderValue),
    UrlParse(UrlParseError),
}

#[derive(Debug)]
pub enum ProcessingError {
    Network(ReqwestError),
    Deserialization(SerdeError),
}

impl fmt::Display for TonApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(err) => write!(f, "Invalid input: {}", err),
            Self::ProcessingError(err) => write!(f, "Processing error: {}", err),
            Self::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            Self::ApiError { code, message } => {
                write!(f, "Api error (code: {}): {}", code, message)
            }
        }
    }
}

impl Display for InvalidInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvalidInput::HeaderValue(err) => write!(f, "Invalid header value: {}", err),
            InvalidInput::UrlParse(err) => write!(f, "URL parse error: {}", err),
        }
    }
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network(err) => write!(f, "Network error: {}", err),
            Self::Deserialization(err) => write!(f, "Deserialization error: {}", err),
        }
    }
}

impl From<InvalidHeaderValue> for TonApiError {
    fn from(err: InvalidHeaderValue) -> Self {
        Self::InvalidInput(InvalidInput::HeaderValue(err))
    }
}

impl From<UrlParseError> for TonApiError {
    fn from(err: UrlParseError) -> Self {
        Self::InvalidInput(InvalidInput::UrlParse(err))
    }
}

impl From<ReqwestError> for TonApiError {
    fn from(err: ReqwestError) -> Self {
        Self::ProcessingError(ProcessingError::Network(err))
    }
}

impl From<SerdeError> for TonApiError {
    fn from(err: SerdeError) -> Self {
        Self::ProcessingError(ProcessingError::Deserialization(err))
    }
}
