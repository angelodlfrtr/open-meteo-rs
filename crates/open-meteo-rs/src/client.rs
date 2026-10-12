use std::time::Duration;

use open_meteo_core::{air_quality, forecast, geocoding, Endpoints, Error as ApiError, Request};

use crate::Result;

pub(crate) const DEFAULT_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
pub(crate) const DEFAULT_TIMEOUT: Duration = Duration::from_millis(5000);
pub(crate) const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_millis(2000);

/// Asynchronous Open-Meteo client.
///
/// Cheap to clone. Build one with [`Client::new`] and share it; the underlying `reqwest`
/// client pools connections.
#[derive(Debug, Clone)]
pub struct Client {
    pub endpoints: Endpoints,
    pub http_client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            endpoints: Endpoints::default(),
            http_client: reqwest::Client::builder()
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
    pub fn with_reqwest_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = client;
        self
    }

    /// Send any [`Request`] to the endpoint at `base` and decode the answer.
    ///
    /// The endpoint methods below are thin wrappers around this. Use it directly to reach an
    /// endpoint this crate has no method for yet.
    ///
    /// # Errors
    ///
    /// [`Error::Http`](crate::Error::Http) when the request fails to complete, and
    /// [`Error::Api`](crate::Error::Api) on a non-success status or an undecodable body.
    pub async fn send<R: Request>(&self, base: &str, request: &R) -> Result<R::Response> {
        let response = self
            .http_client
            .get(open_meteo_core::url(base, request))
            .send()
            .await?;
        let status = response.status();
        let body = response.bytes().await?;

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
    pub async fn forecast(&self, opts: forecast::Options) -> Result<forecast::ForecastResult> {
        self.send(&self.endpoints.forecast, &opts).await
    }

    /// Historical weather from reanalysis models, back to 1940.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub async fn archive(&self, opts: forecast::Options) -> Result<forecast::ForecastResult> {
        self.send(&self.endpoints.archive, &opts).await
    }

    /// Air quality: pollutants, pollen and indices.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub async fn air_quality(
        &self,
        opts: air_quality::Options,
    ) -> Result<air_quality::AirQualityResult> {
        self.send(&self.endpoints.air_quality, &opts).await
    }

    /// Search places by name.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub async fn geocoding(
        &self,
        opts: geocoding::Options,
    ) -> Result<geocoding::GeocodingResponse> {
        self.send(&self.endpoints.geocoding_search, &opts).await
    }

    /// Fetch one place by the id a search returned.
    ///
    /// # Errors
    ///
    /// See [`Client::send`].
    pub async fn geocoding_get(
        &self,
        opts: geocoding::GetOptions,
    ) -> Result<geocoding::GeocodingResult> {
        self.send(&self.endpoints.geocoding_get, &opts).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_public_endpoints_by_default() {
        let client = Client::new();
        assert_eq!(client.endpoints, Endpoints::default());
    }

    #[test]
    fn endpoints_can_be_overridden() {
        let endpoints = Endpoints {
            forecast: "http://localhost:8080/v1/forecast".into(),
            ..Default::default()
        };
        let client = Client::new().with_endpoints(endpoints.clone());
        assert_eq!(client.endpoints, endpoints);
    }
}
