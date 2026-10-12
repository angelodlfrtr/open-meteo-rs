use alloc::string::String;

/// Public URL of the weather forecast endpoint.
pub const FORECAST: &str = "https://api.open-meteo.com/v1/forecast";
/// Public URL of the historical weather endpoint.
pub const ARCHIVE: &str = "https://archive-api.open-meteo.com/v1/archive";
/// Public URL of the historical forecast endpoint.
pub const HISTORICAL_FORECAST: &str = "https://historical-forecast-api.open-meteo.com/v1/forecast";
/// Public URL of the previous model runs endpoint.
pub const PREVIOUS_RUNS: &str = "https://previous-runs-api.open-meteo.com/v1/forecast";
/// Public URL of the ensemble endpoint.
pub const ENSEMBLE: &str = "https://ensemble-api.open-meteo.com/v1/ensemble";
/// Public URL of the climate change endpoint.
pub const CLIMATE: &str = "https://climate-api.open-meteo.com/v1/climate";
/// Public URL of the marine weather endpoint.
pub const MARINE: &str = "https://marine-api.open-meteo.com/v1/marine";
/// Public URL of the air quality endpoint.
pub const AIR_QUALITY: &str = "https://air-quality-api.open-meteo.com/v1/air-quality";
/// Public URL of the satellite radiation endpoint.
pub const SATELLITE_RADIATION: &str = "https://satellite-api.open-meteo.com/v1/archive";
/// Public URL of the seasonal forecast endpoint.
pub const SEASONAL: &str = "https://seasonal-api.open-meteo.com/v1/seasonal";
/// Public URL of the flood endpoint.
pub const FLOOD: &str = "https://flood-api.open-meteo.com/v1/flood";
/// Public URL of the elevation endpoint.
pub const ELEVATION: &str = "https://api.open-meteo.com/v1/elevation";
/// Public URL of the geocoding search endpoint.
pub const GEOCODING_SEARCH: &str = "https://geocoding-api.open-meteo.com/v1/search";
/// Public URL of the geocoding lookup by id endpoint.
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
    /// Base URL of the weather forecast endpoint.
    pub forecast: String,
    /// Base URL of the historical weather endpoint.
    pub archive: String,
    /// Base URL of the historical forecast endpoint.
    pub historical_forecast: String,
    /// Base URL of the previous model runs endpoint.
    pub previous_runs: String,
    /// Base URL of the ensemble endpoint.
    pub ensemble: String,
    /// Base URL of the climate change endpoint.
    pub climate: String,
    /// Base URL of the marine weather endpoint.
    pub marine: String,
    /// Base URL of the air quality endpoint.
    pub air_quality: String,
    /// Base URL of the satellite radiation endpoint.
    pub satellite_radiation: String,
    /// Base URL of the seasonal forecast endpoint.
    pub seasonal: String,
    /// Base URL of the flood endpoint.
    pub flood: String,
    /// Base URL of the elevation endpoint.
    pub elevation: String,
    /// Base URL of the geocoding search endpoint.
    pub geocoding_search: String,
    /// Base URL of the geocoding lookup by id endpoint.
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
