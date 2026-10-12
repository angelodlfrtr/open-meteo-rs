//! The JSON envelope shared by every time-series endpoint, and the typed records it is turned into.
//!
//! Open-Meteo answers all weather-like endpoints with the same layout: an optional `current`
//! object plus optional time-series sections (`minutely_15`, `hourly`, `six_hourly`, `daily`,
//! `weekly`, `monthly`). Each section is a column store: a `time` array and one array per
//! variable, all the same length, with units in a sibling `<section>_units` object.
//!
//! [`RawResponse`] mirrors that layout. Its methods pivot the columns into one record per
//! time step so callers can work row by row.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use chrono::{DateTime, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Error;

/// One variable at one point in time, with the unit the API reported for it.
///
/// `value` is kept as raw JSON because the API returns numbers, strings (e.g. ISO
/// sunrise times) and nulls depending on the variable.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Measurement {
    /// Unit as reported by the API, e.g. `°C`, when it provided one.
    pub unit: Option<String>,
    /// The raw JSON value: a number, a string, or null when the API has no data.
    pub value: Value,
}

/// All variables at one instant, as returned by `current` and sub-daily sections.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TimeStep {
    /// Local time in the requested time zone.
    pub datetime: NaiveDateTime,
    /// Every requested variable, by name.
    pub values: BTreeMap<String, Measurement>,
}

/// All variables for one day, week or month.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DateStep {
    /// Local date in the requested time zone.
    pub date: NaiveDate,
    /// Every requested variable, by name.
    pub values: BTreeMap<String, Measurement>,
}

type Section = BTreeMap<String, Value>;
type Units = BTreeMap<String, String>;
/// One pivoted time step before it is given its final shape.
type Row = (NaiveDateTime, BTreeMap<String, Measurement>);

/// The envelope of every time-series endpoint, exactly as the API sends it.
#[derive(Debug, Default, Deserialize)]
pub(crate) struct RawResponse {
    #[serde(default)]
    utc_offset_seconds: i64,
    current: Option<Section>,
    current_units: Option<Units>,
    minutely_15: Option<Section>,
    minutely_15_units: Option<Units>,
    hourly: Option<Section>,
    hourly_units: Option<Units>,
    six_hourly: Option<Section>,
    six_hourly_units: Option<Units>,
    daily: Option<Section>,
    daily_units: Option<Units>,
    weekly: Option<Section>,
    weekly_units: Option<Units>,
    monthly: Option<Section>,
    monthly_units: Option<Units>,
}

