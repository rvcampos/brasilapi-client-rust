#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#[macro_use] 
extern crate lazy_static;

/**
This module should be used to get the Helper Client
*/
pub mod client;

pub(crate) mod definitions;

pub use crate::definitions::*;
use serde::{Serialize, Deserialize};
use std::error::Error;
use thiserror::Error;


/// Default Result based onstd::result::Result<T, Box<dyn Error>> 
pub type BrResult<'a, T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
/// Based on https://brasilapi.com.br/doc using all the fields, so we can reuse the struct
pub struct ServiceErrorDetail {
    name: String,
    message: String,
    service: String
}

impl Default for ServiceErrorDetail {
    fn default() -> Self {
        ServiceErrorDetail { name: "".into(), message: "".into(), service: "".into() }
    }
}

impl ServiceError {
    fn new(err: &str) -> ServiceError {
        ServiceError {
            message: err.into(), 
            name: "Server Error".into(),
            r#type: "ServerError".into(),
            errors: Vec::new()
        }
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "There is an error: {}", self.message)
    }
}


impl Error for ServiceError {}


#[derive(Serialize, Deserialize, Debug)]
/// This is based on https://brasilapi.com.br/doc
pub struct ServiceError {
    /// The error message
    pub message: String,
    /// The error type
    pub r#type:  String,
    /// The error name
    pub name:  String,
    /// All detailed error info
    #[serde(default)]
    pub errors: Vec<ServiceErrorDetail>
}
#[derive(Debug, Error)]
/// BrasilApiClientError with all pushed errors
pub enum BrasilApiClientError {
    #[error("Field is filled but have invalid values")]
    /// E
    InvalidInputError(#[from] std::io::Error),
    #[error("invalid value [found:?], input should be between {min:?} and {max:?}")]
    /// Invalid Range error 
    InvalidRangeError{
        /// The actual value
        found: i32,
        /// The min range value
        min: i32,
        /// The max range value
        max: i32
    },
    #[error("Service just broke")]
    /// Service Error from BrasilApi
    ServiceError(#[from] ServiceError),
    #[error("One Unexpected error just happened")]
    /// Not Expected error
    UnexpectedError
}