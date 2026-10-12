//! A synchronous client, available with the `blocking` feature.
//!
//! It must not be used from inside an async runtime, or it will panic when it blocks.
//! Use [`crate::Client`] from async code instead.
//!
// Options are taken by value so the blocking and async clients read the same; they are
// small, built per call, and the async methods cannot take them by reference anyway
// without forcing a lifetime onto every future.
#![allow(clippy::needless_pass_by_value)]

use open_meteo_core::{air_quality, forecast, geocoding, Endpoints, Error as ApiError, Request};

use crate::client::{DEFAULT_CONNECT_TIMEOUT, DEFAULT_TIMEOUT, DEFAULT_USER_AGENT};
use crate::Result;

/// Synchronous Open-Meteo client.
#[derive(Debug, Clone)]
pub struct Client {
    pub endpoints: Endpoints,
    pub http_client: reqwest::blocking::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            endpoints: Endpoints::default(),
            http_client: reqwest::blocking::Client::builder()
                .timeout(DEFAULT_TIMEOUT)
                .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
                .user_agent(DEFAULT_USER_AGENT)
                .build()
                .expect("the default reqwest client configuration is valid"),
        }
    }
}

impl Client {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Use other base URLs, e.g. for the commercial API or a self-hosted instance.
    #[must_use]
    pub fn with_endpoints(mut self, endpoints: Endpoints) -> Self {
        self.endpoints = endpoints;
        self
    }

    /// Use a pre-configured `reqwest` client (proxy, custom timeouts, ...).
    #[must_use]
    pub fn with_reqwest_client(mut self, client: reqwest::blocking::Client) -> Self {
        self.http_client = client;
        self
    }

    /// Send any [`Request`] to the endpoint at `base` and decode the answer.
    ///
    /// # Errors
    ///
    /// [`Error::Http`](crate::Error::Http) when the request fails to complete, and
    /// [`Error::Api`](crate::Error::Api) on a non-success status or an undecodable body.
    pub fn send<R: Request>(&self, base: &str, request: &R) -> Result<R::Response> {
        let response = self
            .http_client
            .get(open_meteo_core::url(base, request))
            .send()?;
        let status = response.status();
        let body = response.bytes()?;

        if !status.is_success() {
            return Err(ApiError::from_status(status.as_u16(), &body).into());
        }

        Ok(R::parse(&body)?)
    }

    /// Weather forecast, up to 16 days ahead.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn forecast(&self, opts: forecast::Options) -> Result<forecast::ForecastResult> {
        self.send(&self.endpoints.forecast, &opts)
    }

    /// Historical weather from reanalysis models, back to 1940.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn archive(&self, opts: forecast::Options) -> Result<forecast::ForecastResult> {
        self.send(&self.endpoints.archive, &opts)
    }

    /// Air quality: pollutants, pollen and indices.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn air_quality(&self, opts: air_quality::Options) -> Result<air_quality::AirQualityResult> {
        self.send(&self.endpoints.air_quality, &opts)
    }

    /// Search places by name.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn geocoding(&self, opts: geocoding::Options) -> Result<geocoding::GeocodingResponse> {
        self.send(&self.endpoints.geocoding_search, &opts)
    }

    /// Fetch one place by the id a search returned.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn geocoding_get(&self, opts: geocoding::GetOptions) -> Result<geocoding::GeocodingResult> {
        self.send(&self.endpoints.geocoding_get, &opts)
    }
}
