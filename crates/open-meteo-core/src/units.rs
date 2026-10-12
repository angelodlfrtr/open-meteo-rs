//! Units and selectors shared by several endpoints.

use alloc::string::{String, ToString};
use core::fmt::{self, Display};

use crate::ConversionError;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Elevation passed to the API for statistical downscaling.
pub enum Elevation {
    /// Disable downscaling and use the grid cell's average elevation.
    Nan,
    /// An explicit elevation in metres.
    Value(f32),
}

impl Display for Elevation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nan => write!(f, "nan"),
            Self::Value(v) => write!(f, "{v}"),
        }
    }
}

impl From<Elevation> for String {
    fn from(value: Elevation) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for Elevation {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "nan" {
            return Ok(Self::Nan);
        }

        Err(ConversionError::InvalidElevation {
            elevation: value.to_string(),
        })
    }
}

impl From<f32> for Elevation {
    fn from(value: f32) -> Self {
        Self::Value(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Unit for temperature variables.
pub enum TemperatureUnit {
    /// Degrees Celsius, the API default.
    Celsius,
    /// Degrees Fahrenheit.
    Fahrenheit,
}

impl Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Celsius => write!(f, "celsius"),
            Self::Fahrenheit => write!(f, "fahrenheit"),
        }
    }
}

impl From<TemperatureUnit> for String {
    fn from(value: TemperatureUnit) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for TemperatureUnit {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "celsius" => Ok(Self::Celsius),
            "fahrenheit" => Ok(Self::Fahrenheit),
            _ => Err(ConversionError::InvalidTemperatureUnit {
                unit: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Unit for wind speed variables.
pub enum WindSpeedUnit {
    /// Kilometres per hour, the API default.
    Kmh,
    /// Metres per second.
    Ms,
    /// Miles per hour.
    Mph,
    /// Knots.
    Kn,
}

impl Display for WindSpeedUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kmh => write!(f, "kmh"),
            Self::Ms => write!(f, "ms"),
            Self::Mph => write!(f, "mph"),
            Self::Kn => write!(f, "kn"),
        }
    }
}

impl From<WindSpeedUnit> for String {
    /// Default to kmh
    fn from(value: WindSpeedUnit) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for WindSpeedUnit {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "kmh" => Ok(Self::Kmh),
            "ms" => Ok(Self::Ms),
            "mph" => Ok(Self::Mph),
            "kn" => Ok(Self::Kn),
            _ => Err(ConversionError::InvalidWindspeedUnit {
                unit: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Unit for precipitation variables.
pub enum PrecipitationUnit {
    /// Millimetres, the API default.
    Millimeters,
    /// Inches.
    Inches,
}

impl Display for PrecipitationUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Millimeters => write!(f, "mm"),
            Self::Inches => write!(f, "inch"),
        }
    }
}

impl From<PrecipitationUnit> for String {
    /// Default to mm
    fn from(value: PrecipitationUnit) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for PrecipitationUnit {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "inch" => Ok(Self::Inches),
            "mm" => Ok(Self::Millimeters),
            _ => Err(ConversionError::InvalidPrecipitationUnit {
                unit: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// How the API picks the grid cell for a coordinate.
pub enum CellSelection {
    /// Prefer a land cell close to the coordinate. The default for most endpoints.
    Land,
    /// Prefer a sea cell. The default for the marine endpoint.
    Sea,
    /// Take the nearest cell regardless of its surface.
    Nearest,
}

impl Display for CellSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Land => write!(f, "land"),
            Self::Sea => write!(f, "sea"),
            Self::Nearest => write!(f, "nearest"),
        }
    }
}

impl From<CellSelection> for String {
    fn from(value: CellSelection) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for CellSelection {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "land" => Ok(Self::Land),
            "sea" => Ok(Self::Sea),
            "nearest" => Ok(Self::Nearest),
            _ => Err(ConversionError::InvalidCellSelection {
                selection: value.to_string(),
            }),
        }
    }
}

/// Length unit used by the marine endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Unit for lengths such as wave heights.
pub enum LengthUnit {
    /// Metres, the API default.
    Metric,
    /// Feet.
    Imperial,
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Metric => write!(f, "metric"),
            Self::Imperial => write!(f, "imperial"),
        }
    }
}

impl From<LengthUnit> for String {
    fn from(value: LengthUnit) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for LengthUnit {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "metric" => Ok(Self::Metric),
            "imperial" => Ok(Self::Imperial),
            _ => Err(ConversionError::InvalidLengthUnit {
                unit: value.to_string(),
            }),
        }
    }
}

/// Time step of the returned hourly data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalResolution {
    /// The native resolution of the underlying model.
    Native,
    /// One value per hour.
    Hourly,
    /// One value every 3 hours.
    Hourly3,
    /// One value every 6 hours.
    Hourly6,
}

impl Display for TemporalResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native => write!(f, "native"),
            Self::Hourly => write!(f, "hourly"),
            Self::Hourly3 => write!(f, "hourly_3"),
            Self::Hourly6 => write!(f, "hourly_6"),
        }
    }
}

impl From<TemporalResolution> for String {
    fn from(value: TemporalResolution) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for TemporalResolution {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "native" => Ok(Self::Native),
            "hourly" => Ok(Self::Hourly),
            "hourly_3" => Ok(Self::Hourly3),
            "hourly_6" => Ok(Self::Hourly6),
            _ => Err(ConversionError::InvalidTemporalResolution {
                resolution: value.to_string(),
            }),
        }
    }
}
