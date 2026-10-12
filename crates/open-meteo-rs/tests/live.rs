//! Smoke tests against the public API. Ignored by default so CI stays offline;
//! run them with `make test-live` (`cargo test -- --ignored`).

use chrono::{Duration, Utc};
use open_meteo_rs::{air_quality, forecast, geocoding, Client, Location};

fn berlin_forecast() -> forecast::Options {
    forecast::Options {
        location: Location {
            lat: 52.52,
            lng: 13.41,
        },
        current: vec!["temperature_2m".into()],
        minutely_15: vec!["temperature_2m".into()],
        hourly: vec!["temperature_2m".into()],
        daily: vec!["temperature_2m_max".into()],
        time_zone: Some("Europe/Berlin".into()),
        forecast_days: Some(1),
        ..Default::default()
    }
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn forecast() {
    let result = Client::new().forecast(berlin_forecast()).await.unwrap();

    assert!(result.current.is_some());
    assert_eq!(result.minutely_15.unwrap().len(), 96);
    assert_eq!(result.hourly.unwrap().len(), 24);
    assert_eq!(result.daily.unwrap().len(), 1);
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn forecast_requests_can_run_concurrently() {
    let client = Client::new();
    let (one, two) = futures::join!(
        client.forecast(berlin_forecast()),
        client.forecast(berlin_forecast())
    );

    assert!(one.is_ok() && two.is_ok());
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn archive() {
    let today = Utc::now().date_naive();
    let options = forecast::Options {
        hourly: vec!["temperature_2m".into()],
        start_date: Some(today - Duration::days(10)),
        end_date: Some(today - Duration::days(9)),
        ..Default::default()
    };

    let result = Client::new().archive(options).await.unwrap();
    assert_eq!(result.hourly.unwrap().len(), 48);
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn air_quality() {
    let options = air_quality::Options {
        current: vec!["european_aqi".into()],
        hourly: vec!["pm10".into()],
        forecast_days: Some(1),
        ..Default::default()
    };

    let result = Client::new().air_quality(options).await.unwrap();
    assert!(result.current.is_some());
    assert_eq!(result.hourly.unwrap().len(), 24);
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn geocoding_search_then_get() {
    let client = Client::new();

    let search = geocoding::Options::default()
        .with_name("Paris".into())
        .with_country_code("FR".into());
    let results = client.geocoding(search).await.unwrap().results.unwrap();
    assert!(!results.is_empty());

    let id = results[0].id.unwrap();
    let place = client
        .geocoding_get(geocoding::GetOptions {
            id,
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(place.id, Some(id));
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn api_errors_are_reported() {
    let options = forecast::Options {
        location: Location {
            lat: 999.0,
            lng: 13.41,
        },
        ..Default::default()
    };

    let error = Client::new().forecast(options).await.unwrap_err();
    assert!(matches!(
        error,
        open_meteo_rs::Error::Api(open_meteo_core::Error::Api { status: 400, .. })
    ));
}

#[cfg(feature = "blocking")]
mod blocking {
    use open_meteo_rs::blocking::Client;

    #[test]
    #[ignore = "talks to the public API"]
    fn forecast() {
        let result = Client::new().forecast(super::berlin_forecast()).unwrap();
        assert_eq!(result.hourly.unwrap().len(), 24);
    }
}
