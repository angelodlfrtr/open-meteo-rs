#![deny(clippy::all, clippy::perf, clippy::complexity, clippy::pedantic)]

extern crate serde;
extern crate serde_json;

mod client;
mod errors;
mod location;

pub mod air_quality;
pub mod forecast;
pub mod geocoding;

pub use client::*;
pub use errors::*;
pub use location::*;
