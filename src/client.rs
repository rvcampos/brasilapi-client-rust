use crate::{Rc};

/// Default client for BrasilApi Operations
#[derive(Debug)]
pub struct BrasilApiClient {
    /// The BaseUrl
    pub(crate) base_url: Rc<String>,
}

impl BrasilApiClient {
    pub fn new(base_url: impl Into<String>) -> BrasilApiClient {
        BrasilApiClient {
            base_url: Rc::new(base_url.into())
        }
    }

    /// This method will create the client, using the Default Base URL
    pub fn new_default() -> BrasilApiClient {
        BrasilApiClient::new(crate::constants::DEFAULT_BASE_URL.to_string())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::client::BrasilApiClient;

    pub(crate) fn cli() -> BrasilApiClient {BrasilApiClient::new_default()}
}