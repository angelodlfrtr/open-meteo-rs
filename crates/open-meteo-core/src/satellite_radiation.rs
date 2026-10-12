//! Solar radiation measured by satellites, optionally projected on a tilted panel.

use alloc::string::String;
use alloc::vec::Vec;

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::forecast::hour;
use crate::response::RawResponse;
use crate::{CellSelection, Error, Location, Model, Query, Request, TemporalResolution, TimeStep};

/// Parameters of a satellite radiation request.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    /// Coordinates of the place to query.
    pub location: Location,
    /// Radiation variables to return, e.g. `shortwave_radiation`.
    pub hourly: Vec<String>,
    /// Satellite products to query, e.g. [`Model::SatelliteRadiationSeamless`].
    pub models: Vec<Model>,
    /// Keep the satellite's own time step, or aggregate to hours.
    pub temporal_resolution: Option<TemporalResolution>,
    /// Panel tilt in degrees from horizontal, for `global_tilted_irradiance`.
    pub tilt: Option<f32>,
    /// Panel azimuth in degrees, 0 facing south, for `global_tilted_irradiance`.
    pub azimuth: Option<f32>,
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
    /// Hour-precise start of the requested range, for hourly data.
    pub start_hour: Option<NaiveDateTime>,
    /// Hour-precise end of the requested range, inclusive.
    pub end_hour: Option<NaiveDateTime>,
    /// How to choose the grid cell for the coordinates. The API defaults to `land` on most endpoints.
    pub cell_selection: Option<CellSelection>,
    /// API key for the commercial API. Not needed for the free tier.
    pub apikey: Option<String>,
}

/// Satellite radiation data. Hours not yet observed come back as null values.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SatelliteRadiationResult {
    /// Hourly values, present when `hourly` variables were requested.
    pub hourly: Option<Vec<TimeStep>>,
}

impl Request for Options {
    type Response = SatelliteRadiationResult;

    fn query(&self) -> Query {
        let mut query = Query::new();
        query
            .location(&self.location)
            .param("timeformat", "unixtime")
            .list("hourly", &self.hourly)
            .list("models", &self.models)
            .optional("temporal_resolution", self.temporal_resolution)
            .optional("tilt", self.tilt)
            .optional("azimuth", self.azimuth)
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

    fn parse(body: &[u8]) -> Result<SatelliteRadiationResult, Error> {
        let raw = RawResponse::from_json(body)?;
        Ok(SatelliteRadiationResult {
            hourly: raw.hourly()?,
        })
    }
}
