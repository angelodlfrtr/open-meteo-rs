//! A client for the [Open-Meteo](https://open-meteo.com) weather API, built on `reqwest`.
//!
//! All request and response types come from the `open-meteo-core` crate and are re-exported
//! here, so `open_meteo_rs::forecast::Options` and friends are the only paths you need.
//!
//! ```no_run
//! # async fn run() -> Result<(), open_meteo_rs::Error> {
//! let client = open_meteo_rs::Client::new();
//! let options = open_meteo_rs::forecast::Options {
//!     hourly: vec!["temperature_2m".into()],
//!     ..Default::default()
//! };
//! let forecast = client.forecast(options).await?;
//! println!("{:#?}", forecast.hourly);
//! # Ok(())
//! # }
//! ```
//!
//! Enable the `blocking` feature for a synchronous [`blocking::Client`] with the same methods.

#![deny(clippy::all, clippy::perf, clippy::complexity, clippy::pedantic)]

#[cfg(feature = "blocking")]
pub mod blocking;
mod client;
mod error;

pub use client::Client;
pub use error::{Error, Result};
pub use open_meteo_core::*;
