use serde::Serialize;

/// Represents an [`InputLocationMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputlocationmessagecontent
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[must_use]
pub struct Location {
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<u64>,
}

impl Location {
    /// Constructs a `Location`.
    #[allow(clippy::unused_self)] // https://github.com/rust-lang/rust-clippy/issues/5351
    pub const fn new((latitude, longitude): (f64, f64)) -> Self {
        Self {
            latitude,
            longitude,
            live_period: None,
        }
    }

    /// Configures the period while the location will be live.
    pub const fn live_period(mut self, period: u64) -> Self {
        self.live_period = Some(period);
        self
    }
}
