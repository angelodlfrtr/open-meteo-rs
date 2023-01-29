#[derive(Debug, Clone)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            lat: 52.52,
            lng: 13.41,
        }
    }
}
