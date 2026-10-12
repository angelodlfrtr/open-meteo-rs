//! A synchronous client, available with the `blocking` feature.
//!
//! It must not be used from inside an async runtime, or it will panic when it blocks.
//! Use [`crate::Client`] from async code instead.
//!
// Options are taken by value so the blocking and async clients read the same; they are
// small, built per call, and the async methods cannot take them by reference anyway
// without forcing a lifetime onto every future.
#![allow(clippy::needless_pass_by_value)]

use open_meteo_core::{
    air_quality, climate, elevation, ensemble, flood, forecast, geocoding, marine,
    satellite_radiation, seasonal, Endpoints, Error as ApiError, Request,
};

use crate::client::{DEFAULT_CONNECT_TIMEOUT, DEFAULT_TIMEOUT, DEFAULT_USER_AGENT};
use crate::Result;

/// Synchronous Open-Meteo client.
#[derive(Debug, Clone)]
pub struct Client {
    /// Base URL of every endpoint. Defaults to the public API.
    pub endpoints: Endpoints,
    /// The `reqwest` client requests go through.
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
    /// A client for the public API, with a 5 second timeout and this crate's user agent.
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

    /// Past forecasts as they were issued, stitched into one series. From 2022 onwards.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn historical_forecast(&self, opts: forecast::Options) -> Result<forecast::ForecastResult> {
        self.send(&self.endpoints.historical_forecast, &opts)
    }

    /// Earlier runs of a model, as `<variable>_previous_dayN` columns, to study forecast skill.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn previous_runs(&self, opts: forecast::Options) -> Result<forecast::ForecastResult> {
        self.send(&self.endpoints.previous_runs, &opts)
    }

    /// Marine weather: waves, swell and ocean currents.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn marine(&self, opts: marine::Options) -> Result<marine::MarineResult> {
        self.send(&self.endpoints.marine, &opts)
    }

    /// Every member of an ensemble model run.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn ensemble(&self, opts: ensemble::Options) -> Result<ensemble::EnsembleResult> {
        self.send(&self.endpoints.ensemble, &opts)
    }

    /// Climate projections from 1950 to 2050.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn climate(&self, opts: climate::Options) -> Result<climate::ClimateResult> {
        self.send(&self.endpoints.climate, &opts)
    }

    /// River discharge forecasts.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn flood(&self, opts: flood::Options) -> Result<flood::FloodResult> {
        self.send(&self.endpoints.flood, &opts)
    }

    /// Solar radiation measured by satellites.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn satellite_radiation(
        &self,
        opts: satellite_radiation::Options,
    ) -> Result<satellite_radiation::SatelliteRadiationResult> {
        self.send(&self.endpoints.satellite_radiation, &opts)
    }

    /// Seasonal forecasts, weeks to months ahead.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn seasonal(&self, opts: seasonal::Options) -> Result<seasonal::SeasonalResult> {
        self.send(&self.endpoints.seasonal, &opts)
    }

    /// Terrain elevation in metres, one value per requested location.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub fn elevation(&self, opts: elevation::Options) -> Result<Vec<f64>> {
        self.send(&self.endpoints.elevation, &opts)
    }
}
