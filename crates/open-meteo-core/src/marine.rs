//! Marine weather: waves, swell and ocean currents.

use alloc::string::String;
use alloc::vec::Vec;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::response::RawResponse;
use crate::{
    CellSelection, DateStep, Error, LengthUnit, Location, Model, Query, Request, TimeStep,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    pub location: Location,
    /// Variables to return for the current conditions.
    pub current: Vec<String>,
    /// Variables to return at hourly resolution.
    pub hourly: Vec<String>,
    /// Variables to return as daily aggregations.
    pub daily: Vec<String>,
    /// Unit for wave heights. The API defaults to metric.
    pub length_unit: Option<LengthUnit>,
    pub time_zone: Option<String>,
    pub past_days: Option<u8>,
    pub forecast_days: Option<u8>,
    pub past_hours: Option<u32>,
    pub forecast_hours: Option<u32>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub models: Vec<Model>,
    /// The API defaults to `sea` for this endpoint.
    pub cell_selection: Option<CellSelection>,
    pub apikey: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MarineResult {
    pub current: Option<TimeStep>,
    pub hourly: Option<Vec<TimeStep>>,
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
