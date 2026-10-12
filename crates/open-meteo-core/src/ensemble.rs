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

/// Parameters of an ensemble request.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    /// Coordinates of the place to query.
    pub location: Location,
    /// Elevation used for statistical downscaling. Defaults to a 90 m digital elevation model.
    pub elevation: Option<Elevation>,
    /// Variables to return at hourly resolution.
    pub hourly: Vec<String>,
    /// Ensemble models to query. The API requires at least one.
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
    /// 0 to 35.
    pub forecast_days: Option<u8>,
    /// Also return this many hours before now. Hourly data only.
    pub past_hours: Option<u32>,
    /// Number of hours to return, starting now. Hourly data only.
    pub forecast_hours: Option<u32>,
    /// First day of the requested range. Mutually exclusive with `past_days` and `forecast_days`.
    pub start_date: Option<NaiveDate>,
    /// Last day of the requested range, inclusive.
    pub end_date: Option<NaiveDate>,
    /// Hour-precise start of the requested range, for hourly data.
    pub start_hour: Option<NaiveDateTime>,
    /// Hour-precise end of the requested range, inclusive.
    pub end_hour: Option<NaiveDateTime>,
    /// How to choose the grid cell for the coordinates. The API defaults to `land` on most endpoints.
    pub cell_selection: Option<CellSelection>,
    /// API key for the commercial API. Not needed for the free tier.
    pub apikey: Option<String>,
}

/// Ensemble data. Members are separate variables of each time step.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EnsembleResult {
    /// Hourly values, present when `hourly` variables were requested.
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