impl RawResponse {
    pub(crate) fn from_json(body: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice(body)?)
    }

    pub(crate) fn current(&self) -> Result<Option<TimeStep>, Error> {
        let Some(section) = &self.current else {
            return Ok(None);
        };

        let time = section
            .get("time")
            .ok_or(Error::Malformed("`current` has no `time`"))?;
        let mut values = BTreeMap::new();
        for (name, value) in section {
            if name != "time" {
                values.insert(
                    name.clone(),
                    self.measurement(self.current_units.as_ref(), name, value.clone()),
                );
            }
        }

        Ok(Some(TimeStep {
            datetime: self.local_datetime(time)?,
            values,
        }))
    }

    pub(crate) fn minutely_15(&self) -> Result<Option<Vec<TimeStep>>, Error> {
        self.time_steps(self.minutely_15.as_ref(), self.minutely_15_units.as_ref())
    }

    pub(crate) fn hourly(&self) -> Result<Option<Vec<TimeStep>>, Error> {
        self.time_steps(self.hourly.as_ref(), self.hourly_units.as_ref())
    }

    pub(crate) fn six_hourly(&self) -> Result<Option<Vec<TimeStep>>, Error> {
        self.time_steps(self.six_hourly.as_ref(), self.six_hourly_units.as_ref())
    }

    pub(crate) fn daily(&self) -> Result<Option<Vec<DateStep>>, Error> {
        self.date_steps(self.daily.as_ref(), self.daily_units.as_ref())
    }

    pub(crate) fn weekly(&self) -> Result<Option<Vec<DateStep>>, Error> {
        self.date_steps(self.weekly.as_ref(), self.weekly_units.as_ref())
    }

    pub(crate) fn monthly(&self) -> Result<Option<Vec<DateStep>>, Error> {
        self.date_steps(self.monthly.as_ref(), self.monthly_units.as_ref())
    }

    fn time_steps(
        &self,
        section: Option<&Section>,
        units: Option<&Units>,
    ) -> Result<Option<Vec<TimeStep>>, Error> {
        let Some(rows) = self.rows(section, units)? else {
            return Ok(None);
        };

        Ok(Some(
            rows.into_iter()
                .map(|(datetime, values)| TimeStep { datetime, values })
                .collect(),
        ))
    }

    fn date_steps(
        &self,
        section: Option<&Section>,
        units: Option<&Units>,
    ) -> Result<Option<Vec<DateStep>>, Error> {
        let Some(rows) = self.rows(section, units)? else {
            return Ok(None);
        };

        Ok(Some(
            rows.into_iter()
                .map(|(datetime, values)| DateStep {
                    date: datetime.date(),
                    values,
                })
                .collect(),
        ))
    }

    /// Pivot a column-oriented section into one row per entry of its `time` array.
    fn rows(
        &self,
        section: Option<&Section>,
        units: Option<&Units>,
    ) -> Result<Option<Vec<Row>>, Error> {
        let Some(section) = section else {
            return Ok(None);
        };

        let times = section
            .get("time")
            .and_then(Value::as_array)
            .ok_or(Error::Malformed("section has no `time` array"))?;

        let mut rows = Vec::with_capacity(times.len());
        for (index, time) in times.iter().enumerate() {
            let mut values = BTreeMap::new();
            for (name, column) in section {
                if name == "time" {
                    continue;
                }
                let value = column
                    .as_array()
                    .and_then(|column| column.get(index))
                    .ok_or(Error::Malformed(
                        "a variable has fewer values than the `time` array",
                    ))?;
                values.insert(name.clone(), self.measurement(units, name, value.clone()));
            }
            rows.push((self.local_datetime(time)?, values));
        }

        Ok(Some(rows))
    }

    #[allow(clippy::unused_self)]
    fn measurement(&self, units: Option<&Units>, name: &str, value: Value) -> Measurement {
        Measurement {
            unit: units.and_then(|units| units.get(name).cloned()),
            value,
        }
    }

    /// Convert a `unixtime` value into the local time of the requested time zone.
    ///
    /// Requests always ask for `timeformat=unixtime`, so timestamps are UTC seconds and the
    /// API tells us the zone's offset separately.
    fn local_datetime(&self, time: &Value) -> Result<NaiveDateTime, Error> {
        let timestamp = time
            .as_i64()
            .ok_or(Error::Malformed("`time` is not a unix timestamp"))?;

        DateTime::from_timestamp(timestamp + self.utc_offset_seconds, 0)
            .map(|datetime| datetime.naive_utc())
            .ok_or(Error::Malformed("`time` is out of range"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response(json: &str) -> RawResponse {
        RawResponse::from_json(json.as_bytes()).unwrap()
    }

    #[test]
    fn pivots_columns_into_rows_with_units_and_local_time() {
        let raw = response(
            r#"{
                "utc_offset_seconds": 3600,
                "hourly_units": {"time": "unixtime", "temperature_2m": "°C"},
                "hourly": {"time": [1700000000, 1700003600], "temperature_2m": [1.5, null]}
            }"#,
        );

        let hourly = raw.hourly().unwrap().unwrap();
        assert_eq!(hourly.len(), 2);
        assert_eq!(
            hourly[0].datetime,
            DateTime::from_timestamp(1_700_000_000 + 3600, 0)
                .unwrap()
                .naive_utc()
        );
        assert_eq!(
            hourly[0].values["temperature_2m"].unit.as_deref(),
            Some("°C")
        );
        assert_eq!(hourly[0].values["temperature_2m"].value, Value::from(1.5));
        assert_eq!(hourly[1].values["temperature_2m"].value, Value::Null);
        assert!(raw.daily().unwrap().is_none());
    }

    #[test]
    fn current_is_a_single_time_step() {
        let raw =
            response(r#"{"current": {"time": 1700000000, "interval": 900, "wave_height": 0.3}}"#);

        let current = raw.current().unwrap().unwrap();
        assert_eq!(current.values.len(), 2);
        assert_eq!(current.values["wave_height"].unit, None);
    }

    #[test]
    fn short_columns_are_rejected() {
        let raw = response(r#"{"daily": {"time": [1700000000, 1700086400], "rain_sum": [0.0]}}"#);
        assert!(matches!(raw.daily(), Err(Error::Malformed(_))));
    }

    #[test]
    fn missing_time_is_rejected() {
        let raw = response(r#"{"hourly": {"temperature_2m": [1.0]}}"#);
        assert!(matches!(raw.hourly(), Err(Error::Malformed(_))));
    }
}
