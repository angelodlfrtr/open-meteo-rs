//! A synchronous client, available with the `blocking` feature.
//!
//! The functionality in this module must not be executed within an async
//! runtime, or it will panic when attempting to block. Use [`crate::Client`]
//! from async code instead.

use std::error::Error;

use crate::{air_quality, client, errors, forecast, geocoding};

/// Synchronous client for the open-meteo.com API.
#[derive(Debug)]
pub struct Client {
    /// Forecast API URL
    pub forecast_endpoint: String,
    /// Archive API URL
    pub archive_endpoint: String,
    pub geocoding_endpoint: String,
    pub air_quality_endpoint: String,
    pub http_client: reqwest::blocking::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            forecast_endpoint: client::DEFAULT_FORECAST_ENDPOINT.to_string(),
            archive_endpoint: client::DEFAULT_ARCHIVE_ENDPOINT.to_string(),
            geocoding_endpoint: client::DEFAULT_GEOCODING_ENDPOINT.to_string(),
            air_quality_endpoint: client::DEFAULT_AIR_QUALITY_ENDPOINT.to_string(),
            http_client: reqwest::blocking::Client::builder()
                .timeout(client::DEFAULT_TIMEOUT)
                .connect_timeout(client::DEFAULT_CONNECT_TIMEOUT)
                .user_agent(client::DEFAULT_USER_AGENT)
                .build()
                .unwrap(),
        }
    }
}

impl Client {
    #[must_use]
    pub fn new() -> Client {
        Self::default()
    }

    #[must_use]
    pub fn with_endpoints(
        forecast_endpoint: &str,
        archive_endpoint: &str,
        geocoding_endpoint: &str,
        air_quality_endpoint: &str,
    ) -> Client {
        Self {
            forecast_endpoint: forecast_endpoint.to_string(),
            archive_endpoint: archive_endpoint.to_string(),
            geocoding_endpoint: geocoding_endpoint.to_string(),
            air_quality_endpoint: air_quality_endpoint.to_string(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_forecast_endpoint(mut self, endpoint: String) -> Client {
        self.forecast_endpoint = endpoint;
        self
    }

    #[must_use]
    pub fn with_geocoding_endpoint(mut self, endpoint: String) -> Client {
        self.geocoding_endpoint = endpoint;
        self
    }

    #[must_use]
    pub fn with_reqwest_client(mut self, client: reqwest::blocking::Client) -> Client {
        self.http_client = client;
        self
    }

    /// Request forecast data
    ///
    /// ### Errors
    ///
    /// Return an `Err` if api return an error or in case of network error.
    pub fn forecast(
        &self,
        opts: forecast::Options,
    ) -> Result<forecast::ForecastResult, Box<dyn Error>> {
        self.forecast_request(opts, &format!("{}forecast", self.forecast_endpoint))
    }

    /// Request data from the archive (historic weather data)
    ///
    /// ### Errors
    ///
    /// Return an `Err` if api return an error or in case of network error.
    pub fn archive(
        &self,
        opts: forecast::Options,
    ) -> Result<forecast::ForecastResult, Box<dyn Error>> {
        self.forecast_request(opts, &format!("{}archive", self.archive_endpoint))
    }

    fn forecast_request(
        &self,
        opts: forecast::Options,
        api_endpoint: &str,
    ) -> Result<forecast::ForecastResult, Box<dyn Error>> {
        let url = reqwest::Url::parse_with_params(api_endpoint, opts.into_params())?;
        let res = self.http_client.get(url).send()?;

        if res.status().is_success() {
            let api_res = res.json::<forecast::ApiForecastResponse>()?;
            return forecast::api_to_result(api_res);
        }

        Err(Box::new(errors::ClientError::InvalidResponseStatus {
            status_code: res.status().as_u16(),
            text: res.text().unwrap_or(String::new()),
        }))
    }

    /// Make a geocoding request.
    ///
    /// ### Errors
    ///
    /// Will return `Err` if api return an invaid response or in case of network error.
    pub fn geocoding(
        &self,
        opts: geocoding::Options,
    ) -> Result<geocoding::GeocodingResponse, Box<dyn Error>> {
        let url = reqwest::Url::parse_with_params(&self.geocoding_endpoint, opts.into_params())?;
        let res = self.http_client.get(url).send()?;

        if res.status().is_success() {
            let res = res.json()?;
            return Ok(res);
        }

        Err(Box::new(errors::ClientError::InvalidResponseStatus {
            status_code: res.status().as_u16(),
            text: res.text().unwrap_or(String::new()),
        }))
    }

    /// Request air quality data
    ///
    /// ### Errors
    ///
    /// Return an `Err` if api call fail or in case of network error.
    pub fn air_quality(
        &self,
        opts: air_quality::Options,
    ) -> Result<air_quality::AirQualityResult, Box<dyn Error>> {
        let url = reqwest::Url::parse_with_params(&self.air_quality_endpoint, opts.into_params())?;
        let res = self.http_client.get(url).send()?;

        if res.status().is_success() {
            let res = res.json::<air_quality::ApiAirQualityResponse>()?;
            return air_quality::api_to_result(res);
        }

        Err(Box::new(errors::ClientError::InvalidResponseStatus {
            status_code: res.status().as_u16(),
            text: res.text().unwrap_or(String::new()),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::location::Location;
    use chrono::Duration;

    #[test]
    fn has_correct_default() {
        let clt = Client::new();
        assert_eq!(clt.forecast_endpoint, client::DEFAULT_FORECAST_ENDPOINT);
    }

    #[test]
    fn get_forecast() {
        let clt = Client::new();
        let mut opts = forecast::Options {
            location: Location {
                lat: 52.52,
                lng: 13.41,
            },
            current: vec!["temperature_2m".into()],
            ..Default::default()
        };

        opts.hourly.push("temperature_2m".into());
        opts.daily.push("temperature_2m_max".into());
        opts.time_zone = Some(chrono_tz::Tz::Europe__Paris.to_string());
        opts.start_date = Some(chrono::Utc::now().date_naive());
        opts.end_date = Some((chrono::Utc::now() + Duration::days(2)).date_naive());

        let res = clt.forecast(opts).unwrap();
        assert!(res.current.is_some());
        assert!(res.hourly.is_some());
        assert!(res.daily.is_some());
    }

    #[test]
    fn geocoding_search() {
        let clt = Client::new();
        let opts = geocoding::Options::default().with_name("Paris".into());
        let res = clt.geocoding(opts).unwrap();
        assert!(!res.results.unwrap().is_empty());
    }

    #[test]
    fn get_air_quality() {
        let clt = Client::new();
        let opts = air_quality::Options {
            location: Location {
                lat: 52.52,
                lng: 13.41,
            },
            current: vec!["sulphur_dioxide".into()],
            hourly: vec!["ozone".into()],
            ..Default::default()
        };

        let res = clt.air_quality(opts).unwrap();
        assert!(res.current.is_some());
        assert!(res.hourly.is_some());
    }
}
