extern crate open_meteo_rs;

#[tokio::main]
async fn main() {
    let client = open_meteo_rs::Client::new();
    let mut opts = open_meteo_rs::forecast::Options::default();

    // Location
    opts.location = open_meteo_rs::Location {
        lat: 48.864716,
        lng: 2.349014,
    };

    // Elevation
    opts.elevation = Some(open_meteo_rs::forecast::Elevation::Nan); // or
    opts.elevation = Some(open_meteo_rs::forecast::Elevation::Value(150.9)); // or
    opts.elevation = Some("nan".try_into().unwrap()); // or
    opts.elevation = Some(150.9.into());

    // Current weather
    opts.current_weather = Some(true);

    // Temperature unit
    opts.temperature_unit = Some(open_meteo_rs::forecast::TemperatureUnit::Fahrenheit); // or
    opts.temperature_unit = Some(open_meteo_rs::forecast::TemperatureUnit::Celsius); // or
    opts.temperature_unit = Some("fahrenheit".try_into().unwrap()); // or
    opts.temperature_unit = Some("celsius".try_into().unwrap()); // or

    // Wind speed unit
    opts.wind_speed_unit = Some(open_meteo_rs::forecast::WindSpeedUnit::Kmh); // or
    opts.wind_speed_unit = Some(open_meteo_rs::forecast::WindSpeedUnit::Ms); // or
    opts.wind_speed_unit = Some(open_meteo_rs::forecast::WindSpeedUnit::Mph); // or
    opts.wind_speed_unit = Some(open_meteo_rs::forecast::WindSpeedUnit::Kn); // or
    opts.wind_speed_unit = Some("kmh".try_into().unwrap()); // or
    opts.wind_speed_unit = Some("ms".try_into().unwrap()); // or
    opts.wind_speed_unit = Some("mph".try_into().unwrap()); // or
    opts.wind_speed_unit = Some("kn".try_into().unwrap());

    // Precipitation unit
    opts.precipitation_unit = Some(open_meteo_rs::forecast::PrecipitationUnit::Millimeters); // or
    opts.precipitation_unit = Some(open_meteo_rs::forecast::PrecipitationUnit::Inches); // or
    opts.precipitation_unit = Some("mm".try_into().unwrap()); // or
    opts.precipitation_unit = Some("inch".try_into().unwrap()); // or

    // Time zone (default to UTC)
    opts.time_zone = Some(chrono_tz::Europe::Paris.name().into());

    // Past days (0-2)
    // opts.past_days = Some(2); // !! mutually exclusive with dates

    // Forecast days (0-16)
    // opts.forecast_days = Some(2); // !! mutually exclusive with dates

    // Dates
    let start_date = chrono::Utc::now()
        .with_timezone(&chrono_tz::Europe::Paris)
        .naive_local()
        .date();
    opts.start_date = Some(start_date);
    opts.end_date = Some(start_date + chrono::Duration::days(2));

    // Models
    // opts.models = Some(vec!["auto".into()]); // Crash on server side

    // Cell selection
    opts.cell_selection = Some(open_meteo_rs::forecast::CellSelection::Land); // or
    opts.cell_selection = Some(open_meteo_rs::forecast::CellSelection::Sea); // or
    opts.cell_selection = Some(open_meteo_rs::forecast::CellSelection::Nearest); // or
    opts.cell_selection = Some("land".try_into().unwrap()); // or
    opts.cell_selection = Some("sea".try_into().unwrap()); // or
    opts.cell_selection = Some("nearest".try_into().unwrap());

    // Hourly parameters
    opts.hourly.push("temperature_2m".into());
    opts.hourly.push("snowfall".into());
    // ...

    // Daily parameters
    opts.daily.push("temperature_2m_max".into());
    opts.daily.push("snowfall_sum".into());

    let res = client.forecast(opts).await.unwrap();

    println!("{:#?}", res);
}
