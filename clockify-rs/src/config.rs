/// The clockify client configuration
#[derive(Clone)]
pub struct Config {
    /// The clockify API key to authorize access
    pub api_key: String,

    /// The rest API endpoint
    pub endpoint: String,
}

impl Config {
    /// Returns a new config object with default values.
    ///
    /// # Arguments
    /// * `api_key` - The API key used for accessing the API.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            endpoint: "https://api.clockify.me/api/".to_owned(),
        }
    }
}
