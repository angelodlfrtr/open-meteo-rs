use std::time::Duration;

const DEFAULT_ENDPOINT: &str = "https://api.open-meteo.com/v1/";
const DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
const DEFAULT_TIMEOUT: Duration = Duration::from_millis(5000);
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_millis(2000);

#[derive(Default, Debug)]
pub struct Client {
    pub endpoint: String,
    pub http_client: reqwest::Client,
}

impl Client {
    pub fn new() -> Client {
        let mut clt = Client::default();
        clt.endpoint = DEFAULT_ENDPOINT.into();

        clt.http_client = reqwest::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .unwrap();

        clt
    }

    pub fn with_endpoint(mut self, endpoint: String) -> Client {
        self.endpoint = endpoint;
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
        assert_eq!(clt.endpoint, DEFAULT_ENDPOINT);
    }

    #[test]
    fn set_endpoint() {
        let endpoint = String::from("http://some.where");
        let clt = Client::new().with_endpoint(endpoint.clone());
        assert_eq!(clt.endpoint, endpoint);
    }
}
