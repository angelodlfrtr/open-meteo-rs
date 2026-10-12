use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{self, Display};

use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

use crate::Location;

/// Characters percent-encoded in query values.
///
/// Everything outside the unreserved set is encoded, except `,`, `:` and `/`, which Open-Meteo
/// accepts verbatim. Leaving them alone keeps variable lists and time zones readable in URLs.
const VALUE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~')
    .remove(b',')
    .remove(b':')
    .remove(b'/');

/// A URL query string under construction.
///
/// Every endpoint builds its query through this type so that encoding, list joining and
/// the handling of optional parameters are decided in exactly one place.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Query {
    pairs: Vec<(&'static str, String)>,
}

impl Query {
    /// An empty query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append `key=value`.
    pub fn param(&mut self, key: &'static str, value: impl Display) -> &mut Self {
        self.pairs.push((key, value.to_string()));
        self
    }

    /// Append `key=value` when `value` is `Some`, do nothing otherwise.
    pub fn optional(&mut self, key: &'static str, value: Option<impl Display>) -> &mut Self {
        if let Some(value) = value {
            self.param(key, value);
        }
        self
    }

    /// Append `key=true` when `enabled`, do nothing otherwise.
    pub fn flag(&mut self, key: &'static str, enabled: bool) -> &mut Self {
        if enabled {
            self.param(key, "true");
        }
        self
    }

    /// Append `key=a,b,c` when `items` is not empty, do nothing otherwise.
    pub fn list<T: Display>(&mut self, key: &'static str, items: &[T]) -> &mut Self {
        if !items.is_empty() {
            self.param(key, Joined(items));
        }
        self
    }

    /// Append the `latitude` and `longitude` parameters of one location.
    pub fn location(&mut self, location: &Location) -> &mut Self {
        self.param("latitude", location.lat)
            .param("longitude", location.lng)
    }

    /// Append the `latitude` and `longitude` parameters of several locations as comma separated lists.
    pub fn locations(&mut self, locations: &[Location]) -> &mut Self {
        let lats: Vec<f64> = locations.iter().map(|l| l.lat).collect();
        let lngs: Vec<f64> = locations.iter().map(|l| l.lng).collect();
        self.list("latitude", &lats).list("longitude", &lngs)
    }

    /// The raw, unencoded key/value pairs in insertion order.
    #[must_use]
    pub fn pairs(&self) -> &[(&'static str, String)] {
        &self.pairs
    }
}

impl Display for Query {
    /// Percent-encoded `key=value&key=value` form, ready to be appended after `?`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, (key, value)) in self.pairs.iter().enumerate() {
            if index > 0 {
                f.write_str("&")?;
            }
            write!(f, "{key}={}", utf8_percent_encode(value, VALUE_SET))?;
        }
        Ok(())
    }
}

/// Displays a slice as comma separated values.
struct Joined<'a, T>(&'a [T]);

impl<T: Display> Display for Joined<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, item) in self.0.iter().enumerate() {
            if index > 0 {
                f.write_str(",")?;
            }
            write!(f, "{item}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_pairs_in_order() {
        let mut query = Query::new();
        query
            .location(&Location::default())
            .param("timeformat", "unixtime")
            .optional("timezone", Some("Europe/Paris"))
            .optional("past_days", None::<&str>)
            .flag("ensemble", true)
            .flag("skipped", false)
            .list("hourly", &["temperature_2m", "snowfall"])
            .list("daily", &[] as &[&str])
            .param("name", "Saint Étienne");

        assert_eq!(query.pairs().len(), 7);
        assert_eq!(query.pairs()[0], ("latitude", "52.52".to_string()));

        assert_eq!(
            query.to_string(),
            "latitude=52.52&longitude=13.41&timeformat=unixtime&timezone=Europe/Paris\
             &ensemble=true&hourly=temperature_2m,snowfall&name=Saint%20%C3%89tienne"
        );
    }

    #[test]
    fn several_locations_become_lists() {
        let mut query = Query::new();
        query.locations(&[
            Location {
                lat: 52.52,
                lng: 13.41,
            },
            Location {
                lat: 48.85,
                lng: 2.35,
            },
        ]);

        assert_eq!(
            query.to_string(),
            "latitude=52.52,48.85&longitude=13.41,2.35"
        );
    }
}
