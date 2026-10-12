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

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks `Display`, `TryFrom<&str>` and `From<T> for String` agree on every variant.
    fn round_trips<T>(cases: &[(T, &str)])
    where
        T: Copy + PartialEq + core::fmt::Debug + Display + Into<String> + for<'a> TryFrom<&'a str>,
        for<'a> <T as TryFrom<&'a str>>::Error: core::fmt::Debug,
    {
        for (value, id) in cases {
            assert_eq!(value.to_string(), *id);
            assert_eq!(Into::<String>::into(*value), *id);
            assert_eq!(T::try_from(*id).unwrap(), *value);
        }
    }

    #[test]
    fn temperature_units() {
        round_trips(&[
            (TemperatureUnit::Celsius, "celsius"),
            (TemperatureUnit::Fahrenheit, "fahrenheit"),
        ]);
        assert_eq!(
            TemperatureUnit::try_from("kelvin").unwrap_err().to_string(),
            "invalid temperature unit 'kelvin'"
        );
    }

    #[test]
    fn wind_speed_units() {
        round_trips(&[
            (WindSpeedUnit::Kmh, "kmh"),
            (WindSpeedUnit::Ms, "ms"),
            (WindSpeedUnit::Mph, "mph"),
            (WindSpeedUnit::Kn, "kn"),
        ]);
        assert_eq!(
            WindSpeedUnit::try_from("bft").unwrap_err().to_string(),
            "invalid wind speed unit 'bft'"
        );
    }

    #[test]
    fn precipitation_units() {
        round_trips(&[
            (PrecipitationUnit::Millimeters, "mm"),
            (PrecipitationUnit::Inches, "inch"),
        ]);
        assert_eq!(
            PrecipitationUnit::try_from("cm").unwrap_err().to_string(),
            "invalid precipitation unit 'cm'"
        );
    }

    #[test]
    fn length_units() {
        round_trips(&[
            (LengthUnit::Metric, "metric"),
            (LengthUnit::Imperial, "imperial"),
        ]);
        assert_eq!(
            LengthUnit::try_from("nautical").unwrap_err().to_string(),
            "invalid length unit 'nautical'"
        );
    }

    #[test]
    fn cell_selections() {
        round_trips(&[
            (CellSelection::Land, "land"),
            (CellSelection::Sea, "sea"),
            (CellSelection::Nearest, "nearest"),
        ]);
        assert_eq!(
            CellSelection::try_from("air").unwrap_err().to_string(),
            "invalid cell selection 'air'"
        );
    }

    #[test]
    fn temporal_resolutions() {
        round_trips(&[
            (TemporalResolution::Native, "native"),
            (TemporalResolution::Hourly, "hourly"),
            (TemporalResolution::Hourly3, "hourly_3"),
            (TemporalResolution::Hourly6, "hourly_6"),
        ]);
        assert_eq!(
            TemporalResolution::try_from("daily")
                .unwrap_err()
                .to_string(),
            "invalid temporal resolution 'daily'"
        );
    }

    #[test]
    fn elevation() {
        assert_eq!(Elevation::Nan.to_string(), "nan");
        assert_eq!(String::from(Elevation::Value(150.9)), "150.9");
        assert_eq!(Elevation::from(8.5), Elevation::Value(8.5));
        assert_eq!(Elevation::try_from("nan").unwrap(), Elevation::Nan);
        // Only `nan` parses; numbers come through `From<f32>`.
        assert_eq!(
            Elevation::try_from("150").unwrap_err().to_string(),
            "invalid elevation '150'"
        );
    }
}
