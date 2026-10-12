//! Run with: cargo run --example blocking --features blocking

fn main() {
    let client = open_meteo_rs::blocking::Client::new();

    let mut opts = open_meteo_rs::forecast::Options {
        location: open_meteo_rs::Location {
            lat: 48.864716,
            lng: 2.349014,
        },
        ..Default::default()
    };
    opts.current.push("temperature_2m".into());
    opts.hourly.push("temperature_2m".into());
    opts.daily.push("temperature_2m_max".into());

    let res = client.forecast(opts).unwrap();

    println!("{res:#?}");
}
