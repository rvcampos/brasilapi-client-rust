use crate::{client::*, constants::cep::{SVC_V1_URL, SVC_V2_URL}, errors::*, request::*};
use serde::{Deserialize, Serialize};

/**
The Desired CEP Search Version
*/
pub enum EnumCepRequestVersion {
    /// V1 for common data, without GeoLocalization
    V1,
    /// V2 for common data + GeoLocalization
    V2
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// The coordinates in Latitude & Longitude for this address
pub struct Coordinates {
    /// The Latitude
    pub latitude: String,
    /// The Longitude
    pub longitude: String
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.latitude == other.latitude && self.longitude == other.longitude
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// GeoLocation info
pub struct Location
{
    /// The GeoCoordinates
    pub coordinates: Coordinates
}

impl Default for Location {
    fn default() -> Self {
        Location { coordinates: Coordinates { latitude: String::new(), longitude: String::new() } }
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// The Zipcode data struct
pub struct CepResponseData {
    /// The zipcode itself
    pub cep: String,
    /// The State name
    pub state: String,
    /// The City name
    pub city: String,
    /// The Neighborhood name
    pub neighborhood: String,
    /// The Street name
    pub street: String,
    /// Which service returned this
    pub service: String,
    /// The Geolocation data, only filled on V2
    #[serde(default)]
    pub location: Location
}

impl PartialEq for CepResponseData {
    fn eq(&self, other: &Self) -> bool {
        self.cep == other.cep 
        && self.state == other.state 
        && self.city == other.city 
        && self.neighborhood == other.neighborhood 
        && self.street == other.street
        && self.location == other.location
    }
}

impl BrasilApiClient {
    pub async fn get_cep(&self, cep: &str, cep_version: Option<EnumCepRequestVersion>) -> Result<CepResponseData, Error> {
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"[^0-9]").unwrap();
        }
        let cepver = cep_version.unwrap_or(EnumCepRequestVersion::V1);
        let url = match cepver {
            EnumCepRequestVersion::V2 => SVC_V2_URL,
            _ => SVC_V1_URL     
        };
        
        let temp_zipcode = RE.replace_all(cep, "");
        if temp_zipcode.is_empty() || temp_zipcode.len() > 8 {
            return Err(Error::InvalidInputLenError
                {
                    name: "cep".to_string(),
                    min: 8, 
                    max: 8
                })
        }
        
        Ok(get::<(), CepResponseData>(
            &format!("{}/{}/{}", self.base_url, url, temp_zipcode)
        ).await?)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::tests::*;
    use futures_await_test::async_test;

    #[async_test]
    async fn testc_invalid_input_minlen_none_ver() {
        let resp = cli().get_cep("09777", None)
        .await;

        assert!(resp.is_err());
    }

    #[async_test]
    async fn test_invalid_input_empty_none_ver() {
        let resp = cli().get_cep("", None)
        .await;

        assert!(resp.is_err());
    }

    #[async_test]
    async fn test_valid_none_ver() {
        let resp = cli().get_cep("01402-000", None).await;
        assert!(resp.is_ok());

        let expected_text = r#"{"cep":"01402000","state":"SP","city":"São Paulo","neighborhood":"Jardim Paulista","street":"Avenida Brigadeiro Luís Antônio","service":"viacep"}"#;
        let mut expected_json = serde_json::from_str::<CepResponseData>(expected_text).unwrap();
        expected_json.service = "".into();

        let mut from_svc = resp.unwrap();
        from_svc.service = "".into();

        assert_eq!(from_svc, expected_json);
    }

    #[async_test]
    async fn test_valid_v1_same_as_none() {
        let resp_v1 = cli().get_cep("01402-000", Some(EnumCepRequestVersion::V1)).await;
        let resp_none = cli().get_cep("01402-000", None).await;
        assert!(resp_v1.is_ok());
        assert!(resp_none.is_ok());

        let expected_text = r#"{"cep":"01402000","state":"SP","city":"São Paulo","neighborhood":"Jardim Paulista","street":"Avenida Brigadeiro Luís Antônio","service":"viacep"}"#;
        let mut expected_json = serde_json::from_str::<CepResponseData>(expected_text).unwrap();
        expected_json.service = "".into();

        let mut from_svc = resp_v1.unwrap();
        from_svc.service = "".into();

        assert_eq!(from_svc, expected_json);

        let mut from_svc_none = resp_none.unwrap();
        from_svc_none.service = "".into();

        assert_eq!(from_svc, from_svc_none);
    }

    #[async_test]
    async fn test_valid_v2() {
        let resp = cli().get_cep("01402-000", Some(EnumCepRequestVersion::V2)).await;
        assert!(resp.is_ok());

        let expected_text = r#"{"cep":"01402000","state":"SP","city":"São Paulo","neighborhood":"Jardim Paulista","street":"Avenida Brigadeiro Luís Antônio","service":"viacep","location":{"type":"Point","coordinates":{"longitude":"-46.6367822","latitude":"-23.5507017"}}}"#;
        let mut expected_json = serde_json::from_str::<CepResponseData>(expected_text).unwrap();
        expected_json.service = "".into();

        let mut from_svc = resp.unwrap();
        from_svc.service = "".into();

        assert_eq!(from_svc, expected_json);
    }

    #[async_test]
    async fn test_valid_v2_not_equals_v1() {
        let resp_v2 = cli().get_cep("01402-000", Some(EnumCepRequestVersion::V2)).await;
        let resp_none = cli().get_cep("01402-000", None).await;
        assert!(resp_v2.is_ok());
        assert!(resp_none.is_ok());

        let expected_text = r#"{"cep":"01402000","state":"SP","city":"São Paulo","neighborhood":"Jardim Paulista","street":"Avenida Brigadeiro Luís Antônio","service":"viacep","location":{"type":"Point","coordinates":{"longitude":"-46.6367822","latitude":"-23.5507017"}}}"#;
        let mut expected_json = serde_json::from_str::<CepResponseData>(expected_text).unwrap();
        expected_json.service = "".into();

        let mut from_svc = resp_v2.unwrap();
        from_svc.service = "".into();

        assert_eq!(from_svc, expected_json);

        let mut from_svc_none = resp_none.unwrap();
        from_svc_none.service = "".into();

        assert_ne!(from_svc, from_svc_none);
    }
}