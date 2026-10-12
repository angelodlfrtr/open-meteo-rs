//! Climate projections: daily data from 1950 to 2050 out of CMIP6 models.

use alloc::string::String;
use alloc::vec::Vec;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::response::RawResponse;
use crate::{
    CellSelection, DateStep, Error, Location, Model, PrecipitationUnit, Query, Request,
    TemperatureUnit, WindSpeedUnit,
};

/// Parameters of a climate request. Unlike the other endpoints, the date range and the
/// models are required by the API, so they are not optional here.
#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    pub location: Location,
    /// First day of the range, 1950-01-01 at the earliest.
    pub start_date: NaiveDate,
    /// Last day of the range, 2050-12-31 at the latest.
    pub end_date: NaiveDate,
    /// Climate models to query, e.g. [`Model::MRIAGCM32S`].
    pub models: Vec<Model>,
    /// Variables to return as daily aggregations.
    pub daily: Vec<String>,
    pub temperature_unit: Option<TemperatureUnit>,
    pub wind_speed_unit: Option<WindSpeedUnit>,
    pub precipitation_unit: Option<PrecipitationUnit>,
    /// Return raw model output instead of values bias-corrected against ERA5-Land.
    pub disable_bias_correction: bool,
    pub cell_selection: Option<CellSelection>,
    pub apikey: Option<String>,
}

impl Default for Options {
    /// Every field empty except the date range, which covers the current climate normal
    /// period (1991 to 2020). Set `models` and `daily` before sending.
    fn default() -> Self {
        Self {
            location: Location::default(),
            start_date: NaiveDate::from_ymd_opt(1991, 1, 1).expect("valid date"),
            end_date: NaiveDate::from_ymd_opt(2020, 12, 31).expect("valid date"),
            models: Vec::new(),
            daily: Vec::new(),
            temperature_unit: None,
            wind_speed_unit: None,
            precipitation_unit: None,
            disable_bias_correction: false,
            cell_selection: None,
            apikey: None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ClimateResult {
    pub daily: Option<Vec<DateStep>>,
}

impl Request for Options {
    type Response = ClimateResult;

    fn query(&self) -> Query {
        let mut query = Query::new();
        query
            .location(&self.location)
            .param("timeformat", "unixtime")
            .param("start_date", self.start_date)
            .param("end_date", self.end_date)
            .list("models", &self.models)
            .list("daily", &self.daily)
            .optional("temperature_unit", self.temperature_unit)
            .optional("wind_speed_unit", self.wind_speed_unit)
            .optional("precipitation_unit", self.precipitation_unit)
            .flag("disable_bias_correction", self.disable_bias_correction)
            .optional("cell_selection", self.cell_selection)
            .optional("apikey", self.apikey.as_deref());
        query
    }

    fn parse(body: &[u8]) -> Result<ClimateResult, Error> {
        let raw = RawResponse::from_json(body)?;
        Ok(ClimateResult {
            daily: raw.daily()?,
        })
    }
}
