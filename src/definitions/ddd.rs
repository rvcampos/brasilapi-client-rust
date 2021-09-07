use crate::{client::*, constants::ddd::*, errors::*, request::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// The DDD message struct
pub struct DDDResponseData {
    /// The country state (i.e: SP)
    pub state: String,
    /// The cities using this DDD
    pub cities: Vec<String>
}

impl PartialEq for DDDResponseData {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.cities == other.cities
    }
}

impl BrasilApiClient{
    pub async fn get_city_and_states_by_ddd(&self, ddd: &i8) -> Result<DDDResponseData, Error> {

        if ddd < MIN_DDD || ddd > MAX_DDD {
            return Err(Error::InvalidInputRangeError
                {
                    name: "ddd".to_string(),
                    min: MIN_DDD.clone() as i32, 
                    max: MAX_DDD.clone() as i32
                })?
        }

        Ok(get::<(), DDDResponseData>(
            &format!("{}/{}/{}", self.base_url, SVC_URL, ddd)
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
        let code = MIN_DDD - 1;
        let resp = cli().get_city_and_states_by_ddd(&code).await;
        assert!(resp.is_err());
    }

    #[async_test]
    async fn test_invalid_max_range() {
        let code = MAX_DDD + 1;
        let resp = cli().get_city_and_states_by_ddd(&code).await;
        assert!(resp.is_err());
    }


    #[async_test]
    async fn test_valid_ddd_97() {
        let resp = cli().get_city_and_states_by_ddd(&97).await;
        assert!(resp.is_ok());

        let expected_text = r#"{"state":"AM","cities":["UARINI","TONANTINS","TEFÉ","TAPAUÁ","TABATINGA","SÃO PAULO DE OLIVENÇA","SÃO GABRIEL DA CACHOEIRA","SANTO ANTÔNIO DO IÇÁ","SANTA ISABEL DO RIO NEGRO","PAUINI","NOVO ARIPUANÃ","MARAÃ","MANICORÉ","LÁBREA","JUTAÍ","JURUÁ","JAPURÁ","ITAMARATI","IPIXUNA","HUMAITÁ","GUAJARÁ","FONTE BOA","ENVIRA","EIRUNEPÉ","CODAJÁS","COARI","CARAUARI","CANUTAMA","BOCA DO ACRE","BERURI","BENJAMIN CONSTANT","BARCELOS","ATALAIA DO NORTE","APUÍ","ANORI","ANAMÃ","AMATURÁ","ALVARÃES"]}"#;
        let expected_json = serde_json::from_str::<DDDResponseData>(expected_text).unwrap();

        let from_svc = resp.unwrap();

        assert_eq!(from_svc, expected_json);
    }

    #[async_test]
    async fn test_non_existing() {
        let resp = cli().get_city_and_states_by_ddd(&26).await;
        assert!(resp.is_err());
    }

}