use crate::error::Result;
use reqwest::Client;
use semver::Version;

const CRATES_IO_API: &str = "https://crates.io/api/v1";

pub struct RegistryClient {
    #[allow(dead_code)]
    client: Client,
}

impl RegistryClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("cargo-check-updates")
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    /// Get the latest version of a crate from crates.io
    pub async fn get_latest_version(&self, crate_name: &str) -> Result<Version> {
        let url = format!("{}/crates/{}", CRATES_IO_API, crate_name);

        // TODO: Implement actual API call
        // 1. Make GET request to crates.io API
        // 2. Parse JSON response
        // 3. Extract latest version from response
        // 4. Return Version

        // Placeholder for now
        let _ = url;
        Ok(Version::new(0, 1, 0))
    }

    /// Get all versions of a crate from crates.io
    pub async fn get_all_versions(&self, _crate_name: &str) -> Result<Vec<Version>> {
        // TODO: Implement actual API call
        Ok(vec![])
    }
}

impl Default for RegistryClient {
    fn default() -> Self {
        Self::new()
    }
}
