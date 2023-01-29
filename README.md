# open-meteo-rs

A simple rust client for https://open-meteo.com/ API. It support the `forecast` endpoint.

## Usage

```rust
use open_meteo_rs;

#[tokio::main]
async fn main() {
    let client = open_meteo_rs::Client::new();

    let mut opts = open_meteo_rs::forecast::Options::default();
    opts.location = open_meteo_rs::Location {
        lat: 48.864716,
        lng: 2.349014,
    };
    opts.current_weather = Some(true);

    let res = client.forecast(opts).await.unwrap();
    println!("{:?}", res);
}
```
