//! The exact URLs each request produces.

use chrono::NaiveDate;
use open_meteo_core::{
    air_quality, climate, elevation, ensemble, flood, forecast, geocoding, marine,
    satellite_radiation, seasonal, url, CellSelection, Endpoints, LengthUnit, Location, Model,
    TemporalResolution,
};

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

#[test]
fn marine() {
    let options = marine::Options {
        current: vec!["wave_height".into()],
        hourly: vec!["wave_height".into(), "wave_direction".into()],
        daily: vec!["wave_height_max".into()],
        length_unit: Some(LengthUnit::Imperial),
        forecast_days: Some(1),
        cell_selection: Some(CellSelection::Nearest),
        ..Default::default()
    };

    assert_eq!(
        url(&Endpoints::default().marine, &options),
        "https://marine-api.open-meteo.com/v1/marine\
         ?latitude=52.52&longitude=13.41&timeformat=unixtime&current=wave_height\
         &hourly=wave_height,wave_direction&daily=wave_height_max&length_unit=imperial\
         &forecast_days=1&cell_selection=nearest"
    );
}

#[test]
fn ensemble() {
    let options = ensemble::Options {
        hourly: vec!["temperature_2m".into()],
        models: vec![Model::IconSeamless, Model::GfsSeamless],
        forecast_days: Some(2),
        start_hour: Some(date(2024, 6, 1).and_hms_opt(0, 0, 0).unwrap()),
        ..Default::default()
    };

    assert_eq!(
        url(&Endpoints::default().ensemble, &options),
        "https://ensemble-api.open-meteo.com/v1/ensemble\
         ?latitude=52.52&longitude=13.41&timeformat=unixtime&hourly=temperature_2m\
         &models=icon_seamless,gfs_seamless&forecast_days=2&start_hour=2024-06-01T00:00"
    );
}

#[test]
fn climate() {
    let options = climate::Options {
        start_date: date(2030, 1, 1),
        end_date: date(2030, 1, 3),
        models: vec![Model::MRIAGCM32S, Model::ECEarth3pHR],
        daily: vec!["temperature_2m_max".into()],
        disable_bias_correction: true,
        ..Default::default()
    };

    assert_eq!(
        url(&Endpoints::default().climate, &options),
        "https://climate-api.open-meteo.com/v1/climate\
         ?latitude=52.52&longitude=13.41&timeformat=unixtime&start_date=2030-01-01\
         &end_date=2030-01-03&models=MRI_AGCM3_2_S,EC_Earth3P_HR&daily=temperature_2m_max\
         &disable_bias_correction=true"
    );
}

#[test]
fn flood() {
    let options = flood::Options {
        location: Location {
            lat: 59.91,
            lng: 10.75,
        },
        daily: vec!["river_discharge".into()],
        forecast_days: Some(3),
        ensemble: true,
        ..Default::default()
    };

    assert_eq!(
        url(&Endpoints::default().flood, &options),
        "https://flood-api.open-meteo.com/v1/flood\
         ?latitude=59.91&longitude=10.75&timeformat=unixtime&daily=river_discharge\
         &forecast_days=3&ensemble=true"
    );
}

#[test]
fn satellite_radiation() {
    let options = satellite_radiation::Options {
        hourly: vec![
            "shortwave_radiation".into(),
            "global_tilted_irradiance".into(),
        ],
        models: vec![Model::SatelliteRadiationSeamless],
        temporal_resolution: Some(TemporalResolution::Native),
        tilt: Some(35.0),
        azimuth: Some(-10.0),
        past_days: Some(1),
        forecast_days: Some(0),
        ..Default::default()
    };

    assert_eq!(
        url(&Endpoints::default().satellite_radiation, &options),
        "https://satellite-api.open-meteo.com/v1/archive\
         ?latitude=52.52&longitude=13.41&timeformat=unixtime\
         &hourly=shortwave_radiation,global_tilted_irradiance\
         &models=satellite_radiation_seamless&temporal_resolution=native&tilt=35&azimuth=-10\
         &past_days=1&forecast_days=0"
    );
}

#[test]
fn seasonal() {
    let options = seasonal::Options {
        daily: vec!["temperature_2m_max".into()],
        weekly: vec!["temperature_2m_mean".into()],
        monthly: vec!["temperature_2m_mean".into()],
        models: vec![Model::EcmwfSeas5],
        forecast_days: Some(45),
        ..Default::default()
    };

    assert_eq!(
        url(&Endpoints::default().seasonal, &options),
        "https://seasonal-api.open-meteo.com/v1/seasonal\
         ?latitude=52.52&longitude=13.41&timeformat=unixtime&daily=temperature_2m_max\
         &weekly=temperature_2m_mean&monthly=temperature_2m_mean&models=ecmwf_seas5\
         &forecast_days=45"
    );
}

#[test]
fn elevation() {
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

    assert_eq!(
        url(&Endpoints::default().elevation, &options),
        "https://api.open-meteo.com/v1/elevation?latitude=52.52,48.85&longitude=13.41,2.35"
    );
    assert_eq!(
        url(
            &Endpoints::default().elevation,
            &elevation::Options::single(Location::default())
        ),
        "https://api.open-meteo.com/v1/elevation?latitude=52.52&longitude=13.41"
    );
}
