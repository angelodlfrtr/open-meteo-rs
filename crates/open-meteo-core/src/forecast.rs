//! Weather forecast, and the three endpoints that share its request format:
//! historical weather (`archive`), historical forecast and previous model runs.

use alloc::string::String;
use alloc::vec::Vec;

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::response::RawResponse;
use crate::{DateStep, Error, Location, Measurement, Model, Query, Request, TimeStep};

pub use crate::units::{
    CellSelection, Elevation, PrecipitationUnit, TemperatureUnit, WindSpeedUnit,
};

/// Parameters of a forecast-like request.
///
/// Variable names (`hourly`, `daily`, ...) are passed through as-is; see the Open-Meteo
/// documentation for the list each endpoint accepts.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    /// Coordinates of the place to query.
    pub location: Location,
    /// Elevation used for statistical downscaling. Defaults to a 90 m digital elevation model.
    pub elevation: Option<Elevation>,
    /// Variables to return at 15 minute resolution.
    pub minutely_15: Vec<String>,
    /// Variables to return at hourly resolution.
    pub hourly: Vec<String>,
    /// Variables to return as daily aggregations.
    pub daily: Vec<String>,
    /// Variables to return for the current conditions.
    pub current: Vec<String>,
    /// Unit for temperatures. The API defaults to Celsius.
    pub temperature_unit: Option<TemperatureUnit>,
    /// Unit for wind speeds. The API defaults to km/h.
    pub wind_speed_unit: Option<WindSpeedUnit>,
    /// Unit for precipitation. The API defaults to millimetres.
    pub precipitation_unit: Option<PrecipitationUnit>,
    /// IANA time zone name, or `auto`. Defaults to UTC on the API side.
    pub time_zone: Option<String>,
    /// Also return this many days before today.
    pub past_days: Option<u8>,
    /// Number of days to return, starting today.
    pub forecast_days: Option<u8>,
    /// Also return this many hours before now. Hourly data only.
    pub past_hours: Option<u32>,
    /// Number of hours to return, starting now. Hourly data only.
    pub forecast_hours: Option<u32>,
    /// Number of 15 minute steps to return. The API caps it at 1536.
    pub forecast_minutely_15: Option<u16>,
    /// First day of the requested range. Mutually exclusive with `past_days` and `forecast_days`.
    pub start_date: Option<NaiveDate>,
    /// Last day of the requested range, inclusive.
    pub end_date: Option<NaiveDate>,
    /// Hour-precise alternative to `start_date`/`end_date` for hourly data.
    pub start_hour: Option<NaiveDateTime>,
    /// Hour-precise end of the requested range, inclusive.
    pub end_hour: Option<NaiveDateTime>,
    /// Weather models to query. The API picks the best match for the location when empty.
    pub models: Vec<Model>,
    /// How to choose the grid cell for the coordinates. The API defaults to `land` on most endpoints.
    pub cell_selection: Option<CellSelection>,
    /// API key for the commercial API. Not needed for the free tier.
    pub apikey: Option<String>,
}

/// Forecast-like data, one entry per requested resolution.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ForecastResult {
    /// Current conditions, present when `current` variables were requested.
    pub current: Option<TimeStep>,
    /// 15 minute values, present when `minutely_15` variables were requested.
    pub minutely_15: Option<Vec<TimeStep>>,
    /// Hourly values, present when `hourly` variables were requested.
    pub hourly: Option<Vec<TimeStep>>,
    /// Daily values, present when `daily` variables were requested.
    pub daily: Option<Vec<DateStep>>,
}

/// Previous name of [`Measurement`].
pub type ForecastResultItem = Measurement;
/// Previous name of [`TimeStep`].
pub type ForecastResultHourly = TimeStep;
/// Previous name of [`TimeStep`].
pub type ForecastResultMinutely15 = TimeStep;
/// Previous name of [`TimeStep`].
pub type CurrentResult = TimeStep;
/// Previous name of [`DateStep`].
pub type ForecastResultDaily = DateStep;

impl Request for Options {
    type Response = ForecastResult;

    fn query(&self) -> Query {
        let mut query = Query::new();
        query
            .location(&self.location)
            .param("timeformat", "unixtime")
            .optional("elevation", self.elevation.as_ref())
            .optional("temperature_unit", self.temperature_unit)
            .optional("wind_speed_unit", self.wind_speed_unit)
            .optional("precipitation_unit", self.precipitation_unit)
            .optional("timezone", self.time_zone.as_deref())
            .optional("past_days", self.past_days)
            .optional("forecast_days", self.forecast_days)
            .optional("past_hours", self.past_hours)
            .optional("forecast_hours", self.forecast_hours)
            .optional("forecast_minutely_15", self.forecast_minutely_15)
            .optional("start_date", self.start_date)
            .optional("end_date", self.end_date)
            .optional("start_hour", self.start_hour.map(hour))
            .optional("end_hour", self.end_hour.map(hour))
            .list("current", &self.current)
            .list("minutely_15", &self.minutely_15)
            .list("hourly", &self.hourly)
            .list("daily", &self.daily)
            .list("models", &self.models)
            .optional("cell_selection", self.cell_selection)
            .optional("apikey", self.apikey.as_deref());
        query
    }

    fn parse(body: &[u8]) -> Result<ForecastResult, Error> {
        let raw = RawResponse::from_json(body)?;
        Ok(ForecastResult {
            current: raw.current()?,
            minutely_15: raw.minutely_15()?,
            hourly: raw.hourly()?,
            daily: raw.daily()?,
        })
    }
}

/// Format a date-time the way `start_hour`/`end_hour` expect it: `yyyy-mm-ddThh:mm`.
pub(crate) fn hour(value: NaiveDateTime) -> impl core::fmt::Display {
    value.format("%Y-%m-%dT%H:%M")
}
