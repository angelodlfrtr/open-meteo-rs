use super::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub name: Option<String>,
    pub language: Option<String>,
    pub count: Option<u16>,
}

impl Options {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    pub fn with_count(mut self, count: u16) -> Self {
        self.count = Some(count);
        self
    }

    fn as_params(self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        match self.name {
            Some(v) => params.push(("name".into(), v)),
            None => (),
        }

        match self.language {
            Some(v) => params.push(("language".into(), v)),
            None => (),
        }

        match self.count {
            Some(v) => params.push(("count".into(), v.to_string())),
            None => (),
        }

        params
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeocodingResponse {
    pub results: Option<Vec<GeocodingResult>>,
    pub generationtime_ms: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeocodingResult {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation: Option<f64>,
    pub feature_code: Option<String>,
    pub country_code: Option<String>,
    pub admin1_id: Option<i64>,
    pub admin3_id: Option<i64>,
    pub admin4_id: Option<i64>,
    pub timezone: Option<String>,
    pub population: Option<i64>,
    pub postcodes: Option<Vec<String>>,
    pub country_id: Option<i64>,
    pub country: Option<String>,
    pub admin1: Option<String>,
    pub admin3: Option<String>,
    pub admin4: Option<String>,
    pub admin2_id: Option<i64>,
    pub admin2: Option<String>,
}

impl client::Client {
    pub async fn geocoding(&self, opts: Options) -> Result<GeocodingResponse, Box<dyn Error>> {
        let url = reqwest::Url::parse_with_params(&self.geocoding_endpoint, opts.as_params())?;
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

    #[tokio::test]
    async fn search() {
        let clt = Client::new();
        let opts = Options::default().with_name("Paris".into());
        let res = clt.geocoding(opts).await.unwrap();
        println!("{:?}", res);
        assert!(res.results.unwrap().len() > 0);
    }
}
