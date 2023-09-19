use std::time::Duration;

const DEFAULT_FORECAST_ENDPOINT: &str = "https://api.open-meteo.com/v1/";
const DEFAULT_GEOCODING_ENDPOINT: &str = "https://geocoding-api.open-meteo.com/v1/search";

const DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
const DEFAULT_TIMEOUT: Duration = Duration::from_millis(5000);
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_millis(2000);

#[derive(Default, Debug)]
pub struct Client {
    /// API endpoint
    pub forecast_endpoint: String,
    pub geocoding_endpoint: String,
    pub http_client: reqwest::Client,
}

impl Client {
    pub fn new() -> Client {
        let mut clt = Client::default();
        clt.forecast_endpoint = DEFAULT_FORECAST_ENDPOINT.into();
        clt.geocoding_endpoint = DEFAULT_GEOCODING_ENDPOINT.into();

        clt.http_client = reqwest::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .unwrap();

        clt
    }

    pub fn with_forecast_endpoint(mut self, endpoint: String) -> Client {
        self.forecast_endpoint = endpoint;
        self
    }

    #[deprecated(note="this method contains a typo; please use `with_geocoding_endpoint` instead")]
    pub fn with_geowoding_endpoint(mut self, endpoint: String) -> Client {
        self.with_geocoding_endpoint(endpoint)
    }

    pub fn with_geocoding_endpoint(mut self, endpoint: String) -> Client {
        self.geocoding_endpoint = endpoint;
        self
    }

    pub fn with_reqwest_client(mut self, client: reqwest::Client) -> Client {
        self.http_client = client;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_correct_default() {
        let clt = Client::new();
        assert_eq!(clt.forecast_endpoint, DEFAULT_FORECAST_ENDPOINT);
    }

    #[test]
    fn set_forecast_endpoint() {
        let endpoint = String::from("http://some.where");
        let clt = Client::new().with_forecast_endpoint(endpoint.clone());
        assert_eq!(clt.forecast_endpoint, endpoint);
    }
}
