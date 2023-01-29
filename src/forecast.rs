use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Options {
    pub location: location::Location,
    pub temperature_unit: Option<String>,
    pub wind_speed_unit: Option<String>,
    pub precipitation_unit: Option<String>,
    pub time_format: Option<String>,
    pub time_zone: Option<String>,
    pub past_days: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub current_weather: Option<bool>,
    pub hourly: Vec<String>,
    pub daily: Vec<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            location: location::Location::default(),
            temperature_unit: None,
            wind_speed_unit: None,
            precipitation_unit: None,
            time_format: None,
            time_zone: Some("CET".into()),
            past_days: None,
            start_date: None,
            end_date: None,
            current_weather: None,
            hourly: Vec::new(),
            daily: Vec::new(),
        }
    }
}

impl Options {
    pub fn as_params(self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        params.push(("latitude".into(), self.location.lat.to_string()));
        params.push(("longitude".into(), self.location.lng.to_string()));

        match self.temperature_unit {
            Some(v) => params.push(("temperature_unit".into(), v)),
            None => (),
        }

        match self.wind_speed_unit {
            Some(v) => params.push(("wind_speed_unit".into(), v)),
            None => (),
        }

        match self.precipitation_unit {
            Some(v) => params.push(("precipitation_unit".into(), v)),
            None => (),
        }

        match self.time_format {
            Some(v) => params.push(("time_format".into(), v)),
            None => (),
        }

        match self.time_zone {
            Some(v) => params.push(("time_zone".into(), v)),
            None => (),
        }

        match self.past_days {
            Some(v) => params.push(("past_days".into(), v)),
            None => (),
        }

        match self.start_date {
            Some(v) => params.push(("start_date".into(), v)),
            None => (),
        }

        match self.end_date {
            Some(v) => params.push(("end_date".into(), v)),
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

        params
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastResponse {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub generationtime_ms: Option<f64>,
    pub utc_offset_seconds: Option<f64>,
    pub timezone: Option<String>,
    pub timezone_abbreviation: Option<String>,
    pub elevation: Option<f64>,
    pub current_weather: Option<CurrentWeather>,
    pub hourly_units: Option<HashMap<String, serde_json::Value>>,
    pub hourly: Option<HashMap<String, serde_json::Value>>,
    pub daily_units: Option<HashMap<String, serde_json::Value>>,
    pub daily: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub temperature: Option<f64>,
    pub windspeed: Option<f64>,
    pub winddirection: Option<f64>,
    pub weathercode: Option<f64>,
    pub time: Option<String>,
}

impl client::Client {
    pub async fn forecast(&self, opts: Options) -> Result<ForecastResponse, Box<dyn Error>> {
        let mut forecast_endpoint = self.forecast_endpoint.to_owned();
        forecast_endpoint.push_str("/forecast");
        // forecast_endpoint.push_str("/foecast");
        let url = reqwest::Url::parse_with_params(&forecast_endpoint, opts.as_params())?;
        let res = self.http_client.get(url).send().await?;

        if res.status().is_success() {
            let res = res.json().await?;
            return Ok(res);
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
    use futures::join;

    #[tokio::test]
    async fn get_forecast() {
        let clt = Client::new();
        let mut opts = Options::default();
        opts.location = Location {
            lat: 48.864716,
            lng: 2.349014,
        };
        opts.hourly.push("temperature_2m".into());

        let res = clt.forecast(opts).await.unwrap();
        println!("{:?}", res);
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
