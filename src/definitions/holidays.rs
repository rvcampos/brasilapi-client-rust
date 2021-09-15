use crate::{client::*, constants::holidays::{SVC_URL,MIN_YEAR, MAX_YEAR}, errors::*, request::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/**
Data contract for Brasil holidays
 */
pub struct HolidaysResponseData {
    /// The holiday date
    pub date : chrono::NaiveDate,
    /// The holiday name
    pub name: String,
    /// The holiday type
    pub r#type : String
}

impl PartialEq for HolidaysResponseData {

    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.name == other.name && self.r#type == other.r#type
    }
}

impl BrasilApiClient {
    pub async fn get_holidays(&self, year: &i32) -> Result<Vec<HolidaysResponseData>, Error> {
        if year < MIN_YEAR || year > MAX_YEAR {
            return Err(Error::InvalidInputRangeError
                {
                    name: "year".to_string(),
                    min: *MIN_YEAR, 
                    max: *MAX_YEAR
                })
        }

        
        Ok(get::<(), Vec<HolidaysResponseData>>(
            &format!("{}/{}/{}", self.base_url, SVC_URL, year)
        ).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{holidays::{HolidaysResponseData}, client::tests::cli};

    use super::*;
    use futures_await_test::async_test;

    #[async_test]
    async fn test_invalid_min_range() {
        let year = MIN_YEAR - 1;
        let resp = cli().get_holidays(&year).await;
        assert!(resp.is_err());
    }

    #[async_test]
    async fn test_invalid_max_range() {
        let year = MAX_YEAR + 1;
        let resp = cli().get_holidays(&year).await;
        assert!(resp.is_err());
    }

    #[async_test]
    async fn test_valid_at_1900() {
        let year = MIN_YEAR;
        let resp = cli().get_holidays(year).await;
        assert!(resp.is_ok());
        
        let expected_resp = r#"[{"date":"1900-01-01","name":"Confraternização mundial","type":"national"},{"date":"1900-02-27","name":"Carnaval","type":"national"},{"date":"1900-04-15","name":"Páscoa","type":"national"},{"date":"1900-04-21","name":"Tiradentes","type":"national"},{"date":"1900-05-01","name":"Dia do trabalho","type":"national"},{"date":"1900-06-14","name":"Corpus Christi","type":"national"},{"date":"1900-09-07","name":"Independência do Brasil","type":"national"},{"date":"1900-10-12","name":"Nossa Senhora Aparecida","type":"national"},{"date":"1900-11-02","name":"Finados","type":"national"},{"date":"1900-11-15","name":"Proclamação da República","type":"national"},{"date":"1900-12-25","name":"Natal","type":"national"}]"#;
        let expected =  serde_json::from_str::<Vec<HolidaysResponseData>>(expected_resp).unwrap();
        let received = resp.unwrap();
        assert_eq!(expected.len(), received.len());

        let difference:bool = received.iter().any(|item| !expected.contains(item));
        assert!(!difference);
    }
}