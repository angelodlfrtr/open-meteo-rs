use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum ClientError {
    #[snafu(display("The status code '{status_code}' was returned: {text}"))]
    InvalidResponseStatus { status_code: u16, text: String },
}

#[derive(Debug, Snafu)]
pub enum ConversionError {
    #[snafu(display("Invalid elevation '{elevation}'"))]
    InvalidElevation { elevation: String },

    #[snafu(display("Invalid temperature unit '{unit}'"))]
    InvalidTemperatureUnit { unit: String },

    #[snafu(display("Invalid windspeed unit '{unit}'"))]
    InvalidWindspeedUnit { unit: String },

    #[snafu(display("Invalid precipitation unit '{unit}'"))]
    InvalidPrecipitationUnit { unit: String },

    #[snafu(display("Invalid cell selection '{selection}'"))]
    InvalidCellSelection { selection: String },
}
