use alloc::string::String;

use serde::Deserialize;

/// Errors produced while decoding an API response.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The API answered with a non-success HTTP status.
    #[error("the API returned status {status}: {reason}")]
    Api { status: u16, reason: String },

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
    #[error("invalid elevation '{elevation}'")]
    InvalidElevation { elevation: String },

    #[error("invalid temperature unit '{unit}'")]
    InvalidTemperatureUnit { unit: String },

    #[error("invalid wind speed unit '{unit}'")]
    InvalidWindspeedUnit { unit: String },

    #[error("invalid precipitation unit '{unit}'")]
    InvalidPrecipitationUnit { unit: String },

    #[error("invalid length unit '{unit}'")]
    InvalidLengthUnit { unit: String },

    #[error("invalid temporal resolution '{resolution}'")]
    InvalidTemporalResolution { resolution: String },

    #[error("invalid model '{model}'")]
    InvalidModel { model: String },

    #[error("invalid cell selection '{selection}'")]
    InvalidCellSelection { selection: String },
}
