//! River discharge forecasts from the `GloFAS` models.

use alloc::string::String;
use alloc::vec::Vec;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::response::RawResponse;
use crate::{CellSelection, DateStep, Error, Location, Model, Query, Request};

/// Parameters of a flood request.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    /// Coordinates of the place to query.
    pub location: Location,
    /// Variables to return as daily values, e.g. `river_discharge`.
    pub daily: Vec<String>,
    /// IANA time zone name, or `auto` to derive it from the coordinates. Times in the result are local to it. The API defaults to UTC.
    pub time_zone: Option<String>,
    /// Also return this many days before today.
    pub past_days: Option<u8>,
    /// 0 to 210. The API defaults to 92.
    pub forecast_days: Option<u8>,
    /// First day of the requested range. Mutually exclusive with `past_days` and `forecast_days`.
    pub start_date: Option<NaiveDate>,
    /// Last day of the requested range, inclusive.
    pub end_date: Option<NaiveDate>,
    /// Also return every ensemble member as `<variable>_memberNN`.
    pub ensemble: bool,
    /// Weather models to query. The API picks the best match for the location when empty.
    pub models: Vec<Model>,
    /// The API defaults to `nearest` for this endpoint.
    pub cell_selection: Option<CellSelection>,
    /// API key for the commercial API. Not needed for the free tier.
    pub apikey: Option<String>,
}

/// River discharge data.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FloodResult {
    /// Daily values, present when `daily` variables were requested.
    pub daily: Option<Vec<DateStep>>,
}

impl Request for Options {
    type Response = FloodResult;

    fn query(&self) -> Query {
        let mut query = Query::new();
        query
            .location(&self.location)
            .param("timeformat", "unixtime")
            .list("daily", &self.daily)
            .optional("timezone", self.time_zone.as_deref())
            .optional("past_days", self.past_days)
            .optional("forecast_days", self.forecast_days)
            .optional("start_date", self.start_date)
            .optional("end_date", self.end_date)
            .flag("ensemble", self.ensemble)
            .list("models", &self.models)
            .optional("cell_selection", self.cell_selection)
            .optional("apikey", self.apikey.as_deref());
        query
    }

    fn parse(body: &[u8]) -> Result<FloodResult, Error> {
        let raw = RawResponse::from_json(body)?;
        Ok(FloodResult {
            daily: raw.daily()?,
        })
    }
}
