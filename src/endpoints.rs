//! REST API endpoints implementation

use crate::DeribitHttpClient;

impl DeribitHttpClient {
    /// Get the base URL for API requests
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }
}
