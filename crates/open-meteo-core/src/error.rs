use alloc::string::String;

use serde::Deserialize;

/// Errors produced while decoding an API response.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The API answered with a non-success HTTP status.
    #[error("the API returned status {status}: {reason}")]
    Api {
        /// The HTTP status code.
        status: u16,
        /// The `reason` the API gave, or the raw body when there was none.
        reason: String,
    },

    /// The response body is not valid JSON, or does not match the expected types.
    #[error("cannot decode the API response: {0}")]
    Json(#[from] serde_json::Error),

    /// The JSON was valid but not shaped the way this crate expects.
    #[error("malformed API response: {0}")]
    Malformed(&'static str),
}

impl Error {
    /// Build an [`Error::Api`] from a non-success status and the raw response body.
    ///
    /// Open-Meteo reports errors as `{"error": true, "reason": "..."}`. The reason is used
    /// when present; otherwise the body is kept verbatim so nothing is lost.
    #[must_use]
    pub fn from_status(status: u16, body: &[u8]) -> Self {
        #[derive(Deserialize)]
        struct ApiError {
            reason: String,
        }

        let reason = serde_json::from_slice::<ApiError>(body).map_or_else(
            |_| String::from_utf8_lossy(body).into_owned(),
            |error| error.reason,
        );

        Self::Api { status, reason }
    }
}

/// Errors produced when converting a string into one of the option enums.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ConversionError {
    /// Not `nan`; numeric elevations are built with `Elevation::from(f32)`.
    #[error("invalid elevation '{elevation}'")]
    InvalidElevation {
        /// The rejected input.
        elevation: String,
    },

    /// Not one of `celsius`, `fahrenheit`.
    #[error("invalid temperature unit '{unit}'")]
    InvalidTemperatureUnit {
        /// The rejected input.
        unit: String,
    },

    /// Not one of `kmh`, `ms`, `mph`, `kn`.
    #[error("invalid wind speed unit '{unit}'")]
    InvalidWindspeedUnit {
        /// The rejected input.
        unit: String,
    },

    /// Not one of `mm`, `inch`.
    #[error("invalid precipitation unit '{unit}'")]
    InvalidPrecipitationUnit {
        /// The rejected input.
        unit: String,
    },

    /// Not one of `metric`, `imperial`.
    #[error("invalid length unit '{unit}'")]
    InvalidLengthUnit {
        /// The rejected input.
        unit: String,
    },

    /// Not one of `native`, `hourly`, `hourly_3`, `hourly_6`.
    #[error("invalid temporal resolution '{resolution}'")]
    InvalidTemporalResolution {
        /// The rejected input.
        resolution: String,
    },

    /// Not a model id this crate knows. See [`crate::Model`].
    #[error("invalid model '{model}'")]
    InvalidModel {
        /// The rejected input.
        model: String,
    },

    /// Not one of `land`, `sea`, `nearest`.
    #[error("invalid cell selection '{selection}'")]
    InvalidCellSelection {
        /// The rejected input.
        selection: String,
    },
}
