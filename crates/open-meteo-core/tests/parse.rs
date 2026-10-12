//! Parsing of real API answers, recorded in `tests/fixtures`.
//!
//! Each fixture was captured once from the public API with `timeformat=unixtime`. The tests
//! pin the values a user would read, so a change in parsing shows up as a precise diff.

use chrono::NaiveDate;
use open_meteo_core::{
    air_quality, climate, elevation, ensemble, flood, forecast, geocoding, marine,
    satellite_radiation, seasonal, Error, Request,
};
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

#[test]
fn marine_current_hourly_and_daily() {
    let body = include_bytes!("fixtures/marine.json");
    let result = marine::Options::parse(body).unwrap();

    let current = result.current.unwrap();
    assert_eq!(current.datetime, local(2026, 10, 12, 7, 45));
    assert_eq!(current.values["wave_height"].value, json!(0.3));
    assert_eq!(current.values["wave_height"].unit.as_deref(), Some("m"));

    let hourly = result.hourly.unwrap();
    assert_eq!(hourly.len(), 24);
    assert_eq!(hourly[0].datetime, local(2026, 10, 12, 0, 0));
    assert_eq!(hourly[0].values["wave_height"].value, json!(0.5));
    assert_eq!(hourly[0].values["wave_direction"].value, json!(331));

    let daily = result.daily.unwrap();
    assert_eq!(daily.len(), 1);
    assert_eq!(daily[0].values["wave_height_max"].value, json!(0.5));
}

#[test]
fn historical_forecast_is_forecast_shaped() {
    let body = include_bytes!("fixtures/historical_forecast.json");
    let result = forecast::Options::parse(body).unwrap();

    let hourly = result.hourly.unwrap();
    assert_eq!(hourly.len(), 48);
    assert_eq!(hourly[0].datetime, local(2026, 9, 23, 0, 0));
    assert_eq!(hourly[0].values["temperature_2m"].value, json!(10.3));
}

#[test]
fn previous_runs_keep_lead_time_columns() {
    let body = include_bytes!("fixtures/previous_runs.json");
    let result = forecast::Options::parse(body).unwrap();

    let hourly = result.hourly.unwrap();
    assert_eq!(hourly.len(), 24);
    assert_eq!(hourly[0].values["temperature_2m"].value, json!(9.6));
    assert_eq!(
        hourly[0].values["temperature_2m_previous_day1"].value,
        json!(9.5)
    );
}

#[test]
fn ensemble_members_are_separate_variables() {
    let body = include_bytes!("fixtures/ensemble.json");
    let result = ensemble::Options::parse(body).unwrap();

    let hourly = result.hourly.unwrap();
    assert_eq!(hourly.len(), 24);
    // control run + 40 members, `time` excluded
    assert_eq!(hourly[0].values.len(), 40);
    assert_eq!(hourly[0].values["temperature_2m"].value, json!(9.6));
    assert_eq!(
        hourly[0].values["temperature_2m_member01"].value,
        json!(8.8)
    );
}

#[test]
fn climate_daily_per_model() {
    let body = include_bytes!("fixtures/climate.json");
    let result = climate::Options::parse(body).unwrap();

    let daily = result.daily.unwrap();
    assert_eq!(daily.len(), 3);
    assert_eq!(daily[0].date, NaiveDate::from_ymd_opt(2030, 1, 1).unwrap());
    // With several models, each variable is suffixed with the model name.
    let mri = &daily[0].values["temperature_2m_max_MRI_AGCM3_2_S"];
    assert_eq!(mri.value, json!(2.2));
    assert_eq!(mri.unit.as_deref(), Some("°C"));
    let ec_earth = &daily[0].values["temperature_2m_max_EC_Earth3P_HR"];
    assert_eq!(ec_earth.value, json!(1.7));
}

#[test]
fn flood_daily_discharge() {
    let body = include_bytes!("fixtures/flood.json");
    let result = flood::Options::parse(body).unwrap();

    let daily = result.daily.unwrap();
    assert_eq!(daily.len(), 3);
    assert_eq!(
        daily[0].date,
        NaiveDate::from_ymd_opt(2026, 10, 12).unwrap()
    );
    assert_eq!(daily[0].values["river_discharge"].value, json!(8.08));
    assert_eq!(
        daily[0].values["river_discharge"].unit.as_deref(),
        Some("m³/s")
    );
}

