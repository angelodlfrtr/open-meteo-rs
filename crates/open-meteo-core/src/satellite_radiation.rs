//! Solar radiation measured by satellites, optionally projected on a tilted panel.

use alloc::string::String;
use alloc::vec::Vec;

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::forecast::hour;
use crate::response::RawResponse;
use crate::{CellSelection, Error, Location, Model, Query, Request, TemporalResolution, TimeStep};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
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
    pub time_zone: Option<String>,
    pub past_days: Option<u8>,
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
pub struct SatelliteRadiationResult {
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
