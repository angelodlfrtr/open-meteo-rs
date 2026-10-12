//! Terrain elevation from the Copernicus 90 m digital elevation model.

use alloc::string::String;
use alloc::vec::Vec;

use serde::Deserialize;

use crate::{Error, Location, Query, Request};

/// Parameters of an elevation lookup. Up to 100 locations per request.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    pub locations: Vec<Location>,
    pub apikey: Option<String>,
}

impl Options {
    /// Look up a single location.
    #[must_use]
    pub fn single(location: Location) -> Self {
        Self {
            locations: alloc::vec![location],
            apikey: None,
        }
    }
}

impl Request for Options {
    /// One elevation in metres per requested location, in request order.
    type Response = Vec<f64>;

    fn query(&self) -> Query {
        let mut query = Query::new();
        query
            .locations(&self.locations)
            .optional("apikey", self.apikey.as_deref());
        query
    }

    fn parse(body: &[u8]) -> Result<Vec<f64>, Error> {
        #[derive(Deserialize)]
        struct Response {
            elevation: Vec<f64>,
        }

        Ok(serde_json::from_slice::<Response>(body)?.elevation)
    }
}
