//! Air quality: pollutants, pollen and air quality indices.

use alloc::string::String;
use alloc::vec::Vec;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::response::RawResponse;
use crate::{CellSelection, Error, Location, Model, Query, Request, TimeStep};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    pub location: Location,
    /// Variables to return at hourly resolution.
    pub hourly: Vec<String>,
    /// Variables to return for the current conditions.
    pub current: Vec<String>,
    /// `cams_europe`, `cams_global` or `auto`.
    pub domains: Option<String>,
    pub time_zone: Option<String>,
    pub past_days: Option<u8>,
    pub forecast_days: Option<u8>,
    pub past_hours: Option<u32>,
    pub forecast_hours: Option<u32>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub models: Vec<Model>,
    pub cell_selection: Option<CellSelection>,
    pub apikey: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AirQualityResult {
    pub current: Option<TimeStep>,
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
