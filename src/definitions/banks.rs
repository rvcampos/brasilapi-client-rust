use crate::{Rc, client::*, constants::banks::*, errors::*, request::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// The Banks data struct
pub struct BankResponseData {
    /// The ISPB code
    pub ispb: String,
    /// The bank Name
    pub name: String,
    /// The bank Code
    pub code: i16,
    /// The bank FULL_NAME
    pub full_name: String,
}

impl PartialEq for BankResponseData {
    fn eq(&self, other: &Self) -> bool {
        self.ispb == other.ispb && self.name == other.name && self.code == other.code && self.full_name == other.full_name
    }
}

impl BrasilApiClient {
    /// Get all banks
    pub async fn get_banks(&self) -> Result<Vec<BankResponseData>, Error> {
        Ok(get::<(), Vec<BankResponseData>>(
            &format!("{}/{}", self.base_url, SVC_URL)
        ).await?)
    }

    /**
    Get Banks by code <br />
    Example: 33 - Santander
    */
    pub async fn get_banks_by_code(&self, code: &i16) -> Result<BankResponseData, Error> {
        if code < MIN_CODE || code > MAX_CODE {
            return Err(Error::InvalidInputRangeError
                {
                    name: "code".to_string(),
                    min: 1, 
                    max: 999
                })?
        }

        
        Ok(get::<(), BankResponseData>(
            &format!("{}/{}/{}", self.base_url, SVC_URL, code)
        ).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::tests::cli;
    use futures_await_test::async_test;

    #[async_test]
    async fn test_invalid_min_range() {
        let code = MIN_CODE - 1;
        let resp = cli().get_banks_by_code(&code).await;
        assert!(resp.is_err());
    }

    #[async_test]
    async fn test_invalid_max_range() {
        let code = MAX_CODE + 1;
        let resp = cli().get_banks_by_code(&code).await;
        assert!(resp.is_err());
    }

    #[async_test]
    async fn test_valid_banco_do_brasil() {
        let resp = cli().get_banks_by_code(&1).await;
        assert!(resp.is_ok());

        let expected_text = r#"{"ispb":"00000000","name":"BCO DO BRASIL S.A.","code":1,"fullName":"Banco do Brasil S.A."}"#;
        let expected_json = serde_json::from_str::<BankResponseData>(expected_text).unwrap();

        let from_svc = resp.unwrap();

        assert_eq!(from_svc, expected_json);
    }

    #[async_test]
    async fn test_non_existing() {
        let resp = cli().get_banks_by_code(&999).await;
        assert!(resp.is_err());
    }

}