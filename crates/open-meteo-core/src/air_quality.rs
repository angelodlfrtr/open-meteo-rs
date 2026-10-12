//! Air quality: pollutants, pollen and air quality indices.

use alloc::string::String;
use alloc::vec::Vec;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::response::RawResponse;
use crate::{CellSelection, Error, Location, Model, Query, Request, TimeStep};

/// Parameters of an air quality request.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    /// Coordinates of the place to query.
    pub location: Location,
    /// Variables to return at hourly resolution.
    pub hourly: Vec<String>,
    /// Variables to return for the current conditions.
    pub current: Vec<String>,
    /// `cams_europe`, `cams_global` or `auto`.
    pub domains: Option<String>,
    /// IANA time zone name, or `auto` to derive it from the coordinates. Times in the result are local to it. The API defaults to UTC.
    pub time_zone: Option<String>,
    /// Also return this many days before today.
    pub past_days: Option<u8>,
    /// Number of days to return, starting today.
    pub forecast_days: Option<u8>,
    /// Also return this many hours before now. Hourly data only.
    pub past_hours: Option<u32>,
    /// Number of hours to return, starting now. Hourly data only.
    pub forecast_hours: Option<u32>,
    /// First day of the requested range. Mutually exclusive with `past_days` and `forecast_days`.
    pub start_date: Option<NaiveDate>,
    /// Last day of the requested range, inclusive.
    pub end_date: Option<NaiveDate>,
    /// Weather models to query. The API picks the best match for the location when empty.
    pub models: Vec<Model>,
    /// How to choose the grid cell for the coordinates. The API defaults to `land` on most endpoints.
    pub cell_selection: Option<CellSelection>,
    /// API key for the commercial API. Not needed for the free tier.
    pub apikey: Option<String>,
}

/// Air quality data, one entry per requested resolution.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AirQualityResult {
    /// Current conditions, present when `current` variables were requested.
    pub current: Option<TimeStep>,
    /// Hourly values, present when `hourly` variables were requested.
    pub hourly: Option<Vec<TimeStep>>,
}

impl Request for Options {
    type Response = AirQualityResult;

    fn query(&self) -> Query {
        let mut query = Query::new();
        query
            .location(&self.location)
            .param("timeformat", "unixtime")
            .list("current", &self.current)
            .list("hourly", &self.hourly)
            .optional("domains", self.domains.as_deref())
            .optional("timezone", self.time_zone.as_deref())
            .optional("past_days", self.past_days)
            .optional("forecast_days", self.forecast_days)
            .optional("past_hours", self.past_hours)
            .optional("forecast_hours", self.forecast_hours)
            .optional("start_date", self.start_date)
            .optional("end_date", self.end_date)
            .list("models", &self.models)
            .optional("cell_selection", self.cell_selection)
            .optional("apikey", self.apikey.as_deref());
        query
    }

    fn parse(body: &[u8]) -> Result<AirQualityResult, Error> {
        let raw = RawResponse::from_json(body)?;
        Ok(AirQualityResult {
            current: raw.current()?,
            hourly: raw.hourly()?,
        })
    }
}
