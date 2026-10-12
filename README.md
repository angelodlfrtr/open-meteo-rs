# open-meteo-rs

Rust client for the [Open-Meteo](https://open-meteo.com/) weather API. Every endpoint of the
public API is supported, with an async client by default and a blocking one behind a feature.

- Crates.io: <https://crates.io/crates/open-meteo-rs>
- Docs.rs: <https://docs.rs/open-meteo-rs/>

The repository is a workspace of two crates:

| Crate             | What it is                                                                                                                                          |
| ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `open-meteo-rs`   | The `reqwest` based client. Start here.                                                                                                             |
| `open-meteo-core` | Request building and response parsing with no I/O, `no_std` + `alloc`. Use it to bring your own HTTP transport, for instance on an embedded target. |

## Installation

```sh
cargo add open-meteo-rs
```

## Usage

```rust
use open_meteo_rs::{forecast, Client, Location};

#[tokio::main]
async fn main() -> Result<(), open_meteo_rs::Error> {
    let client = Client::new();

    let options = forecast::Options {
        location: Location { lat: 48.864716, lng: 2.349014 },
        current: vec!["temperature_2m".into()],
        hourly: vec!["temperature_2m".into(), "precipitation".into()],
        daily: vec!["temperature_2m_max".into()],
        temperature_unit: Some(forecast::TemperatureUnit::Celsius),
        time_zone: Some("Europe/Paris".into()),
        forecast_days: Some(3),
        ..Default::default()
    };

    let forecast = client.forecast(options).await?;

    if let Some(current) = forecast.current {
        println!("now ({}): {:?}", current.datetime, current.values["temperature_2m"]);
    }
    for hour in forecast.hourly.unwrap_or_default() {
        println!("{}: {:?}", hour.datetime, hour.values["temperature_2m"].value);
    }

    Ok(())
}
```

Variable names (`temperature_2m`, `wave_height`, `river_discharge`, ...) are passed through
to the API as-is; the [Open-Meteo documentation](https://open-meteo.com/en/docs) lists what
each endpoint accepts. Results come back as one record per time step, each holding a map of
variable name to value and unit. Times are local to the requested time zone.

## Endpoints

| Method on `Client`    | Options type                   | Open-Meteo product                                                                       |
| --------------------- | ------------------------------ | ---------------------------------------------------------------------------------------- |
| `forecast`            | `forecast::Options`            | [Weather forecast](https://open-meteo.com/en/docs)                                       |
| `archive`             | `forecast::Options`            | [Historical weather](https://open-meteo.com/en/docs/historical-weather-api)              |
| `historical_forecast` | `forecast::Options`            | [Historical forecast](https://open-meteo.com/en/docs/historical-forecast-api)            |
| `previous_runs`       | `forecast::Options`            | [Previous model runs](https://open-meteo.com/en/docs/previous-runs-api)                  |
| `ensemble`            | `ensemble::Options`            | [Ensemble models](https://open-meteo.com/en/docs/ensemble-api), including ensemble means |
| `seasonal`            | `seasonal::Options`            | [Seasonal forecast](https://open-meteo.com/en/docs/seasonal-forecast-api)                |
| `climate`             | `climate::Options`             | [Climate change](https://open-meteo.com/en/docs/climate-api)                             |
| `marine`              | `marine::Options`              | [Marine weather](https://open-meteo.com/en/docs/marine-weather-api)                      |
| `air_quality`         | `air_quality::Options`         | [Air quality](https://open-meteo.com/en/docs/air-quality-api)                            |
| `satellite_radiation` | `satellite_radiation::Options` | [Satellite radiation](https://open-meteo.com/en/docs/satellite-radiation-api)            |
| `flood`               | `flood::Options`               | [Flood](https://open-meteo.com/en/docs/flood-api)                                        |
| `elevation`           | `elevation::Options`           | [Elevation](https://open-meteo.com/en/docs/elevation-api)                                |
| `geocoding`           | `geocoding::Options`           | [Geocoding](https://open-meteo.com/en/docs/geocoding-api) search                         |
| `geocoding_get`       | `geocoding::GetOptions`        | Geocoding lookup by id                                                                   |

Every option struct implements `Default`, so only the fields you need have to be set.

## Blocking client

Enable the `blocking` feature for a synchronous client with the same methods:

```sh
cargo add open-meteo-rs --features blocking
```

```rust
let client = open_meteo_rs::blocking::Client::new();
let forecast = client.forecast(options)?;
```

The blocking client must not be used from inside an async runtime, or it will panic.

## Errors

Every method returns `Result<_, open_meteo_rs::Error>`:

- `Error::Http` when the request did not complete (connection, timeout, TLS).
- `Error::Api` when the API answered with an error status (the message from the API is
  included) or with a body that could not be decoded.

## Commercial API and self-hosting

Base URLs live in the `Endpoints` struct. Override the ones you need:

```rust
let endpoints = open_meteo_rs::Endpoints {
    forecast: "https://customer-api.open-meteo.com/v1/forecast".into(),
    ..Default::default()
};
let client = open_meteo_rs::Client::new().with_endpoints(endpoints);
```

Pass your key through the `apikey` field of any options struct.

## Bring your own transport

`open-meteo-core` has no I/O and no dependency on `std`. A transport is a GET request plus one
parse call:

```rust
use open_meteo_core::{forecast, url, Endpoints, Request};

let endpoints = Endpoints::default();
let options = forecast::Options { hourly: vec!["temperature_2m".into()], ..Default::default() };

let request_url = url(&endpoints.forecast, &options);
let body: Vec<u8> = my_http_get(&request_url)?;
let forecast = forecast::Options::parse(&body)?;
```

Check the status before parsing; on a non-success status, `Error::from_status(status, &body)`
turns the API's error body into a readable error.

## Development

```sh
make clippy        # lints, all features and targets
make test          # offline tests: recorded API answers and exact URLs
make test-live     # smoke tests against the public API
make check-no-std  # builds the core for a target without std
make fmt           # dprint + cargo fmt
```
