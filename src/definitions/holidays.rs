use crate::{BrasilApiClientError, client};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/**
Data contract for Brasil holidays
 */
pub struct HolidaysResponseData {
    /// The holiday date
    pub date : String,
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


/// Trait for Holidays Operations
pub trait HolidaysOperations {
    /// Retrieve the Holidays for the given year
    fn get_holidays(&self, year: &i32) -> crate::BrResult<Vec<HolidaysResponseData>>;
}

const URL: &str = "feriados/v1";
const MIN_YEAR: &i32 = &1900;
const MAX_YEAR: &i32 = &2199;

impl HolidaysOperations for client::BrasilApiClient {
    fn get_holidays(&self, year: &i32) -> crate::BrResult<Vec<HolidaysResponseData>> {
        if year < MIN_YEAR || year > MAX_YEAR {
            Err(BrasilApiClientError::InvalidRangeError
                {
                    found: year.clone(), 
                    min: MIN_YEAR.clone(), 
                    max: MAX_YEAR.clone()
                })?
        }

        return self.get_helper::<Vec<HolidaysResponseData>>(format!("{}/{}",URL, year));
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::tests::cli, holidays::HolidaysResponseData};

    use super::{HolidaysOperations, MIN_YEAR, MAX_YEAR};

    #[test]
    fn test_invalid_min_range() {
        let year = MIN_YEAR - 1;
        let resp = cli().get_holidays(&year);
        assert!(resp.is_err());
    }

    #[test]
    fn test_invalid_max_range() {
        let year = MAX_YEAR + 1;
        let resp = cli().get_holidays(&year);
        assert!(resp.is_err());
    }

    #[test]
    fn test_valid_at_1900() {
        let year = MIN_YEAR;
        let resp = cli().get_holidays(&year);
        assert!(resp.is_ok());
        
        let expected_resp = r#"[{"date":"1900-01-01","name":"Confraternização mundial","type":"national"},{"date":"1900-02-27","name":"Carnaval","type":"national"},{"date":"1900-04-15","name":"Páscoa","type":"national"},{"date":"1900-04-21","name":"Tiradentes","type":"national"},{"date":"1900-05-01","name":"Dia do trabalho","type":"national"},{"date":"1900-06-14","name":"Corpus Christi","type":"national"},{"date":"1900-09-07","name":"Independência do Brasil","type":"national"},{"date":"1900-10-12","name":"Nossa Senhora Aparecida","type":"national"},{"date":"1900-11-02","name":"Finados","type":"national"},{"date":"1900-11-15","name":"Proclamação da República","type":"national"},{"date":"1900-12-25","name":"Natal","type":"national"}]"#;
        let expected =  serde_json::from_str::<Vec<HolidaysResponseData>>(expected_resp).unwrap();
        let received = resp.unwrap();
        assert_eq!(expected.len(), received.len());

        let difference: Vec<_> = received.into_iter().filter(|item| !expected.contains(item)).collect();
        assert_eq!(difference.len(), 0);
    }
}