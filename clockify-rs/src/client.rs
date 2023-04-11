use crate::Config;

/// A single connection to clockify.
pub struct Client {
    /// The configuration for the client to
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
