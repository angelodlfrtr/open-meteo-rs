use alloc::string::String;

pub const FORECAST: &str = "https://api.open-meteo.com/v1/forecast";
pub const ARCHIVE: &str = "https://archive-api.open-meteo.com/v1/archive";
pub const HISTORICAL_FORECAST: &str = "https://historical-forecast-api.open-meteo.com/v1/forecast";
pub const PREVIOUS_RUNS: &str = "https://previous-runs-api.open-meteo.com/v1/forecast";
pub const ENSEMBLE: &str = "https://ensemble-api.open-meteo.com/v1/ensemble";
pub const CLIMATE: &str = "https://climate-api.open-meteo.com/v1/climate";
pub const MARINE: &str = "https://marine-api.open-meteo.com/v1/marine";
pub const AIR_QUALITY: &str = "https://air-quality-api.open-meteo.com/v1/air-quality";
pub const SATELLITE_RADIATION: &str = "https://satellite-api.open-meteo.com/v1/archive";
pub const SEASONAL: &str = "https://seasonal-api.open-meteo.com/v1/seasonal";
pub const FLOOD: &str = "https://flood-api.open-meteo.com/v1/flood";
pub const ELEVATION: &str = "https://api.open-meteo.com/v1/elevation";
pub const GEOCODING_SEARCH: &str = "https://geocoding-api.open-meteo.com/v1/search";
pub const GEOCODING_GET: &str = "https://geocoding-api.open-meteo.com/v1/get";

/// The base URL of every Open-Meteo endpoint.
///
/// The defaults point at the public API. Override individual fields to use a
/// self-hosted instance or the commercial `customer-` hosts:
///
/// ```
/// let endpoints = open_meteo_core::Endpoints {
///     forecast: "https://customer-api.open-meteo.com/v1/forecast".into(),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoints {
    pub forecast: String,
    pub archive: String,
    pub historical_forecast: String,
    pub previous_runs: String,
    pub ensemble: String,
    pub climate: String,
    pub marine: String,
    pub air_quality: String,
    pub satellite_radiation: String,
    pub seasonal: String,
    pub flood: String,
    pub elevation: String,
    pub geocoding_search: String,
    pub geocoding_get: String,
}

impl Default for Endpoints {
    fn default() -> Self {
        Self {
            forecast: FORECAST.into(),
            archive: ARCHIVE.into(),
            historical_forecast: HISTORICAL_FORECAST.into(),
            previous_runs: PREVIOUS_RUNS.into(),
            ensemble: ENSEMBLE.into(),
            climate: CLIMATE.into(),
            marine: MARINE.into(),
            air_quality: AIR_QUALITY.into(),
            satellite_radiation: SATELLITE_RADIATION.into(),
            seasonal: SEASONAL.into(),
            flood: FLOOD.into(),
            elevation: ELEVATION.into(),
            geocoding_search: GEOCODING_SEARCH.into(),
            geocoding_get: GEOCODING_GET.into(),
        }
    }
}
