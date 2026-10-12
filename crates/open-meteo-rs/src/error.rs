/// Anything that can go wrong while talking to the API.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The request never completed: connection, timeout, TLS, ...
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    /// The API answered, but with an error status or an undecodable body.
    #[error(transparent)]
    Api(#[from] open_meteo_core::Error),
}

/// Shorthand for results of this crate's [`Error`].
pub type Result<T> = core::result::Result<T, Error>;
