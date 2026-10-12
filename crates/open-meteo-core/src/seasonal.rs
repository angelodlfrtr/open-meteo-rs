//! Seasonal forecasts: weeks to months ahead, from ensemble models.
//!
//! Members and statistics come back as suffixed variables (`temperature_2m_max_member01`,
//! `_mean`, `_spread`, `_anomaly`), like the ensemble endpoint.

use alloc::string::String;
use alloc::vec::Vec;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::response::RawResponse;
use crate::{
    CellSelection, DateStep, Error, Location, Model, PrecipitationUnit, Query, Request,
    TemperatureUnit, TimeStep, WindSpeedUnit,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    pub location: Location,
    /// Variables to return every 6 hours.
    pub six_hourly: Vec<String>,
    /// Variables to return as daily aggregations.
    pub daily: Vec<String>,
    /// Variables to return as weekly aggregations.
    pub weekly: Vec<String>,
    /// Variables to return as monthly aggregations.
    pub monthly: Vec<String>,
    /// Seasonal models, e.g. [`Model::EcmwfSeas5`] or [`Model::EcmwfEc46`].
    pub models: Vec<Model>,
    pub temperature_unit: Option<TemperatureUnit>,
    pub wind_speed_unit: Option<WindSpeedUnit>,
    pub precipitation_unit: Option<PrecipitationUnit>,
    pub time_zone: Option<String>,
    pub past_days: Option<u8>,
    /// Up to about 7 months. The API defaults to 6 months.
    pub forecast_days: Option<u16>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub cell_selection: Option<CellSelection>,
    pub apikey: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SeasonalResult {
    pub six_hourly: Option<Vec<TimeStep>>,
    pub daily: Option<Vec<DateStep>>,
    pub weekly: Option<Vec<DateStep>>,
    pub monthly: Option<Vec<DateStep>>,
}

impl Request for Options {
    type Response = SeasonalResult;

    fn query(&self) -> Query {
        let mut query = Query::new();
        query
            .location(&self.location)
            .param("timeformat", "unixtime")
            .list("six_hourly", &self.six_hourly)
            .list("daily", &self.daily)
            .list("weekly", &self.weekly)
            .list("monthly", &self.monthly)
            .list("models", &self.models)
            .optional("temperature_unit", self.temperature_unit)
            .optional("wind_speed_unit", self.wind_speed_unit)
            .optional("precipitation_unit", self.precipitation_unit)
            .optional("timezone", self.time_zone.as_deref())
            .optional("past_days", self.past_days)
            .optional("forecast_days", self.forecast_days)
            .optional("start_date", self.start_date)
            .optional("end_date", self.end_date)
            .optional("cell_selection", self.cell_selection)
            .optional("apikey", self.apikey.as_deref());
        query
    }

    fn parse(body: &[u8]) -> Result<SeasonalResult, Error> {
        let raw = RawResponse::from_json(body)?;
        Ok(SeasonalResult {
            six_hourly: raw.six_hourly()?,
            daily: raw.daily()?,
            weekly: raw.weekly()?,
            monthly: raw.monthly()?,
        })
    }
}
