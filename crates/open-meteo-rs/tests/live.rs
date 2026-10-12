//! Smoke tests against the public API. Ignored by default so CI stays offline;
//! run them with `make test-live` (`cargo test -- --ignored`).

use chrono::{Duration, NaiveDate, Utc};
use open_meteo_rs::{
    air_quality, climate, elevation, ensemble, flood, forecast, geocoding, marine,
    satellite_radiation, seasonal, Client, Location, Model,
};

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

#[tokio::test]
#[ignore = "talks to the public API"]
async fn historical_forecast_and_previous_runs() {
    let client = Client::new();
    let today = Utc::now().date_naive();

    let historical = forecast::Options {
        hourly: vec!["temperature_2m".into()],
        start_date: Some(today - Duration::days(10)),
        end_date: Some(today - Duration::days(10)),
        ..Default::default()
    };
    let result = client.historical_forecast(historical).await.unwrap();
    assert_eq!(result.hourly.unwrap().len(), 24);

    let previous = forecast::Options {
        hourly: vec![
            "temperature_2m".into(),
            "temperature_2m_previous_day1".into(),
        ],
        forecast_days: Some(1),
        ..Default::default()
    };
    let result = client.previous_runs(previous).await.unwrap();
    let hourly = result.hourly.unwrap();
    assert!(hourly[0]
        .values
        .contains_key("temperature_2m_previous_day1"));
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn marine() {
    let options = marine::Options {
        location: Location {
            lat: 54.544_587,
            lng: 10.227_487,
        },
        current: vec!["wave_height".into()],
        hourly: vec!["wave_height".into()],
        daily: vec!["wave_height_max".into()],
        forecast_days: Some(1),
        ..Default::default()
    };

    let result = Client::new().marine(options).await.unwrap();
    assert!(result.current.is_some());
    assert_eq!(result.hourly.unwrap().len(), 24);
    assert_eq!(result.daily.unwrap().len(), 1);
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn ensemble() {
    let options = ensemble::Options {
        hourly: vec!["temperature_2m".into()],
        models: vec![Model::IconSeamless],
        forecast_days: Some(1),
        ..Default::default()
    };

    let result = Client::new().ensemble(options).await.unwrap();
    let hourly = result.hourly.unwrap();
    assert_eq!(hourly.len(), 24);
    assert!(hourly[0].values.contains_key("temperature_2m_member01"));
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn climate() {
    let options = climate::Options {
        start_date: NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2030, 1, 3).unwrap(),
        models: vec![Model::MRIAGCM32S],
        daily: vec!["temperature_2m_max".into()],
        ..Default::default()
    };

    let result = Client::new().climate(options).await.unwrap();
    assert_eq!(result.daily.unwrap().len(), 3);
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn flood() {
    let options = flood::Options {
        location: Location {
            lat: 59.91,
            lng: 10.75,
        },
        daily: vec!["river_discharge".into()],
        forecast_days: Some(3),
        ..Default::default()
    };

    let result = Client::new().flood(options).await.unwrap();
    assert_eq!(result.daily.unwrap().len(), 3);
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn satellite_radiation() {
    let options = satellite_radiation::Options {
        hourly: vec!["shortwave_radiation".into()],
        models: vec![Model::SatelliteRadiationSeamless],
        past_days: Some(1),
        forecast_days: Some(0),
        ..Default::default()
    };

    let result = Client::new().satellite_radiation(options).await.unwrap();
    assert_eq!(result.hourly.unwrap().len(), 24);
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn seasonal() {
    let options = seasonal::Options {
        daily: vec!["temperature_2m_max".into()],
        weekly: vec!["temperature_2m_mean".into()],
        forecast_days: Some(14),
        ..Default::default()
    };

    let result = Client::new().seasonal(options).await.unwrap();
    assert_eq!(result.daily.unwrap().len(), 14);
    assert!(!result.weekly.unwrap().is_empty());
}

#[tokio::test]
#[ignore = "talks to the public API"]
async fn elevation() {
    let options = elevation::Options {
        locations: vec![
            Location::default(),
            Location {
                lat: 48.85,
                lng: 2.35,
            },
        ],
        apikey: None,
    };

    let elevations = Client::new().elevation(options).await.unwrap();
    assert_eq!(elevations.len(), 2);
    assert!(elevations
        .iter()
        .all(|metres| (0.0..9000.0).contains(metres)));
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
