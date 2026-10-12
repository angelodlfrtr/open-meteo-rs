//! The exact URLs each request produces.

use chrono::NaiveDate;
use open_meteo_core::{air_quality, forecast, geocoding, url, Endpoints, Location, Model};

fn date(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

#[test]
fn forecast_minimal() {
    let options = forecast::Options {
        hourly: vec!["temperature_2m".into()],
        ..Default::default()
    };

    assert_eq!(
        url(&Endpoints::default().forecast, &options),
        "https://api.open-meteo.com/v1/forecast\
         ?latitude=52.52&longitude=13.41&timeformat=unixtime&hourly=temperature_2m"
    );
}

#[test]
fn forecast_every_option() {
    let options = forecast::Options {
        location: Location {
            lat: 48.864_716,
            lng: 2.349_014,
        },
        elevation: Some(forecast::Elevation::Nan),
        minutely_15: vec!["temperature_2m".into()],
        hourly: vec!["temperature_2m".into(), "snowfall".into()],
        daily: vec!["temperature_2m_max".into()],
        current: vec!["temperature_2m".into()],
        temperature_unit: Some(forecast::TemperatureUnit::Fahrenheit),
        wind_speed_unit: Some(forecast::WindSpeedUnit::Ms),
        precipitation_unit: Some(forecast::PrecipitationUnit::Inches),
        time_zone: Some("Europe/Paris".into()),
        past_days: Some(1),
        forecast_days: Some(3),
        past_hours: Some(6),
        forecast_hours: Some(12),
        forecast_minutely_15: Some(96),
        start_date: Some(date(2024, 1, 1)),
        end_date: Some(date(2024, 1, 2)),
        start_hour: Some(date(2024, 1, 1).and_hms_opt(6, 0, 0).unwrap()),
        end_hour: Some(date(2024, 1, 1).and_hms_opt(18, 30, 0).unwrap()),
        models: vec![Model::IconSeamless, Model::GfsGlobal],
        cell_selection: Some(forecast::CellSelection::Sea),
        apikey: Some("secret".into()),
    };

    assert_eq!(
        url("https://example.test/v1/forecast", &options),
        "https://example.test/v1/forecast\
         ?latitude=48.864716&longitude=2.349014&timeformat=unixtime&elevation=nan\
         &temperature_unit=fahrenheit&wind_speed_unit=ms&precipitation_unit=inch\
         &timezone=Europe/Paris&past_days=1&forecast_days=3&past_hours=6&forecast_hours=12\
         &forecast_minutely_15=96&start_date=2024-01-01&end_date=2024-01-02\
         &start_hour=2024-01-01T06:00&end_hour=2024-01-01T18:30\
         &current=temperature_2m&minutely_15=temperature_2m&hourly=temperature_2m,snowfall\
         &daily=temperature_2m_max&models=icon_seamless,gfs_global&cell_selection=sea\
         &apikey=secret"
    );
}

#[test]
fn air_quality() {
    let options = air_quality::Options {
        current: vec!["european_aqi".into()],
        hourly: vec!["pm10".into(), "ozone".into()],
        domains: Some("cams_europe".into()),
        forecast_days: Some(2),
        ..Default::default()
    };

    assert_eq!(
        url(&Endpoints::default().air_quality, &options),
        "https://air-quality-api.open-meteo.com/v1/air-quality\
         ?latitude=52.52&longitude=13.41&timeformat=unixtime&current=european_aqi\
         &hourly=pm10,ozone&domains=cams_europe&forecast_days=2"
    );
}

#[test]
fn geocoding_search_and_get() {
    let search = geocoding::Options::default()
        .with_name("Saint Étienne".into())
        .with_language("fr".into())
        .with_count(5)
        .with_country_code("FR".into());

    assert_eq!(
        url(&Endpoints::default().geocoding_search, &search),
        "https://geocoding-api.open-meteo.com/v1/search\
         ?name=Saint%20%C3%89tienne&language=fr&count=5&countryCode=FR"
    );

    let get = geocoding::GetOptions {
        id: 2_950_159,
        ..Default::default()
    };
    assert_eq!(
        url(&Endpoints::default().geocoding_get, &get),
        "https://geocoding-api.open-meteo.com/v1/get?id=2950159"
    );
}
