use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "cargo-check-updates",
    about = "Upgrade your Cargo.toml dependencies to the latest versions",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Upgrade dependencies in Cargo.toml (default: dry-run only)
    #[arg(short, long)]
    pub upgrade: bool,

    /// Interactive mode - select which packages to upgrade
    #[arg(short, long)]
    pub interactive: bool,

    /// Path to Cargo.toml file (default: ./Cargo.toml)
    #[arg(long, default_value = "Cargo.toml")]
    pub manifest_path: String,

    /// Filter packages by name (glob patterns supported)
    #[arg(value_name = "PACKAGE")]
    pub filter: Vec<String>,

    /// Reject specific packages (won't update these)
    #[arg(long, value_name = "PACKAGE")]
    pub reject: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Check for available updates (default)
    Check {
        /// Show only outdated dependencies
        #[arg(long)]
        outdated: bool,
    },
}
