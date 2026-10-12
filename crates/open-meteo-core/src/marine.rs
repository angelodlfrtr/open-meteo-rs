//! Marine weather: waves, swell and ocean currents.

use alloc::string::String;
use alloc::vec::Vec;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::response::RawResponse;
use crate::{
    CellSelection, DateStep, Error, LengthUnit, Location, Model, Query, Request, TimeStep,
};

/// Parameters of a marine weather request.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    /// Coordinates of the place to query.
    pub location: Location,
    /// Variables to return for the current conditions.
    pub current: Vec<String>,
    /// Variables to return at hourly resolution.
    pub hourly: Vec<String>,
    /// Variables to return as daily aggregations.
    pub daily: Vec<String>,
    /// Unit for wave heights. The API defaults to metric.
    pub length_unit: Option<LengthUnit>,
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
    /// The API defaults to `sea` for this endpoint.
    pub cell_selection: Option<CellSelection>,
    /// API key for the commercial API. Not needed for the free tier.
    pub apikey: Option<String>,
}

/// Marine weather data, one entry per requested resolution.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MarineResult {
    /// Current conditions, present when `current` variables were requested.
    pub current: Option<TimeStep>,
    /// Hourly values, present when `hourly` variables were requested.
    pub hourly: Option<Vec<TimeStep>>,
    /// Daily values, present when `daily` variables were requested.
    pub daily: Option<Vec<DateStep>>,
}

impl Request for Options {
    type Response = MarineResult;

    fn query(&self) -> Query {
        let mut query = Query::new();
        query
            .location(&self.location)
            .param("timeformat", "unixtime")
            .list("current", &self.current)
            .list("hourly", &self.hourly)
            .list("daily", &self.daily)
            .optional("length_unit", self.length_unit)
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

    fn parse(body: &[u8]) -> Result<MarineResult, Error> {
        let raw = RawResponse::from_json(body)?;
        Ok(MarineResult {
            current: raw.current()?,
            hourly: raw.hourly()?,
            daily: raw.daily()?,
        })
    }
}
