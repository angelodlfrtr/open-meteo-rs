use alloc::format;
use alloc::string::String;

use crate::{Error, Query};

/// A request to one Open-Meteo endpoint: how to ask, and how to read the answer.
///
/// Every `Options` type in this crate implements it. A transport only has to GET
/// [`url`] and feed the body to [`Request::parse`]; it never needs to know anything
/// about the endpoint itself.
pub trait Request {
    /// The decoded result for this request.
    type Response;

    /// The query string parameters for this request (everything after `?`).
    fn query(&self) -> Query;

    /// Decode the body of a successful (2xx) response.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] when the body is not the expected JSON and
    /// [`Error::Malformed`] when the JSON does not have the expected shape.
    fn parse(body: &[u8]) -> Result<Self::Response, Error>;
}

/// The full URL for `request` against the endpoint `base`, e.g. one of the fields of [`crate::Endpoints`].
#[must_use]
pub fn url<R: Request>(base: &str, request: &R) -> String {
    format!("{base}?{}", request.query())
}
