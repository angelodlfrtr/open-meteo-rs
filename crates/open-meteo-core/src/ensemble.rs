//! Ensemble forecasts: every member of a probabilistic model run.
//!
//! Members come back as extra variables, `temperature_2m_member01` and so on, next to the
//! control run `temperature_2m`. Ensemble mean models (`*_ensemble_mean_*`) use the same
//! endpoint and return `_mean`/`_spread` suffixed variables instead.

use alloc::string::String;
use alloc::vec::Vec;

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::forecast::hour;
use crate::response::RawResponse;
use crate::{
    CellSelection, Elevation, Error, Location, Model, PrecipitationUnit, Query, Request,
    TemperatureUnit, TimeStep, WindSpeedUnit,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    pub location: Location,
    pub elevation: Option<Elevation>,
    /// Variables to return at hourly resolution.
    pub hourly: Vec<String>,
    /// Ensemble models to query. The API requires at least one.
    pub models: Vec<Model>,
    pub temperature_unit: Option<TemperatureUnit>,
    pub wind_speed_unit: Option<WindSpeedUnit>,
    pub precipitation_unit: Option<PrecipitationUnit>,
    pub time_zone: Option<String>,
    pub past_days: Option<u8>,
    /// 0 to 35.
    pub forecast_days: Option<u8>,
    pub past_hours: Option<u32>,
    pub forecast_hours: Option<u32>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub start_hour: Option<NaiveDateTime>,
    pub end_hour: Option<NaiveDateTime>,
    pub cell_selection: Option<CellSelection>,
    pub apikey: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EnsembleResult {
    pub hourly: Option<Vec<TimeStep>>,
}

impl Request for Options {
    type Response = EnsembleResult;

    fn query(&self) -> Query {
        let mut query = Query::new();
        query
            .location(&self.location)
            .param("timeformat", "unixtime")
            .optional("elevation", self.elevation.as_ref())
            .list("hourly", &self.hourly)
            .list("models", &self.models)
            .optional("temperature_unit", self.temperature_unit)
            .optional("wind_speed_unit", self.wind_speed_unit)
            .optional("precipitation_unit", self.precipitation_unit)
            .optional("timezone", self.time_zone.as_deref())
            .optional("past_days", self.past_days)
            .optional("forecast_days", self.forecast_days)
            .optional("past_hours", self.past_hours)
            .optional("forecast_hours", self.forecast_hours)
            .optional("start_date", self.start_date)
            .optional("end_date", self.end_date)
            .optional("start_hour", self.start_hour.map(hour))
            .optional("end_hour", self.end_hour.map(hour))
            .optional("cell_selection", self.cell_selection)
            .optional("apikey", self.apikey.as_deref());
        query
    }

    fn parse(body: &[u8]) -> Result<EnsembleResult, Error> {
        let raw = RawResponse::from_json(body)?;
        Ok(EnsembleResult {
            hourly: raw.hourly()?,
        })
    }
}
