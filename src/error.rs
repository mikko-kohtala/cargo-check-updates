use thiserror::Error;

#[derive(Error, Debug)]
pub enum CcuError {
    #[error("Failed to parse Cargo.toml: {0}")]
    CargoTomlParse(String),

    #[error("Failed to query crates.io: {0}")]
    RegistryQuery(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("TOML edit error: {0}")]
    TomlEdit(#[from] toml_edit::TomlError),

    #[error("Semver error: {0}")]
    Semver(#[from] semver::Error),
}

pub type Result<T> = std::result::Result<T, CcuError>;
