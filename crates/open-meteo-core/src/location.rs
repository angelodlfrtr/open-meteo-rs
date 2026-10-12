use serde::{Deserialize, Serialize};

/// A WGS84 coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

impl Default for Location {
    /// Berlin, the location used throughout the Open-Meteo documentation.
    fn default() -> Self {
        Self {
            lat: 52.52,
            lng: 13.41,
        }
    }
}
