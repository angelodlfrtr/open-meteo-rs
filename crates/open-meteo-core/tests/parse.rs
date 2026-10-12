//! Parsing of real API answers, recorded in `tests/fixtures`.
//!
//! Each fixture was captured once from the public API with `timeformat=unixtime`. The tests
//! pin the values a user would read, so a change in parsing shows up as a precise diff.

use chrono::NaiveDate;
use open_meteo_core::{air_quality, forecast, geocoding, Error, Request};
use serde_json::json;

fn local(y: i32, m: u32, d: u32, h: u32, min: u32) -> chrono::NaiveDateTime {
    NaiveDate::from_ymd_opt(y, m, d)
        .unwrap()
        .and_hms_opt(h, min, 0)
        .unwrap()
}

#[test]
fn forecast_with_every_section() {
    // Berlin, UTC+2: hourly series start at local midnight, current is 09:45 local.
    let body = include_bytes!("fixtures/forecast.json");
    let result = forecast::Options::parse(body).unwrap();

    let current = result.current.unwrap();
    assert_eq!(current.datetime, local(2026, 10, 12, 9, 45));
    assert_eq!(current.values["temperature_2m"].value, json!(7.9));
    assert_eq!(current.values["temperature_2m"].unit.as_deref(), Some("°C"));
    assert!(current.values.contains_key("wind_speed_10m"));

    let minutely_15 = result.minutely_15.unwrap();
    assert_eq!(minutely_15.len(), 96);
    assert_eq!(minutely_15[0].datetime, local(2026, 10, 12, 0, 0));
    assert_eq!(minutely_15[1].datetime, local(2026, 10, 12, 0, 15));

    let hourly = result.hourly.unwrap();
    assert_eq!(hourly.len(), 24);
    assert_eq!(hourly[0].datetime, local(2026, 10, 12, 0, 0));
    assert_eq!(hourly[0].values["temperature_2m"].value, json!(10.0));
    assert_eq!(
        hourly[0].values["relative_humidity_2m"].unit.as_deref(),
        Some("%")
    );

    let daily = result.daily.unwrap();
    assert_eq!(daily.len(), 1);
    assert_eq!(
        daily[0].date,
        NaiveDate::from_ymd_opt(2026, 10, 12).unwrap()
    );
    assert_eq!(daily[0].values["temperature_2m_max"].value, json!(13.6));
    // Variables that are themselves times stay raw; the caller knows what they asked for.
    assert_eq!(daily[0].values["sunrise"].value, json!(1_791_782_776));
}

#[test]
fn archive_without_current() {
    let body = include_bytes!("fixtures/archive.json");
    let result = forecast::Options::parse(body).unwrap();

    assert!(result.current.is_none());
    assert!(result.minutely_15.is_none());

    let hourly = result.hourly.unwrap();
    assert_eq!(hourly.len(), 48);
    assert_eq!(hourly[0].datetime, local(2023, 5, 1, 0, 0));
    assert_eq!(hourly[0].values["temperature_2m"].value, json!(4.6));

    let daily = result.daily.unwrap();
    assert_eq!(daily.len(), 2);
    assert_eq!(daily[0].values["temperature_2m_max"].value, json!(17.4));
}

#[test]
fn air_quality_current_and_hourly() {
    let body = include_bytes!("fixtures/air_quality.json");
    let result = air_quality::Options::parse(body).unwrap();

    let current = result.current.unwrap();
    assert_eq!(current.values["european_aqi"].value, json!(27));
    assert_eq!(current.values["european_aqi"].unit.as_deref(), Some("EAQI"));

    let hourly = result.hourly.unwrap();
    assert_eq!(hourly.len(), 24);
    assert_eq!(hourly[0].values["pm10"].value, json!(5.9));
    assert_eq!(hourly[0].values["pm10"].unit.as_deref(), Some("μg/m³"));
    assert_eq!(hourly[0].values["ozone"].value, json!(42.0));
}

#[test]
fn geocoding_search_results() {
    let body = include_bytes!("fixtures/geocoding_search.json");
    let response = geocoding::Options::parse(body).unwrap();

    let results = response.results.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].id, Some(2_950_159));
    assert_eq!(results[0].name.as_deref(), Some("Berlin"));
    assert_eq!(results[0].country_code.as_deref(), Some("DE"));
    assert_eq!(results[0].timezone.as_deref(), Some("Europe/Berlin"));
    assert_eq!(results[0].postcodes.as_ref().map(Vec::len), Some(2));
}

#[test]
fn geocoding_get_single_place() {
    let body = include_bytes!("fixtures/geocoding_get.json");
    let place = geocoding::GetOptions::parse(body).unwrap();

    assert_eq!(place.id, Some(2_950_159));
    assert_eq!(place.name.as_deref(), Some("Berlin"));
    assert_eq!(place.country.as_deref(), Some("Germany"));
    assert_eq!(place.admin1.as_deref(), Some("State of Berlin"));
    assert_eq!(place.population, Some(3_426_354));
}

#[test]
fn api_error_bodies_become_readable_errors() {
    let body = include_bytes!("fixtures/error_400.json");
    let error = Error::from_status(400, body);

    assert!(matches!(
        &error,
        Error::Api { status: 400, reason } if reason.starts_with("Latitude must be in range")
    ));
    assert_eq!(
        error.to_string(),
        "the API returned status 400: Latitude must be in range of -90 to 90°. Given: 999.0."
    );

    // A body that is not the usual error JSON is kept verbatim.
    let error = Error::from_status(502, b"Bad Gateway");
    assert!(matches!(error, Error::Api { status: 502, reason } if reason == "Bad Gateway"));
}

#[test]
fn garbage_is_a_json_error() {
    assert!(matches!(
        forecast::Options::parse(b"<html>"),
        Err(Error::Json(_))
    ));
}
