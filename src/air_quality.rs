use std::collections::HashMap;
use std::error::Error;

use crate::forecast::{
    extract_times, unix_time_to_naive_datetime, CellSelection, CurrentResult, ForecastResultHourly,
    ForecastResultItem,
};
use crate::{client, errors, location};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub location: location::Location,
    /// Attributes to request in hourly intervals
    pub hourly: Vec<String>,
    /// Attributes to request for the current values
    pub current: Vec<String>,
    pub domains: Option<String>,
    /// Timeformat is always set to unix
    pub time_zone: Option<String>,
    pub past_days: Option<u8>,
    pub forecast_days: Option<u8>,
    pub forecast_hours: Option<u32>,
    pub past_hours: Option<u32>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub cell_selection: Option<CellSelection>,
    pub apikey: Option<String>,
}

impl Options {
    fn to_params(self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = Vec::new();

        params.push(("latitude".into(), self.location.lat.to_string()));
        params.push(("longitude".into(), self.location.lng.to_string()));
        params.push(("timeformat".into(), "unixtime".into()));
        if !self.hourly.is_empty() {
            params.push(("hourly".into(), self.hourly.join(",")));
        }
        if !self.current.is_empty() {
            params.push(("current".into(), self.current.join(",")));
        }

        if let Some(domains) = self.domains {
            params.push(("domains".into(), domains));
        }
        if let Some(time_zone) = self.time_zone {
            params.push(("timezone".into(), time_zone));
        }
        if let Some(past_days) = self.past_days {
            params.push(("past_days".into(), past_days.to_string()));
        }
        if let Some(forecast_days) = self.forecast_days {
            params.push(("forecast_days".into(), forecast_days.to_string()));
        }
        if let Some(forecast_hours) = self.forecast_hours {
            params.push(("forecast_hours".into(), forecast_hours.to_string()));
        }
        if let Some(past_hours) = self.past_hours {
            params.push(("past_hours".into(), past_hours.to_string()));
        }
        if let Some(start_date) = self.start_date {
            params.push((
                "start_date".into(),
                start_date.format("%Y-%m-%d").to_string(),
            ));
        }
        if let Some(end_date) = self.end_date {
            params.push(("end_date".into(), end_date.format("%Y-%m-%d").to_string()));
        }
        if let Some(cell_selection) = self.cell_selection {
            params.push(("cell_selection".into(), cell_selection.to_string()));
        }
        if let Some(apikey) = self.apikey {
            params.push(("apikey".into(), apikey.to_string()));
        }

        params
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiAirQualityResponse {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub generationtime_ms: Option<f64>,
    pub utc_offset_seconds: Option<i32>,
    pub timezone: Option<String>,
    pub timezone_abbreviation: Option<String>,
    pub elevation: Option<f32>,
    pub current_units: Option<HashMap<String, String>>,
    pub current: Option<HashMap<String, serde_json::Value>>,
    pub hourly_units: Option<HashMap<String, String>>,
    pub hourly: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AirQualityResult {
    pub current: Option<CurrentResult>,
    pub hourly: Option<Vec<ForecastResultHourly>>,
}

fn api_to_result(api_res: ApiAirQualityResponse) -> Result<AirQualityResult, Box<dyn Error>> {
    let mut result = AirQualityResult::default();

    if let Some(current) = api_res.current {
        let api_units = api_res.current_units.clone();
        // Iterates on values
        let mut current_result = CurrentResult::default();
        for (k, v) in current.iter() {
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
    };

    let utc_offset_seconds = api_res.utc_offset_seconds.unwrap_or(0);
    if let Some(hourly) = api_res.hourly {
        if let Some(hourly_date_times) = extract_times(&hourly, utc_offset_seconds)? {
            let api_units = api_res.hourly_units.clone();
            let mut hourly_result = Vec::new();

            // Iterate on times
            for (idx, time) in hourly_date_times.iter().enumerate() {
                let mut hourly_rec = ForecastResultHourly::default();
                // Iterates on values
                for (k, v) in hourly.iter() {
                    if k == "time" {
                        continue;
                    }

                    let v_arr = v.as_array().expect("Cannot decode JSON");

                    let value = v_arr[idx].clone();
                    // Try to find unit
                    let unit = api_units.as_ref().and_then(|units| units.get(k).cloned());
                    // Push to hourly record
                    hourly_rec = ForecastResultHourly {
                        datetime: *time,
                        values: HashMap::from_iter([(
                            k.clone(),
                            ForecastResultItem { unit, value },
                        )]),
                    }
                }

                // Push hourly rec
                hourly_result.push(hourly_rec);
            }

            result.hourly = Some(hourly_result);
        }
    }

    Ok(result)
}

impl client::Client {
    /// Request forecast data
    pub async fn air_quality(&self, opts: Options) -> Result<AirQualityResult, Box<dyn Error>> {
        let url = reqwest::Url::parse_with_params(&self.air_quality_endpoint, opts.to_params())?;
        let res = self.http_client.get(url).send().await?;

        if res.status().is_success() {
            let res = res.json::<ApiAirQualityResponse>().await?;
            return api_to_result(res);
        }

        Err(Box::new(errors::ClientError::InvalidResponseStatus {
            status_code: res.status().as_u16(),
            text: res.text().await.unwrap_or("".into()),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use chrono::Duration;

    #[tokio::test]
    async fn get_air_quality_single() {
        let clt = Client::new();
        let opts = Options {
            location: Location {
                lat: 52.52,
                lng: 13.41,
            },
            current: vec!["sulphur_dioxide".into()],
            hourly: vec!["ozone".into(), "dust".into()],
            time_zone: Some(chrono_tz::Tz::Europe__Paris.to_string()),
            start_date: Some(chrono::Utc::now().date_naive()),
            end_date: Some((chrono::Utc::now() + Duration::days(4)).date_naive()),
            ..Default::default()
        };

        let res = clt.air_quality(opts).await.unwrap();
        println!("{:#?}", res);
    }
}
