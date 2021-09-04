use std::error::Error;

#[cfg(feature = "async")]
use reqwest::Client;

#[cfg(feature = "sync")]
use reqwest::blocking::Client;
use serde::de;

use crate::{BrasilApiClientError, ServiceError};


/// Default client for BrasilApi Operations
pub struct BrasilApiClient {
    /// The Reqwest Http_Client
    pub http_cli: Client,
    /// The BaseUrl
    pub base_url: String
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::client::BrasilApiClient;

    pub(crate) fn cli() -> BrasilApiClient {BrasilApiClient::new_default()}
}

const DEFAULT_BASE_URL : &str = "https://brasilapi.com.br/api";

impl BrasilApiClient {
    /**
    Constructor
     */
    pub fn new(base_url: String) -> Self {
        Self {
            http_cli: Client::new(),
            base_url
        }
    }

    /**
    Create a new client, however
    using the default URL
     */
    pub fn new_default() -> Self {
        Self::new(DEFAULT_BASE_URL.to_string())
    }

    /**
    Helper Get Method already wrapping the custom errors
    */
    pub fn get_helper<T>(&self, service_url: String) -> Result<T, Box<dyn Error>>
    where T : de::DeserializeOwned {
        let fulluri = format!("{}/{}", self.base_url, service_url);
        let resp = self.http_cli.get(fulluri).send();
        
        if resp.is_err() {
            return Err(Box::new(BrasilApiClientError::UnexpectedError));
        }
        let http_resp = resp?;
        let succ = http_resp.status().is_success().clone();
        
        if succ {
            let suc = http_resp.json::<T>()?;
            return Ok(suc);
        }

        return Err(Box::new(BrasilApiClientError::ServiceError(http_resp.json::<ServiceError>()
        .unwrap_or_else(|_op| ServiceError::new("Err")))));
    }
}