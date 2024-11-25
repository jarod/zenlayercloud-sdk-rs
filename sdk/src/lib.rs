pub mod client;
pub mod credentials;
pub mod errors;
pub mod signer;

use std::env;

pub use client::{Client, Request, Response};
pub use errors::Error;

pub static API_DOMAIN: &str = "console.zenlayer.com";
pub static SDK_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Config {}
