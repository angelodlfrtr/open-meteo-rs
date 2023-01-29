use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum ClientError {
    #[snafu(display("The status code '{status_code}' was returned: {text}"))]
    InvalidResponseStatus { status_code: u16, text: String },
}
