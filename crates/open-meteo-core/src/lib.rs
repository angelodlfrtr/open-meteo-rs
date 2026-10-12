//! Request building and response parsing for the [Open-Meteo](https://open-meteo.com) API.
//!
//! This crate does no I/O. It knows how to turn a set of options into a URL and how to turn
//! the JSON that Open-Meteo answers with into typed results, and nothing else. That makes it
//! `no_std` compatible (it needs `alloc`) and usable with any HTTP transport.
//!
//! The ready-made `reqwest` based client lives in the `open-meteo-rs` crate. Writing your own
//! transport takes a few lines: pick a base URL from [`Endpoints`], build the URL with [`url`],
//! perform a GET, then hand the body to [`Request::parse`].
//!
//! ```
//! use open_meteo_core::{forecast, url, Endpoints, Location, Request};
//!
//! let endpoints = Endpoints::default();
//! let options = forecast::Options {
//!     location: Location { lat: 52.52, lng: 13.41 },
//!     hourly: vec!["temperature_2m".into()],
//!     ..Default::default()
//! };
//!
//! let request_url = url(&endpoints.forecast, &options);
//! assert!(request_url.starts_with("https://api.open-meteo.com/v1/forecast?"));
//!
//! // let body = my_http_get(&request_url)?;
//! // let result = forecast::Options::parse(&body)?;
//! ```

#![no_std]
#![deny(clippy::all, clippy::perf, clippy::complexity, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

extern crate alloc;

mod endpoints;
mod error;
mod location;
mod model;
mod query;
mod request;
mod response;
mod units;

pub mod air_quality;
pub mod climate;
pub mod elevation;
pub mod ensemble;
pub mod flood;
pub mod forecast;
pub mod geocoding;
pub mod marine;
pub mod satellite_radiation;
pub mod seasonal;

pub use endpoints::Endpoints;
pub use error::{ConversionError, Error};
pub use location::Location;
pub use model::Model;
pub use query::Query;
pub use request::{url, Request};
pub use response::{DateStep, Measurement, TimeStep};
pub use units::{
    CellSelection, Elevation, LengthUnit, PrecipitationUnit, TemperatureUnit, TemporalResolution,
    WindSpeedUnit,
};
