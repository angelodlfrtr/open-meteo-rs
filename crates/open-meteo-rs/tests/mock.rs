//! Both clients against a local HTTP server that replays the recorded fixtures.
//!
//! This covers everything the live tests cover, offline: URL building against custom
//! endpoints, status handling, error mapping and each endpoint method.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use chrono::NaiveDate;
use open_meteo_rs::{
    air_quality, climate, elevation, ensemble, flood, forecast, geocoding, marine,
    satellite_radiation, seasonal, Client, Endpoints, Error, Location, Model,
};

macro_rules! fixture {
    ($name:literal) => {
        include_bytes!(concat!(
            "../../open-meteo-core/tests/fixtures/",
            $name,
            ".json"
        ))
        .as_slice()
    };
}

/// A canned response per path, served by a background thread.
struct Server {
    base: String,
}

/// Path of a route that announces a body and then closes the connection early.
const TRUNCATED: &str = "/truncated";

impl Server {
    fn start(routes: &[(&'static str, u16, &'static [u8])]) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let routes: Arc<HashMap<&str, (u16, &[u8])>> = Arc::new(
            routes
                .iter()
                .map(|(path, status, body)| (*path, (*status, *body)))
                .collect(),
        );

        thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let routes = Arc::clone(&routes);
                thread::spawn(move || respond(stream, &routes));
            }
        });

        Self { base }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{path}", self.base)
    }

    /// Every endpoint pointed at this server, each under its own path.
    fn endpoints(&self) -> Endpoints {
        Endpoints {
            forecast: self.url("/forecast"),
            archive: self.url("/archive"),
            historical_forecast: self.url("/historical_forecast"),
            previous_runs: self.url("/previous_runs"),
            ensemble: self.url("/ensemble"),
            climate: self.url("/climate"),
            marine: self.url("/marine"),
            air_quality: self.url("/air_quality"),
            satellite_radiation: self.url("/satellite_radiation"),
            seasonal: self.url("/seasonal"),
            flood: self.url("/flood"),
            elevation: self.url("/elevation"),
            geocoding_search: self.url("/geocoding_search"),
            geocoding_get: self.url("/geocoding_get"),
        }
    }
}

