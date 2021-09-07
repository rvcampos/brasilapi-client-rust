use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CepDetailedError {
    name: String,
    message: String,
    service: String,

}

// impl CepDetailedError {
//     fn new(name: &str, message: &str, service: &str) -> CepDetailedError {
//         CepDetailedError {
//             message: message.into(), 
//             name: name.into(),
//             service: service.into()
//         }
//     }
// }

/// An enum representing the errors that can occur.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    BrasilApiError {
        message: String,
        name: String,
        r#type: String,
    },
    BrasilCepApiError {
        message: String,
        name: String,
        r#type: String,
        errors: Vec<CepDetailedError>
    },
    NotExpectedRequestError,
    HttpError(isahc::Error),
    SerdeJsonError(serde_json::Error),
    InvalidInputLenError {
        name: String,
        min: i32,
        max: i32
    },
    InvalidInputRangeError {
        name: String,
        min: i32,
        max: i32
    }
}

impl std::fmt::Display for CepDetailedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}
            Message: {}
            Service:{}",
            self.name, self.message, self.service
        )
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::BrasilApiError {
                message,
                name,
                r#type
            } => write!(
                fmt,
                "Error during BrasilApiCall [
                    message: {}
                    name: {}
                    type: {}]",
                message,
                name,
                r#type,
            ),
            Error::BrasilCepApiError {
                message,
                name,
                r#type,
                errors
            } => write!(
                fmt,
                "Error during BrasilApiCall [CEP] [
                    message: {}
                    name: {}
                    type: {}
                    errors: {:#?}]",
                message,
                name,
                r#type,
                errors
            ),
            Error::HttpError(e) => write!(fmt, "HTTP request failed: {}", e),
            Error::NotExpectedRequestError => write!(fmt, "Not Expected Error"),
            Error::SerdeJsonError(_) => todo!(),
            Error::InvalidInputLenError { name, min, max } => write!(fmt,
            "Field [{}] expected length should be between {} and {}",
            name, 
            min, 
            max),
            Error::InvalidInputRangeError { name, min, max } => write!(fmt,
                "Field [{}] value range should be between {} and {}",
                name, 
                min, 
                max),
        }
    }
}

impl std::error::Error for Error {}

impl From<&serde_json::Value> for Error {
    fn from(json: &serde_json::Value) -> Error {

        let message = json
            .get("message")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| json.to_string());

        let name = json
            .get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(String::new);

        let ztype = json
        .get("type")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(String::new);

        if !json["errors"].is_array() {
            return Error::BrasilApiError {
                message,
                name,
                r#type: ztype,
            };
        }

        let errors = Vec::<CepDetailedError>::deserialize(&json["errors"]).unwrap();

        Error::BrasilCepApiError {
            message,
            name,
            r#type: ztype,
            errors
        }
    }
}

impl From<isahc::Error> for Error {
    fn from(error: isahc::Error) -> Error {
        if error.kind() == isahc::error::ErrorKind::ConnectionFailed {
            Error::NotExpectedRequestError
        } else {
            Error::HttpError(error)
        }
    }
}