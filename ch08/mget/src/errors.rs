use core::{error, fmt};

use crate::{dns, http::UpstreamError};

#[derive(Debug)]
pub enum MgetError {
    UrlParseError(url::ParseError),
    UpstreamError(UpstreamError),
    DnsError(dns::DnsError),
}

impl error::Error for MgetError {}

impl fmt::Display for MgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MgetError::UrlParseError(e) => write!(f, "UrlParseError: {}", e),
            MgetError::UpstreamError(e) => write!(f, "UpstreamError: {}", e),
            MgetError::DnsError(e) => write!(f, "DnsError: {}", e),
        }
    }
}

impl From<url::ParseError> for MgetError {
    fn from(e: url::ParseError) -> Self {
        MgetError::UrlParseError(e)
    }
}

impl From<UpstreamError> for MgetError {
    fn from(e: UpstreamError) -> Self {
        MgetError::UpstreamError(e)
    }
}

impl From<dns::DnsError> for MgetError {
    fn from(e: dns::DnsError) -> Self {
        MgetError::DnsError(e)
    }
}
