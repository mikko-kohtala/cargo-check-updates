use crate::cargo::parser::CargoTomlParser;
use crate::error::Result;
use crate::registry::client::RegistryClient;
use colored::Colorize;
use semver::Version;
use std::collections::HashMap;

pub struct CheckCommand {
    pub manifest_path: String,
    pub upgrade: bool,
    pub interactive: bool,
    pub filter: Vec<String>,
    pub reject: Vec<String>,
}

impl CheckCommand {
    pub fn new(
        manifest_path: String,
        upgrade: bool,
        interactive: bool,
        filter: Vec<String>,
        reject: Vec<String>,
    ) -> Self {
        Self {
            manifest_path,
            upgrade,
            interactive,
            filter,
            reject,
        }
    }

    pub async fn run(&self) -> Result<()> {
        println!("Checking {}\n", self.manifest_path);

        // 1. Parse Cargo.toml
        let mut parser = CargoTomlParser::from_path(&self.manifest_path)?;
        let dependencies = parser.get_all_dependencies();

        if dependencies.is_empty() {
            println!("No dependencies found.");
            return Ok(());
        }

        // 2. Query crates.io for each dependency in parallel
        let registry_client = RegistryClient::new();
        let mut handles = Vec::new();

        for dep in &dependencies {
            // Apply filters
            if !self.should_check_dependency(&dep.name) {
                continue;
            }

            let client = registry_client.clone();
            let name = dep.name.clone();

            let handle = tokio::spawn(async move {
                match client.get_latest_version(&name).await {
                    Ok(version) => Some((name, version)),
                    Err(_) => None, // Skip on error (e.g., crate not found)
                }
            });

            handles.push(handle);
        }

        // Collect results
        let mut latest_versions = HashMap::new();
        for handle in handles {
            if let Ok(Some((name, version))) = handle.await {
                latest_versions.insert(name, version);
            }
        }

        // 3. Compare versions and prepare updates
        let mut updates = Vec::new();
        for dep in &dependencies {
            if let Some(latest_version) = latest_versions.get(&dep.name) {
                // Parse current version (strip operators and normalize)
                let current_version_str = self.strip_version_operator(&dep.version);
                let normalized_version = self.normalize_version(&current_version_str);
                if let Ok(current_version) = Version::parse(&normalized_version) {
                    // Only show if there's an update
                    if latest_version > &current_version {
                        updates.push((dep, current_version, latest_version.clone()));
                    }
                }
            }
        }

        // 4. Display results
        if updates.is_empty() {
            println!("{}", "All dependencies are up to date!".green());
            return Ok(());
        }

        for (dep, current, latest) in &updates {
            self.print_update(&dep.name, &dep.version, current, latest);
        }

        // 5. If upgrade flag is set, update Cargo.toml
        if self.upgrade {
            println!("\n{}", "Upgrading dependencies...".cyan());

            for (dep, _, latest) in &updates {
                parser.update_dependency(&dep.name, &dep.section, &latest.to_string())?;
            }

            parser.save()?;
            println!("{}", "Cargo.toml has been updated!".green());
        } else {
            println!("\n{}", format!("Run ccu -u to upgrade {}", self.manifest_path).bold());
        }

        Ok(())
    }

    /// Check if a dependency should be checked based on filter/reject patterns
    fn should_check_dependency(&self, name: &str) -> bool {
        // If filters are specified, only check matching dependencies
        if !self.filter.is_empty() {
            let matches = self.filter.iter().any(|pattern| {
                // Simple wildcard matching (can be improved with glob crate)
                if pattern.contains('*') {
                    let pattern = pattern.replace('*', "");
                    name.contains(&pattern)
                } else {
                    name == pattern
                }
            });
            if !matches {
                return false;
            }
        }

        // If rejects are specified, skip matching dependencies
        if !self.reject.is_empty() {
            let matches = self.reject.iter().any(|pattern| {
                if pattern.contains('*') {
                    let pattern = pattern.replace('*', "");
                    name.contains(&pattern)
                } else {
                    name == pattern
                }
            });
            if matches {
                return false;
            }
        }

        true
    }

    /// Strip version operators from version string
    fn strip_version_operator(&self, version: &str) -> String {
        let version = version.trim();
        if version.starts_with("^") || version.starts_with("~") || version.starts_with("=") {
            version[1..].to_string()
        } else if version.starts_with(">=") || version.starts_with("<=") {
            version[2..].to_string()
        } else if version.starts_with('>') || version.starts_with('<') {
            version[1..].to_string()
        } else {
            version.to_string()
        }
    }

    /// Normalize version string to semver format (major.minor.patch)
    /// Cargo allows shorthand like "0.21" or "2", but semver requires full format
    fn normalize_version(&self, version: &str) -> String {
        let parts: Vec<&str> = version.split('.').collect();
        match parts.len() {
            1 => format!("{}.0.0", parts[0]),  // "2" → "2.0.0"
            2 => format!("{}.{}.0", parts[0], parts[1]),  // "0.21" → "0.21.0"
            _ => version.to_string(),  // Already complete or invalid
        }
    }

    /// Print a single update line
    fn print_update(&self, name: &str, current_with_op: &str, current: &Version, latest: &Version) {
        let arrow = "→";

        let colored_latest = if latest.major > current.major {
            latest.to_string().red().bold()
        } else if latest.minor > current.minor {
            latest.to_string().cyan()
        } else {
            latest.to_string().green()
        };

        println!(
            " {:<30} {:>10}  {}  {}",
            name,
            current_with_op,
            arrow,
            colored_latest
        );
    }
}
