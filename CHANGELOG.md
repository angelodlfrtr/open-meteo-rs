# Changelog

## 0.2.0

The crate is now a workspace: `open-meteo-core` holds request building and response parsing
with no I/O (`no_std` + `alloc`), and `open-meteo-rs` is the `reqwest` client on top of it.

### Added

- Every public Open-Meteo endpoint: historical forecast, previous runs, ensemble, seasonal,
  climate, marine, satellite radiation, flood, elevation and geocoding lookup by id.
- A blocking client behind the `blocking` feature.
- `Client::send` to reach any endpoint with any `Request` implementation.
- `Endpoints` to override base URLs, for the commercial API or self-hosted instances.
- `start_hour`/`end_hour` and `past_hours`/`forecast_hours` on forecast options,
  `country_code` on geocoding search.

### Changed (breaking)

- Methods return typed errors (`open_meteo_rs::Error`) instead of `Box<dyn Error>`.
- Result records use `BTreeMap` instead of `HashMap`. `ForecastResultHourly` and friends are
  now aliases of the shared `TimeStep`, `DateStep` and `Measurement` types.
- `Client` exposes `endpoints: Endpoints` instead of one field and builder per host.
- `forecast::Options::models` is a `Vec<Model>` instead of `Option<Vec<Model>>`.
- The `current` block gets the time zone offset applied like every other section. It used to
  be reported in UTC while hourly data was local.
- `forecast::Options::as_params`, `unix_time_to_naive_datetime` and `extract_times` are no
  longer public.
- `tokio` is no longer a dependency of the library.
