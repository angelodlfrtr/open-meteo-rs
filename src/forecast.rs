use super::{client, errors, forecast, location};
use chrono::TimeZone;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Elevation {
    Nan,
    Value(f32),
}

impl Display for Elevation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nan => write!(f, "nan"),
            Self::Value(v) => write!(f, "{v}"),
        }
    }
}

impl From<Elevation> for String {
    fn from(value: Elevation) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for Elevation {
    type Error = errors::ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "nan" {
            return Ok(Self::Nan);
        }

        Err(errors::ConversionError::InvalidElevation {
            elevation: value.to_string(),
        })
    }
}

impl From<f32> for Elevation {
    fn from(value: f32) -> Self {
        Self::Value(value)
    }
}

#[derive(Debug, Clone)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Celsius => write!(f, "celsius"),
            Self::Fahrenheit => write!(f, "fahrenheit"),
        }
    }
}

impl From<TemperatureUnit> for String {
    fn from(value: TemperatureUnit) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for TemperatureUnit {
    type Error = errors::ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "celsius" => Ok(Self::Celsius),
            "fahrenheit" => Ok(Self::Fahrenheit),
            _ => Err(errors::ConversionError::InvalidTemperatureUnit {
                unit: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WindSpeedUnit {
    Kmh,
    Ms,
    Mph,
    Kn,
}

impl Display for WindSpeedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kmh => write!(f, "kmh"),
            Self::Ms => write!(f, "ms"),
            Self::Mph => write!(f, "mph"),
            Self::Kn => write!(f, "kn"),
        }
    }
}

impl From<WindSpeedUnit> for String {
    /// Default to kmh
    fn from(value: WindSpeedUnit) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for WindSpeedUnit {
    type Error = errors::ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "kmh" => Ok(Self::Kmh),
            "ms" => Ok(Self::Ms),
            "mph" => Ok(Self::Mph),
            "kn" => Ok(Self::Kn),
            _ => Err(errors::ConversionError::InvalidWindspeedUnit {
                unit: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PrecipitationUnit {
    Millimeters,
    Inches,
}

impl Display for PrecipitationUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Millimeters => write!(f, "mm"),
            Self::Inches => write!(f, "inch"),
        }
    }
}

impl From<PrecipitationUnit> for String {
    /// Default to mm
    fn from(value: PrecipitationUnit) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for PrecipitationUnit {
    type Error = errors::ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "inch" => Ok(Self::Inches),
            "mm" => Ok(Self::Millimeters),
            _ => Err(errors::ConversionError::InvalidPrecipitationUnit {
                unit: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CellSelection {
    Land,
    Sea,
    Nearest,
}

impl Display for CellSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Land => write!(f, "land"),
            Self::Sea => write!(f, "sea"),
            Self::Nearest => write!(f, "nearest"),
        }
    }
}

impl From<CellSelection> for String {
    fn from(value: CellSelection) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for CellSelection {
    type Error = errors::ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "land" => Ok(Self::Land),
            "sea" => Ok(Self::Sea),
            "nearest" => Ok(Self::Nearest),
            _ => Err(crate::ConversionError::InvalidCellSelection {
                selection: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    pub location: location::Location,
    pub elevation: Option<Elevation>,
    /// Attributes to request for `minutely_15` forecast
    pub minutely_15: Vec<String>,
    /// Attributes to request in hourly intervals
    pub hourly: Vec<String>,
    /// Attributes to request in daily intervals
    pub daily: Vec<String>,
    /// Attributes to request for current weather
    pub current: Vec<String>,
    pub temperature_unit: Option<TemperatureUnit>,
    pub wind_speed_unit: Option<WindSpeedUnit>,
    pub precipitation_unit: Option<PrecipitationUnit>,
    pub time_zone: Option<String>,
    pub past_days: Option<u8>,
    pub forecast_days: Option<u8>,
    // max minutely_15 data points is 1536
    pub forecast_minutely_15: Option<u16>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub models: Option<Vec<String>>,
    pub cell_selection: Option<CellSelection>,
    pub apikey: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            location: location::Location::default(),
            elevation: None,
            minutely_15: Vec::new(),
            hourly: Vec::new(),
            daily: Vec::new(),
            current: Vec::new(),
            temperature_unit: None,
            wind_speed_unit: None,
            precipitation_unit: None,
            time_zone: Some("UTC".into()),
            past_days: None,
            forecast_days: None,
            forecast_minutely_15: None,
            start_date: None,
            end_date: None,
            models: None,
            cell_selection: None,
            apikey: None,
        }
    }
}

impl Options {
    #[must_use]
    pub fn as_params(self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        params.push(("latitude".into(), self.location.lat.to_string()));
        params.push(("longitude".into(), self.location.lng.to_string()));
        params.push(("timeformat".into(), "unixtime".into()));

        if let Some(v) = self.elevation {
            params.push(("elevation".into(), v.into()));
        }

        if let Some(v) = self.temperature_unit {
            params.push(("temperature_unit".into(), v.into()));
        }

        if let Some(v) = self.wind_speed_unit {
            params.push(("windspeed_unit".into(), v.into()));
        }

        if let Some(v) = self.precipitation_unit {
            params.push(("precipitation_unit".into(), v.into()));
        }

        if let Some(v) = self.time_zone {
            params.push(("timezone".into(), v.clone()));
        }

        if let Some(v) = self.past_days {
            params.push(("past_days".into(), v.to_string()));
        }

        if let Some(v) = self.forecast_minutely_15 {
            params.push(("forecast_minutely_15".into(), v.to_string()));
        }

        if let Some(v) = self.forecast_days {
            params.push(("forecast_days".into(), v.to_string()));
        }

        if let Some(v) = self.start_date {
            params.push(("start_date".into(), v.format("%Y-%m-%d").to_string()));
        }

        if let Some(v) = self.end_date {
            params.push(("end_date".into(), v.format("%Y-%m-%d").to_string()));
        }

        if !self.current.is_empty() {
            params.push(("current".into(), self.current.join(",")));
        }

        if !self.minutely_15.is_empty() {
            params.push(("minutely_15".into(), self.minutely_15.join(",")));
        }

        if !self.hourly.is_empty() {
            params.push(("hourly".into(), self.hourly.join(",")));
        }

        if !self.daily.is_empty() {
            params.push(("daily".into(), self.daily.join(",")));
        }

        if let Some(models) = self.models {
            if !models.is_empty() {
                params.push(("models".into(), models.join(",")));
            }
        }

        if let Some(v) = self.cell_selection {
            params.push(("cell_selection".into(), v.into()));
        }

        if let Some(apikey) = self.apikey {
            params.push(("apikey".into(), apikey.clone()));
        }

        params
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiForecastResponse {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation: Option<f32>,
    pub generationtime_ms: Option<f64>,
    pub utc_offset_seconds: Option<i32>,
    pub timezone: Option<String>,
    pub timezone_abbreviation: Option<String>,
    pub current_units: Option<HashMap<String, String>>,
    pub current: Option<HashMap<String, serde_json::Value>>,
    pub minutely_15_units: Option<HashMap<String, String>>,
    pub minutely_15: Option<HashMap<String, serde_json::Value>>,
    pub hourly_units: Option<HashMap<String, String>>,
    pub hourly: Option<HashMap<String, serde_json::Value>>,
    pub daily_units: Option<HashMap<String, String>>,
    pub daily: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForecastResultItem {
    pub unit: Option<String>,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForecastResultHourly {
    pub datetime: chrono::NaiveDateTime,
    pub values: HashMap<String, ForecastResultItem>,
}

pub type CurrentResult = ForecastResultHourly;
pub type ForecastResultMinutely15 = ForecastResultHourly;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForecastResultDaily {
    pub date: chrono::NaiveDate,
    pub values: HashMap<String, ForecastResultItem>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForecastResult {
    pub current: Option<CurrentResult>,
    pub minutely_15: Option<Vec<ForecastResultMinutely15>>,
    pub hourly: Option<Vec<ForecastResultHourly>>,
    pub daily: Option<Vec<ForecastResultDaily>>,
}

impl client::Client {
    /// Request forecast data
    ///
    /// ### Errors
    ///
    /// Return an `Err` if api return an error or in case of network error.
    pub async fn forecast(&self, opts: Options) -> Result<ForecastResult, Box<dyn Error>> {
        self.request(opts, &format!("{}forecast", self.forecast_endpoint))
            .await
    }

    /// Request data from the archive (historic weather data)
    ///
    /// ### Errors
    ///
    /// Return an `Err` if api return an error or in case of network error.
    pub async fn archive(&self, opts: Options) -> Result<ForecastResult, Box<dyn Error>> {
        self.request(opts, &format!("{}archive", self.archive_endpoint))
            .await
    }

    #[allow(clippy::too_many_lines)]
    async fn request(
        &self,
        opts: Options,
        api_endpoint: &str,
    ) -> Result<ForecastResult, Box<dyn Error>> {
        let url = reqwest::Url::parse_with_params(api_endpoint, opts.as_params())?;
        let res = self.http_client.get(url).send().await?;

        if res.status().is_success() {
            let api_res = res.json::<ApiForecastResponse>().await?;
            let mut result = ForecastResult::default();

            // Current weather
            if let Some(current) = api_res.current {
                let api_units = api_res.current_units.clone();
                // Iterates on values
                let mut current_result = CurrentResult::default();
                for (k, v) in &current {
                    if k == "time" {
                        current_result.datetime = match v.as_i64() {
                            Some(v) => unix_time_to_naive_datetime(v, 0),
                            None => {
                                return Err("cannot decode properly json input".into());
                            }
                        };
                        continue;
                    }
                    // Try to find the unit
                    let unit = api_units.as_ref().and_then(|units| units.get(k).cloned());
                    let value = v.clone();
                    current_result
                        .values
                        .insert(k.clone(), ForecastResultItem { unit, value });
                }

                // Push current rec
                result.current = Some(current_result);
            }

            // Get utc offset
            let utc_offset_seconds = api_res.utc_offset_seconds.unwrap_or(0);

            // Minutely 15
            if let Some(minutely_15) = api_res.minutely_15 {
                if let Some(minutely_15_date_times) =
                    extract_times(&minutely_15, utc_offset_seconds)?
                {
                    if let Some(minutely_15_units) = api_res.minutely_15_units {
                        let mut minutely_15_result = Vec::new();

                        // Iterate on times
                        for (idx, time) in minutely_15_date_times.iter().enumerate() {
                            let mut minutely_15_rec = ForecastResultMinutely15 {
                                datetime: *time,
                                ..Default::default()
                            };

                            // Iterates on values
                            for (k, v) in &minutely_15 {
                                if k == "time" {
                                    continue;
                                }

                                let mut item = ForecastResultItem::default();
                                let Some(v_arr) = v.as_array() else {
                                    return Err("cannot decode properly json input".into());
                                };

                                let v_val = v_arr[idx].clone();
                                item.value = v_val;

                                // Try to find unit
                                if let Some(unit) = minutely_15_units.get(k) {
                                    item.unit = Some(unit.clone());
                                }

                                // Push to minutely_15 record
                                minutely_15_rec.values.insert(k.clone(), item);
                            }

                            // Push minutely_15 rec
                            minutely_15_result.push(minutely_15_rec);
                        }

                        result.minutely_15 = Some(minutely_15_result);
                    }
                }
            }

            // Hourly
            if let Some(hourly) = api_res.hourly {
                if let Some(hourly_date_times) = extract_times(&hourly, utc_offset_seconds)? {
                    if let Some(hourly_units) = api_res.hourly_units {
                        let mut hourly_result = Vec::new();

                        // Iterate on times
                        for (idx, time) in hourly_date_times.iter().enumerate() {
                            let mut hourly_rec = forecast::ForecastResultHourly {
                                datetime: *time,
                                ..Default::default()
                            };

                            // Iterates on values
                            for (k, v) in &hourly {
                                if k == "time" {
                                    continue;
                                }

                                let mut item = ForecastResultItem::default();
                                let Some(v_arr) = v.as_array() else {
                                    return Err("cannot decode properly json input".into());
                                };

                                let v_val = v_arr[idx].clone();
                                item.value = v_val;

                                // Try to find unit
                                if let Some(unit) = hourly_units.get(k) {
                                    item.unit = Some(unit.clone());
                                }

                                // Push to hourly record
                                hourly_rec.values.insert(k.clone(), item);
                            }

                            // Push hourly rec
                            hourly_result.push(hourly_rec);
                        }

                        result.hourly = Some(hourly_result);
                    }
                }
            }

            // Daily
            if let Some(daily) = api_res.daily {
                if let Some(daily_date_times) = extract_times(&daily, utc_offset_seconds)? {
                    if let Some(daily_units) = api_res.daily_units {
                        let mut daily_result = Vec::new();

                        // Iterate on times
                        for (idx, time) in daily_date_times.iter().enumerate() {
                            let mut daily_rec = forecast::ForecastResultDaily {
                                date: (*time).date(),
                                ..Default::default()
                            };

                            // Iterates on values
                            for (k, v) in &daily {
                                if k == "time" {
                                    continue;
                                }

                                let mut item = ForecastResultItem::default();
                                let Some(v_arr) = v.as_array() else {
                                    return Err("cannot decode properly json input".into());
                                };
                                let v_val = v_arr[idx].clone();
                                item.value = v_val;

                                // Try to find unit
                                if let Some(unit) = daily_units.get(k) {
                                    item.unit = Some(unit.clone());
                                }

                                // Push to daily record
                                daily_rec.values.insert(k.clone(), item);
                            }

                            // Push daily rec
                            daily_result.push(daily_rec);
                        }

                        result.daily = Some(daily_result);
                    }
                }
            }

            return Ok(result);
        }

        Err(Box::new(errors::ClientError::InvalidResponseStatus {
            status_code: res.status().as_u16(),
            text: res.text().await.unwrap_or(String::new()),
        }))
    }
}

#[must_use]
pub fn unix_time_to_naive_datetime(
    unix_time: i64,
    utc_offset_seconds: i32,
) -> chrono::NaiveDateTime {
    chrono::Utc
        .timestamp_millis_opt((unix_time + i64::from(utc_offset_seconds)) * 1000)
        .unwrap()
        .naive_local()
}

/// Extract times from json and return a `Option<Vec<chrono::NaiveDateTime>>`.
///
/// ### Errors
///
/// Return `Err` if json input cannot be decoded.
pub fn extract_times<S: ::std::hash::BuildHasher>(
    input: &HashMap<String, serde_json::Value, S>,
    utc_offset_seconds: i32,
) -> Result<Option<Vec<chrono::NaiveDateTime>>, Box<dyn Error>> {
    if let Some(time) = input.get("time") {
        if let Some(time_values) = time.as_array() {
            let mut hourly_datetimes = Vec::new();

            for v in time_values {
                let Some(unix_tm) = v.as_i64() else {
                    return Err("cannot decode properly json input".into());
                };

                let dd = unix_time_to_naive_datetime(unix_tm, utc_offset_seconds);

                hourly_datetimes.push(dd);
            }

            return Ok(Some(hourly_datetimes));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use futures::join;

    #[tokio::test]
    async fn get_forecast_single() {
        let clt = client::Client::new();
        let mut opts = Options {
            location: location::Location {
                lat: 52.52,
                lng: 13.41,
            },
            current: vec!["temperature_2m".into()],
            elevation: Some(8.65.into()),
            ..Default::default()
        };

        opts.elevation = Some("nan".try_into().unwrap());

        opts.minutely_15.push("temperature_2m".into());
        opts.minutely_15.push("windspeed_10m".into());
        opts.hourly.push("temperature_2m".into());
        opts.hourly.push("windspeed_120m".into());
        opts.daily.push("temperature_2m_max".into());
        opts.daily.push("shortwave_radiation_sum".into());
        opts.time_zone = Some(chrono_tz::Tz::Europe__Paris.to_string());

        opts.start_date = Some(chrono::Utc::now().date_naive());
        opts.end_date = Some((chrono::Utc::now() + Duration::days(4)).date_naive());

        let res = clt.forecast(opts).await.unwrap();
        println!("{res:#?}");
    }

    #[tokio::test]
    async fn get_forecast_parallel() {
        let clt = client::Client::new();

        let mut opts = Options {
            location: location::Location {
                lat: 48.864_716,
                lng: 2.349_014,
            },
            ..Default::default()
        };

        opts.hourly.push("temperature_2m".into());

        let opts_two = opts.clone();
        let fut_one = clt.forecast(opts);
        let fut_two = clt.forecast(opts_two);

        let (res_one, res_two) = join!(fut_one, fut_two);

        println!("{:?}", res_one.unwrap());
        println!("{:?}", res_two.unwrap());
    }
}