#[test]
fn satellite_radiation_keeps_nulls_for_missing_hours() {
    let body = include_bytes!("fixtures/satellite_radiation.json");
    let result = satellite_radiation::Options::parse(body).unwrap();

    let hourly = result.hourly.unwrap();
    assert_eq!(hourly.len(), 24);
    assert_eq!(hourly[0].datetime, local(2026, 10, 11, 0, 0));
    assert_eq!(hourly[0].values["shortwave_radiation"].value, json!(0.0));
    assert_eq!(hourly[8].values["shortwave_radiation"].value, json!(128.2));
    assert_eq!(
        hourly[0].values["shortwave_radiation"].unit.as_deref(),
        Some("W/m²")
    );
    // Hours not yet observed are null, not dropped.
    assert_eq!(hourly[23].values["shortwave_radiation"].value, json!(null));
}

#[test]
fn seasonal_daily_weekly_and_monthly() {
    let body = include_bytes!("fixtures/seasonal.json");
    let result = seasonal::Options::parse(body).unwrap();

    // Not requested with a resolution the models provide: absent, not an error.
    assert!(result.six_hourly.is_none());

    let daily = result.daily.unwrap();
    assert_eq!(daily.len(), 2);
    assert_eq!(
        daily[0].date,
        NaiveDate::from_ymd_opt(2026, 10, 12).unwrap()
    );
    assert_eq!(daily[0].values["temperature_2m_max"].value, json!(12.9));
    // Members ride along as suffixed variables: control + 50 members.
    assert_eq!(daily[0].values.len(), 51);

    let weekly = result.weekly.unwrap();
    assert_eq!(weekly.len(), 1);
    assert_eq!(
        weekly[0].date,
        NaiveDate::from_ymd_opt(2026, 10, 12).unwrap()
    );
    assert_eq!(weekly[0].values["temperature_2m_mean"].value, json!(10.2));
    assert_eq!(
        weekly[0].values["temperature_2m_mean"].unit.as_deref(),
        Some("°C")
    );

    // A section the API returned with empty columns is an empty list, not `None`.
    assert_eq!(result.monthly.unwrap().len(), 0);
}

#[test]
fn elevation_is_one_value_per_location() {
    let body = include_bytes!("fixtures/elevation.json");
    assert_eq!(elevation::Options::parse(body).unwrap(), vec![38.0, 46.0]);
}

/// A body where `section` is present but has no `time`, so parsing that section fails.
fn malformed(section: &str) -> Vec<u8> {
    format!(r#"{{"{section}": {{"temperature_2m": [1.0]}}}}"#).into_bytes()
}

/// Runs `parse` on garbage and on a body with each of `sections` malformed in turn,
/// so every section's error path is exercised.
fn rejects_bad_bodies<T: std::fmt::Debug>(parse: fn(&[u8]) -> Result<T, Error>, sections: &[&str]) {
    assert!(matches!(parse(b"<html>"), Err(Error::Json(_))));
    for section in sections {
        assert!(
            matches!(parse(&malformed(section)), Err(Error::Malformed(_))),
            "{section} should be rejected"
        );
    }
}

#[test]
fn every_endpoint_rejects_malformed_bodies() {
    rejects_bad_bodies(
        forecast::Options::parse,
        &["current", "minutely_15", "hourly", "daily"],
    );
    rejects_bad_bodies(air_quality::Options::parse, &["current", "hourly"]);
    rejects_bad_bodies(marine::Options::parse, &["current", "hourly", "daily"]);
    rejects_bad_bodies(ensemble::Options::parse, &["hourly"]);
    rejects_bad_bodies(climate::Options::parse, &["daily"]);
    rejects_bad_bodies(flood::Options::parse, &["daily"]);
    rejects_bad_bodies(satellite_radiation::Options::parse, &["hourly"]);
    rejects_bad_bodies(
        seasonal::Options::parse,
        &["six_hourly", "daily", "weekly", "monthly"],
    );
    rejects_bad_bodies(elevation::Options::parse, &[]);
    rejects_bad_bodies(geocoding::Options::parse, &[]);
    rejects_bad_bodies(geocoding::GetOptions::parse, &[]);
}
