use crate::error::Result;
use reqwest::Client;
use semver::Version;
use serde::Deserialize;

const CRATES_IO_API: &str = "https://crates.io/api/v1";

#[derive(Deserialize, Debug)]
struct CrateResponse {
    #[serde(rename = "crate")]
    crate_info: CrateInfo,
}

#[derive(Deserialize, Debug)]
struct CrateInfo {
    newest_version: String,
}

#[derive(Clone)]
pub struct RegistryClient {
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

        // Make GET request to crates.io API
        let response = self.client.get(&url).send().await?;

        // Check if the request was successful
        if !response.status().is_success() {
            return Err(crate::error::CcuError::RegistryQuery(format!(
                "Failed to fetch crate '{}': HTTP {}",
                crate_name,
                response.status()
            )));
        }

        // Parse JSON response
        let crate_response: CrateResponse = response.json().await?;

        // Extract and parse version
        let version = Version::parse(&crate_response.crate_info.newest_version)?;

        Ok(version)
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
