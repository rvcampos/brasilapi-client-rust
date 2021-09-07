
use crate::errors::Error;
use log::{Level, debug, error, log_enabled, trace, warn};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str};

pub(crate) async fn get<Input: Serialize + std::fmt::Debug, Output: 'static + DeserializeOwned>(
    url: &str
) -> Result<Output, Error> {
    use isahc::*;

    trace!("GET Req on {}", url);

    let mut resp = isahc::get_async(url)
    .await
    .map_err(|_|crate::errors::Error::NotExpectedRequestError)
    .unwrap();

    let status = resp.status().as_u16();
    let mut body = resp.text().await.map_err(|e| crate::errors::Error::HttpError(e.into()))?;

    if body.is_empty() {
        body = "null".to_string();
    }

    parse_response(status, body)

}

fn parse_response<Output: DeserializeOwned>(
    status_code: u16,
    body: String,
) -> Result<Output, Error> {
    if (200..=204).contains(&status_code) {
        match from_str::<Output>(&body) {
            Ok(output) => {
                trace!("Request succeed");
                if log_enabled!(Level::Debug) {
                    debug!("Response: {}", &body);
                }

                return Ok(output);
            }
            Err(e) => {
                error!("Request succeed but failed to parse response");
                return Err(Error::SerdeJsonError(e));
            }
        };
    }
    warn!("Expected success response code, got {}", status_code);
    match from_str(&body) {
        Ok(e) => {
            trace!("Request failed");
            if log_enabled!(Level::Debug) {
                debug!("Response: {}", &body);
            }

            Err(Error::from(&e))
        },
        Err(e) => Err(Error::SerdeJsonError(e)),
    }
}