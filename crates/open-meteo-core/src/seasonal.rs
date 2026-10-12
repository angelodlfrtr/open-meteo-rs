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

/// Parameters of a seasonal forecast request.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    /// Coordinates of the place to query.
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
    /// Unit for temperatures. The API defaults to Celsius.
    pub temperature_unit: Option<TemperatureUnit>,
    /// Unit for wind speeds. The API defaults to km/h.
    pub wind_speed_unit: Option<WindSpeedUnit>,
    /// Unit for precipitation. The API defaults to millimetres.
    pub precipitation_unit: Option<PrecipitationUnit>,
    /// IANA time zone name, or `auto` to derive it from the coordinates. Times in the result are local to it. The API defaults to UTC.
    pub time_zone: Option<String>,
    /// Also return this many days before today.
    pub past_days: Option<u8>,
    /// Up to about 7 months. The API defaults to 6 months.
    pub forecast_days: Option<u16>,
    /// First day of the requested range. Mutually exclusive with `past_days` and `forecast_days`.
    pub start_date: Option<NaiveDate>,
    /// Last day of the requested range, inclusive.
    pub end_date: Option<NaiveDate>,
    /// How to choose the grid cell for the coordinates. The API defaults to `land` on most endpoints.
    pub cell_selection: Option<CellSelection>,
    /// API key for the commercial API. Not needed for the free tier.
    pub apikey: Option<String>,
}

/// Seasonal forecast data, one entry per requested resolution.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SeasonalResult {
    /// 6 hourly values, present when `six_hourly` variables were requested.
    pub six_hourly: Option<Vec<TimeStep>>,
    /// Daily values, present when `daily` variables were requested.
    pub daily: Option<Vec<DateStep>>,
    /// Weekly values, present when `weekly` variables were requested.
    pub weekly: Option<Vec<DateStep>>,
    /// Monthly values, present when `monthly` variables were requested.
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
