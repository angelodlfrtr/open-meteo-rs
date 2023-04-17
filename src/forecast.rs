use super::*;
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
            Self::Value(v) => write!(f, "{}", v.to_string()),
        }
    }
}

impl From<Elevation> for String {
    fn from(value: Elevation) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for Elevation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "nan" {
            return Ok(Self::Nan);
        }

        Err(format!(
            "invalid elevation {:?}, only str nan is supported",
            value
        ))
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
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "celsius" => Ok(Self::Celsius),
            "fahrenheit" => Ok(Self::Fahrenheit),
            _ => Err(format!("invalid temperature unit {:?}", value)),
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
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "kmh" => Ok(Self::Kmh),
            "ms" => Ok(Self::Ms),
            "mph" => Ok(Self::Mph),
            "kn" => Ok(Self::Kn),
            _ => Err(format!("invalid windspeed unit {:?}", value)),
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
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "inch" => Ok(Self::Inches),
            "mm" => Ok(Self::Millimeters),
            _ => Err(format!("invalid precicitation unit {:?}", value)),
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
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "land" => Ok(Self::Land),
            "sea" => Ok(Self::Sea),
            "nearest" => Ok(Self::Nearest),
            _ => Err(format!("invalid cell selection {:?}", value)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    pub location: location::Location,
    pub elevation: Option<Elevation>,
    pub hourly: Vec<String>,
    pub daily: Vec<String>,
    pub current_weather: Option<bool>,
    pub temperature_unit: Option<TemperatureUnit>,
    pub wind_speed_unit: Option<WindSpeedUnit>,
    pub precipitation_unit: Option<PrecipitationUnit>,
    pub time_zone: Option<String>,
    pub past_days: Option<u8>,
    pub forecast_days: Option<u8>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub models: Option<Vec<String>>,
    pub cell_selection: Option<CellSelection>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            location: location::Location::default(),
            elevation: None,
            hourly: Vec::new(),
            daily: Vec::new(),
            current_weather: None,
            temperature_unit: None,
            wind_speed_unit: None,
            precipitation_unit: None,
            time_zone: Some("UTC".into()),
            past_days: None,
            forecast_days: None,
            start_date: None,
            end_date: None,
            models: None,
            cell_selection: None,
        }
    }
}

