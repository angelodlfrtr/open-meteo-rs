#![deny(clippy::all, clippy::perf, clippy::complexity, clippy::pedantic)]

mod client;
mod errors;
mod location;

pub mod air_quality;
pub mod forecast;
pub mod geocoding;

pub use client::*;
pub use errors::*;
pub use location::*;
