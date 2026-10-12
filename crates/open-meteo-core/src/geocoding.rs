//! Geocoding: search places by name, or fetch one by its id.

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{Error, Query, Request};

/// Parameters of a place search.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Options {
    /// Place name or postal code. A country or region can follow after a comma.
    /// Name of the place, in the requested language when available.
    pub name: Option<String>,
    /// Two letter language code for the returned names.
    pub language: Option<String>,
    /// Number of results, 1 to 100. The API defaults to 10.
    pub count: Option<u16>,
    /// ISO-3166-1 alpha-2 country code to restrict the search to.
    /// ISO-3166-1 alpha-2 country code.
    pub country_code: Option<String>,
    /// API key for the commercial API. Not needed for the free tier.
    pub apikey: Option<String>,
}

impl Options {
    /// Set the place name or postal code to search for.
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the language of the returned names.
    #[must_use]
    pub fn with_language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    /// Set the maximum number of results, 1 to 100.
    #[must_use]
    pub fn with_count(mut self, count: u16) -> Self {
        self.count = Some(count);
        self
    }

    /// Restrict the search to one country, by ISO-3166-1 alpha-2 code.
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
    /// Two letter language code for the returned names.
    pub language: Option<String>,
    /// API key for the commercial API. Not needed for the free tier.
    pub apikey: Option<String>,
}

/// The answer to a place search.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GeocodingResponse {
    /// Matching places, best match first. Absent when nothing matched.
    pub results: Option<Vec<GeocodingResult>>,
    /// Server-side processing time in milliseconds.
    pub generationtime_ms: Option<f64>,
}

/// One place, as returned by a search or a lookup by id.
///
/// Every field is optional because the database is sparse: smaller places lack population,
/// postcodes or some administrative levels.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GeocodingResult {
    /// Unique id of the place, usable with [`GetOptions`].
    pub id: Option<i64>,
    /// Name of the place, in the requested language when available.
    pub name: Option<String>,
    /// Latitude in decimal degrees.
    pub latitude: Option<f64>,
    /// Longitude in decimal degrees.
    pub longitude: Option<f64>,
    /// Elevation in metres.
    pub elevation: Option<f64>,
    /// `GeoNames` feature code, e.g. `PPLC` for a capital city.
    pub feature_code: Option<String>,
    /// ISO-3166-1 alpha-2 country code.
    pub country_code: Option<String>,
    /// Id in the `GeoNames` database of the country.
    pub country_id: Option<i64>,
    /// Country name.
    pub country: Option<String>,
    /// Id in the `GeoNames` database of the first administrative level (state, region).
    pub admin1_id: Option<i64>,
    /// Name of the first administrative level (state, region).
    pub admin1: Option<String>,
    /// Id in the `GeoNames` database of the second administrative level (county, department).
    pub admin2_id: Option<i64>,
    /// Name of the second administrative level (county, department).
    pub admin2: Option<String>,
    /// Id in the `GeoNames` database of the third administrative level.
    pub admin3_id: Option<i64>,
    /// Name of the third administrative level.
    pub admin3: Option<String>,
    /// Id in the `GeoNames` database of the fourth administrative level.
    pub admin4_id: Option<i64>,
    /// Name of the fourth administrative level.
    pub admin4: Option<String>,
    /// IANA time zone of the place.
    pub timezone: Option<String>,
    /// Number of inhabitants.
    pub population: Option<i64>,
    /// Postal codes of the place.
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
