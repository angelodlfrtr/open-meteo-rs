extern crate open_meteo_rs;

use chrono::{TimeZone, Utc};

#[tokio::main]
async fn main() {
    let client = open_meteo_rs::Client::new();
    let options = open_meteo_rs::forecast::Options {
        hourly: vec!["temperature_2m".to_string()],
        start_date: Some(
            Utc.with_ymd_and_hms(2023, 05, 01, 0, 0, 0)
                .unwrap()
                .date_naive(),
        ),
        end_date: Some(
            Utc.with_ymd_and_hms(2023, 05, 02, 0, 0, 0)
                .unwrap()
                .date_naive(),
        ),
        ..Default::default()
    };

    let response = client.archive(options).await.unwrap();

    dbg!(response);
}
