# open-meteo-core

Request building and response parsing for the [Open-Meteo](https://open-meteo.com) API, with no I/O.

This crate is `no_std` (it needs `alloc`) and is the foundation of
[`open-meteo-rs`](https://crates.io/crates/open-meteo-rs), the `reqwest` based client.
Depend on it directly when you want to bring your own HTTP transport, for instance on an
embedded target.

```rust
use open_meteo_core::{forecast, url, Endpoints, Location, Request};

let endpoints = Endpoints::default();
let options = forecast::Options {
    location: Location { lat: 52.52, lng: 13.41 },
    hourly: vec!["temperature_2m".into()],
    ..Default::default()
};

let request_url = url(&endpoints.forecast, &options);
// let body = my_http_get(&request_url)?;
// let result = forecast::Options::parse(&body)?;
```
