//! Geocoding: search places by name, or fetch one by its id.

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{Error, Query, Request};

/// Parameters of a place search.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Options {
    /// Place name or postal code. A country or region can follow after a comma.
    pub name: Option<String>,
    /// Two letter language code for the returned names.
    pub language: Option<String>,
    /// Number of results, 1 to 100. The API defaults to 10.
    pub count: Option<u16>,
    /// ISO-3166-1 alpha-2 country code to restrict the search to.
    pub country_code: Option<String>,
    pub apikey: Option<String>,
}

impl Options {
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    #[must_use]
    pub fn with_language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    #[must_use]
    pub fn with_count(mut self, count: u16) -> Self {
        self.count = Some(count);
        self
    }

    #[must_use]
    pub fn with_country_code(mut self, country_code: String) -> Self {
        self.country_code = Some(country_code);
        self
    }
}

/// Parameters of a lookup by place id.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GetOptions {
    /// A place id as returned by a search.
    pub id: i64,
    pub language: Option<String>,
    pub apikey: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GeocodingResponse {
    pub results: Option<Vec<GeocodingResult>>,
    pub generationtime_ms: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GeocodingResult {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation: Option<f64>,
    pub feature_code: Option<String>,
    pub country_code: Option<String>,
    pub country_id: Option<i64>,
    pub country: Option<String>,
    pub admin1_id: Option<i64>,
    pub admin1: Option<String>,
    pub admin2_id: Option<i64>,
    pub admin2: Option<String>,
    pub admin3_id: Option<i64>,
    pub admin3: Option<String>,
    pub admin4_id: Option<i64>,
    pub admin4: Option<String>,
    pub timezone: Option<String>,
    pub population: Option<i64>,
    pub postcodes: Option<Vec<String>>,
}

impl Request for Options {
    type Response = GeocodingResponse;

    fn query(&self) -> Query {
        let mut query = Query::new();
        query
            .optional("name", self.name.as_deref())
            .optional("language", self.language.as_deref())
            .optional("count", self.count)
            .optional("countryCode", self.country_code.as_deref())
            .optional("apikey", self.apikey.as_deref());
        query
    }

    fn parse(body: &[u8]) -> Result<GeocodingResponse, Error> {
        Ok(serde_json::from_slice(body)?)
    }
}

impl Request for GetOptions {
    type Response = GeocodingResult;

    fn query(&self) -> Query {
        let mut query = Query::new();
        query
            .param("id", self.id)
            .optional("language", self.language.as_deref())
            .optional("apikey", self.apikey.as_deref());
        query
    }

    fn parse(body: &[u8]) -> Result<GeocodingResult, Error> {
        Ok(serde_json::from_slice(body)?)
    }
}
