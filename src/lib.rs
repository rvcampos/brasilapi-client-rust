//! # BrasilAPI rust client wrapper
//!
//! ## Why I created this?
//! * Well, I plan to use this API at the company I do work
//! * I'm very exhausted to use any language we have on current stack (c#, python, javascript), for real...
//! * A friend of mine talk about Rust every single day
//!
//! ## This code resembles X
//! Yes, probably, event reading some of the official documentation, I checked some cargo SDKs and wrappers, because I plan to use it soon 
//! Sorry, I cheated :(
//!
//! ```toml
//! [dependencies]
//! brasilapi-client = "0.1.0"
//! ```
//!
//! fn main() { block_on(async move {
//!     // As this API is public, you can use the following default builder
//!     let cli = BrasilApiClient::new_default(); // This method will use the default URL
//!
//!     // Get the index called "books"
//!     let zipcode_answer = client.get_cep("01402-000").unwrap();
//!
//!     println!("Street: {}", zipcode_answer.street);
//! })}
#![doc = include_str!("../README.md")]
#[macro_use] 
extern crate lazy_static;

/**
This module should be used to get the Helper Client
*/
pub mod client;
pub mod definitions;
pub mod constants;
pub mod errors;
mod request;

pub use crate::definitions::*;

#[cfg(feature = "sync")]
pub(crate) type Rc<T> = std::sync::Arc<T>;
#[cfg(not(feature = "sync"))]
pub(crate) type Rc<T> = std::rc::Rc<T>;