fn respond(mut stream: TcpStream, routes: &HashMap<&str, (u16, &[u8])>) {
    let mut request = Vec::new();
    let mut buffer = [0u8; 1024];
    while !request.windows(4).any(|w| w == b"\r\n\r\n") {
        match stream.read(&mut buffer) {
            Ok(0) | Err(_) => return,
            Ok(n) => request.extend_from_slice(&buffer[..n]),
        }
    }

    let request = String::from_utf8_lossy(&request);
    let target = request.split_whitespace().nth(1).unwrap_or("/");
    let path = target.split('?').next().unwrap_or("/");

    if path == TRUNCATED {
        let _ = stream.write_all(
            b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 1000\r\nConnection: close\r\n\r\n{",
        );
        return;
    }

    let (status, body) = routes
        .get(path)
        .copied()
        .unwrap_or((404, br#"{"error":true,"reason":"no such route"}"#));
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        _ => "Error",
    };

    let _ = write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(body);
}

/// Every endpoint, answered with its recorded fixture.
fn all_endpoints() -> Server {
    Server::start(&[
        ("/forecast", 200, fixture!("forecast")),
        ("/archive", 200, fixture!("archive")),
        ("/historical_forecast", 200, fixture!("historical_forecast")),
        ("/previous_runs", 200, fixture!("previous_runs")),
        ("/ensemble", 200, fixture!("ensemble")),
        ("/climate", 200, fixture!("climate")),
        ("/marine", 200, fixture!("marine")),
        ("/air_quality", 200, fixture!("air_quality")),
        ("/satellite_radiation", 200, fixture!("satellite_radiation")),
        ("/seasonal", 200, fixture!("seasonal")),
        ("/flood", 200, fixture!("flood")),
        ("/elevation", 200, fixture!("elevation")),
        ("/geocoding_search", 200, fixture!("geocoding_search")),
        ("/geocoding_get", 200, fixture!("geocoding_get")),
    ])
}

fn climate_options() -> climate::Options {
    climate::Options {
        start_date: NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2030, 1, 3).unwrap(),
        models: vec![Model::MRIAGCM32S],
        daily: vec!["temperature_2m_max".into()],
        ..Default::default()
    }
}

#[tokio::test]
async fn every_async_method_reaches_its_endpoint() {
    let server = all_endpoints();
    let client = Client::new().with_endpoints(server.endpoints());
    let opts = forecast::Options::default();

    assert_eq!(
        client
            .forecast(opts.clone())
            .await
            .unwrap()
            .hourly
            .unwrap()
            .len(),
        24
    );
    assert_eq!(
        client
            .archive(opts.clone())
            .await
            .unwrap()
            .hourly
            .unwrap()
            .len(),
        48
    );
    assert_eq!(
        client
            .historical_forecast(opts.clone())
            .await
            .unwrap()
            .hourly
            .unwrap()
            .len(),
        48
    );
    assert_eq!(
        client
            .previous_runs(opts)
            .await
            .unwrap()
            .hourly
            .unwrap()
            .len(),
        24
    );
    assert_eq!(
        client
            .ensemble(ensemble::Options::default())
            .await
            .unwrap()
            .hourly
            .unwrap()
            .len(),
        24
    );
    assert_eq!(
        client
            .climate(climate_options())
            .await
            .unwrap()
            .daily
            .unwrap()
            .len(),
        3
    );
    assert_eq!(
        client
            .marine(marine::Options::default())
            .await
            .unwrap()
            .hourly
            .unwrap()
            .len(),
        24
    );
    assert_eq!(
        client
            .air_quality(air_quality::Options::default())
            .await
            .unwrap()
            .hourly
            .unwrap()
            .len(),
        24
    );
    assert_eq!(
        client
            .satellite_radiation(satellite_radiation::Options::default())
            .await
            .unwrap()
            .hourly
            .unwrap()
            .len(),
        24
    );
    assert_eq!(
        client
            .seasonal(seasonal::Options::default())
            .await
            .unwrap()
            .daily
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        client
            .flood(flood::Options::default())
            .await
            .unwrap()
            .daily
            .unwrap()
            .len(),
        3
    );
    assert_eq!(
        client
            .elevation(elevation::Options::default())
            .await
            .unwrap(),
        vec![38.0, 46.0]
    );
    assert_eq!(
        client
            .geocoding(geocoding::Options::default())
            .await
            .unwrap()
            .results
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        client
            .geocoding_get(geocoding::GetOptions::default())
            .await
            .unwrap()
            .name
            .as_deref(),
        Some("Berlin")
    );
}

#[tokio::test]
async fn api_error_status_becomes_error_api() {
    let server = Server::start(&[("/forecast", 400, fixture!("error_400"))]);
    let client = Client::new().with_endpoints(server.endpoints());

    let error = client
        .forecast(forecast::Options::default())
        .await
        .unwrap_err();
    match error {
        Error::Api(open_meteo_core::Error::Api { status, reason }) => {
            assert_eq!(status, 400);
            assert!(reason.starts_with("Latitude must be in range"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn undecodable_body_becomes_error_api() {
    let server = Server::start(&[("/forecast", 200, b"<html>not json</html>")]);
    let client = Client::new().with_endpoints(server.endpoints());

    let error = client
        .forecast(forecast::Options::default())
        .await
        .unwrap_err();
    assert!(
        matches!(error, Error::Api(open_meteo_core::Error::Json(_))),
        "{error:?}"
    );
    assert!(error
        .to_string()
        .starts_with("cannot decode the API response"));
}

#[tokio::test]
async fn unreachable_host_becomes_error_http() {
    // A listener that is dropped at once leaves a port nothing listens on.
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let endpoints = Endpoints {
        forecast: format!("http://127.0.0.1:{port}/forecast"),
        ..Default::default()
    };
    let client = Client::new().with_endpoints(endpoints);

    let error = client
        .forecast(forecast::Options::default())
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Http(_)), "{error:?}");
}

#[tokio::test]
async fn truncated_body_becomes_error_http() {
    let server = Server::start(&[]);
    let endpoints = Endpoints {
        forecast: server.url(TRUNCATED),
        ..Default::default()
    };
    let client = Client::new().with_endpoints(endpoints);

    let error = client
        .forecast(forecast::Options::default())
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Http(_)), "{error:?}");
}

#[tokio::test]
async fn custom_reqwest_client_is_used() {
    let server = all_endpoints();
    let http_client = reqwest::Client::builder()
        .user_agent("custom-agent")
        .build()
        .unwrap();
    let client = Client::new()
        .with_endpoints(server.endpoints())
        .with_reqwest_client(http_client);

    assert!(client
        .elevation(elevation::Options::single(Location::default()))
        .await
        .is_ok());
}

#[tokio::test]
async fn send_reaches_an_arbitrary_base_url() {
    let server = Server::start(&[("/anywhere", 200, fixture!("elevation"))]);
    let client = Client::new();

    let elevations = client
        .send(&server.url("/anywhere"), &elevation::Options::default())
        .await
        .unwrap();
    assert_eq!(elevations, vec![38.0, 46.0]);
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use open_meteo_rs::blocking::Client;

    #[test]
    fn every_blocking_method_reaches_its_endpoint() {
        let server = all_endpoints();
        let client = Client::new().with_endpoints(server.endpoints());
        let opts = forecast::Options::default();

        assert_eq!(
            client.forecast(opts.clone()).unwrap().hourly.unwrap().len(),
            24
        );
        assert_eq!(
            client.archive(opts.clone()).unwrap().hourly.unwrap().len(),
            48
        );
        assert_eq!(
            client
                .historical_forecast(opts.clone())
                .unwrap()
                .hourly
                .unwrap()
                .len(),
            48
        );
        assert_eq!(
            client.previous_runs(opts).unwrap().hourly.unwrap().len(),
            24
        );
        assert_eq!(
            client
                .ensemble(ensemble::Options::default())
                .unwrap()
                .hourly
                .unwrap()
                .len(),
            24
        );
        assert_eq!(
            client
                .climate(climate_options())
                .unwrap()
                .daily
                .unwrap()
                .len(),
            3
        );
        assert_eq!(
            client
                .marine(marine::Options::default())
                .unwrap()
                .hourly
                .unwrap()
                .len(),
            24
        );
        assert_eq!(
            client
                .air_quality(air_quality::Options::default())
                .unwrap()
                .hourly
                .unwrap()
                .len(),
            24
        );
        assert_eq!(
            client
                .satellite_radiation(satellite_radiation::Options::default())
                .unwrap()
                .hourly
                .unwrap()
                .len(),
            24
        );
        assert_eq!(
            client
                .seasonal(seasonal::Options::default())
                .unwrap()
                .daily
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            client
                .flood(flood::Options::default())
                .unwrap()
                .daily
                .unwrap()
                .len(),
            3
        );
        assert_eq!(
            client.elevation(elevation::Options::default()).unwrap(),
            vec![38.0, 46.0]
        );
        assert_eq!(
            client
                .geocoding(geocoding::Options::default())
                .unwrap()
                .results
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            client
                .geocoding_get(geocoding::GetOptions::default())
                .unwrap()
                .name
                .as_deref(),
            Some("Berlin")
        );
    }

    #[test]
    fn errors_are_mapped_like_the_async_client() {
        let server = Server::start(&[("/forecast", 400, fixture!("error_400"))]);
        let client = Client::new().with_endpoints(server.endpoints());
        let error = client.forecast(forecast::Options::default()).unwrap_err();
        assert!(matches!(
            error,
            Error::Api(open_meteo_core::Error::Api { status: 400, .. })
        ));

        let port = TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let endpoints = Endpoints {
            forecast: format!("http://127.0.0.1:{port}/forecast"),
            ..Default::default()
        };
        let client = Client::new().with_endpoints(endpoints);
        assert!(matches!(
            client.forecast(forecast::Options::default()),
            Err(Error::Http(_))
        ));

        let server = Server::start(&[("/forecast", 200, b"<html>")]);
        let client = Client::new().with_endpoints(server.endpoints());
        assert!(matches!(
            client.forecast(forecast::Options::default()),
            Err(Error::Api(open_meteo_core::Error::Json(_)))
        ));

        let endpoints = Endpoints {
            forecast: server.url(TRUNCATED),
            ..Default::default()
        };
        let client = Client::new().with_endpoints(endpoints);
        assert!(matches!(
            client.forecast(forecast::Options::default()),
            Err(Error::Http(_))
        ));
    }

    #[test]
    fn custom_reqwest_client_is_used() {
        let server = all_endpoints();
        let http_client = reqwest::blocking::Client::builder()
            .user_agent("custom-agent")
            .build()
            .unwrap();
        let client = Client::new()
            .with_endpoints(server.endpoints())
            .with_reqwest_client(http_client);
        assert!(client.elevation(elevation::Options::default()).is_ok());
    }
}