impl Options {
    pub fn as_params(self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        params.push(("latitude".into(), self.location.lat.to_string()));
        params.push(("longitude".into(), self.location.lng.to_string()));
        params.push(("timeformat".into(), "unixtime".into()));

        match self.elevation {
            Some(v) => params.push(("elevation".into(), v.into())),
            None => (),
        }

        match self.temperature_unit {
            Some(v) => params.push(("temperature_unit".into(), v.into())),
            None => (),
        }

        match self.wind_speed_unit {
            Some(v) => params.push(("windspeed_unit".into(), v.into())),
            None => (),
        }

        match self.precipitation_unit {
            Some(v) => params.push(("precipitation_unit".into(), v.into())),
            None => (),
        }

        match self.time_zone {
            Some(v) => params.push(("timezone".into(), v.to_string())),
            None => (),
        }

        match self.past_days {
            Some(v) => params.push(("past_days".into(), v.to_string())),
            None => (),
        }

        match self.forecast_days {
            Some(v) => params.push(("forecast_days".into(), v.to_string())),
            None => (),
        }

        match self.start_date {
            Some(v) => params.push(("start_date".into(), v.format("%Y-%m-%d").to_string())),
            None => (),
        }

        match self.end_date {
            Some(v) => params.push(("end_date".into(), v.format("%Y-%m-%d").to_string())),
            None => (),
        }

        match self.current_weather {
            Some(v) => {
                if v {
                    params.push(("current_weather".into(), "true".into()))
                }
            }
            None => (),
        }

        if self.hourly.len() > 0 {
            params.push(("hourly".into(), self.hourly.join(",")));
        }

        if self.daily.len() > 0 {
            params.push(("daily".into(), self.daily.join(",")));
        }

        if let Some(models) = self.models {
            if models.len() > 0 {
                params.push(("models".into(), models.join(",")));
            }
        }

        match self.cell_selection {
            Some(v) => params.push(("cell_selection".into(), v.into())),
            None => (),
        }

        params
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub temperature: Option<f64>,
    pub windspeed: Option<f64>,
    pub winddirection: Option<f64>,
    pub weathercode: Option<f64>,
    pub is_day: Option<u8>,
    pub time: Option<u64>,
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
    pub current_weather: Option<CurrentWeather>,
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForecastResultDaily {
    pub date: chrono::NaiveDate,
    pub values: HashMap<String, ForecastResultItem>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForecastResult {
    pub current_weather: Option<CurrentWeather>,
    pub hourly: Option<Vec<ForecastResultHourly>>,
    pub daily: Option<Vec<ForecastResultDaily>>,
}

impl client::Client {
    pub async fn forecast(&self, opts: Options) -> Result<ForecastResult, Box<dyn Error>> {
        let mut forecast_endpoint = self.forecast_endpoint.to_owned();
        forecast_endpoint.push_str("/forecast");
        let url = reqwest::Url::parse_with_params(&forecast_endpoint, opts.as_params())?;
        let res = self.http_client.get(url).send().await?;

        if res.status().is_success() {
            let api_res = res.json::<ApiForecastResponse>().await?;
            let mut result = ForecastResult::default();

            // Current weather
            result.current_weather = api_res.current_weather;

            // Get utc offset
            let utc_offset_seconds = api_res.utc_offset_seconds.unwrap_or(0);

            // Hourly
            if let Some(hourly) = api_res.hourly {
                if let Some(hourly_date_times) = extract_times(&hourly, utc_offset_seconds)? {
                    if let Some(hourly_units) = api_res.hourly_units {
                        let mut hourly_result = Vec::new();

                        // Iterate on times
                        for (idx, time) in hourly_date_times.iter().enumerate() {
                            let mut hourly_rec = ForecastResultHourly::default();
                            hourly_rec.datetime = *time;

                            // Iterates on values
                            for (k, v) in hourly.iter() {
                                if k == "time" {
                                    continue;
                                }

                                let mut item = ForecastResultItem::default();
                                let v_arr = match v.as_array() {
                                    Some(v) => v,
                                    None => {
                                        return Err("cannot decode properly json input".into());
                                    }
                                };

                                let v_val = v_arr[idx].clone();
                                item.value = v_val;

                                // Try to find unit
                                match hourly_units.get(k) {
                                    Some(unit) => {
                                        item.unit = Some(unit.clone());
                                    }
                                    None => (),
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
                            let mut daily_rec = ForecastResultDaily::default();
                            daily_rec.date = (*time).date();

                            // Iterates on values
                            for (k, v) in daily.iter() {
                                if k == "time" {
                                    continue;
                                }

                                let mut item = ForecastResultItem::default();
                                let v_arr = match v.as_array() {
                                    Some(v) => v,
                                    None => {
                                        return Err("cannot decode properly json input".into());
                                    }
                                };
                                let v_val = v_arr[idx].clone();
                                item.value = v_val;

                                // Try to find unit
                                match daily_units.get(k) {
                                    Some(unit) => {
                                        item.unit = Some(unit.clone());
                                    }
                                    None => (),
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
            text: res.text().await.unwrap_or("".into()),
        }))
    }
}

fn extract_times(
    input: &HashMap<String, serde_json::Value>,
    utc_offset_seconds: i32,
) -> Result<Option<Vec<chrono::NaiveDateTime>>, Box<dyn Error>> {
    if let Some(time) = input.get("time") {
        if let Some(time_values) = time.as_array() {
            let mut hourly_datetimes = Vec::new();

            for v in time_values.iter() {
                let unix_tm = match v.as_i64() {
                    Some(v) => v,
                    None => {
                        return Err("cannot decode properly json input".into());
                    }
                };

                let dd = chrono::Utc
                    .timestamp_millis_opt((unix_tm + utc_offset_seconds as i64) * 1000)
                    .unwrap()
                    .naive_local();

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
        let clt = Client::new();
        let mut opts = Options::default();
        opts.location = Location {
            lat: 52.52,
            lng: 13.41,
        };
        opts.current_weather = Some(true);
        opts.elevation = Some("nan".try_into().unwrap());
        opts.elevation = Some(8.65.into());

        opts.hourly.push("temperature_2m".into());
        opts.hourly.push("windspeed_120m".into());
        opts.daily.push("temperature_2m_max".into());
        opts.daily.push("shortwave_radiation_sum".into());
        opts.time_zone = Some(chrono_tz::Tz::Europe__Paris.to_string());

        opts.start_date = Some(chrono::Utc::now().date_naive());
        opts.end_date = Some((chrono::Utc::now() + Duration::days(4)).date_naive());

        let res = clt.forecast(opts).await.unwrap();
        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn get_forecast_parallel() {
        let clt = Client::new();

        let mut opts = Options::default();
        opts.location = Location {
            lat: 48.864716,
            lng: 2.349014,
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
