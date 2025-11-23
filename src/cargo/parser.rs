use crate::error::{CcuError, Result};
use std::path::Path;
use toml_edit::Document;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencySection {
    Dependencies,
    DevDependencies,
    BuildDependencies,
}

impl DependencySection {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Dependencies => "dependencies",
            Self::DevDependencies => "dev-dependencies",
            Self::BuildDependencies => "build-dependencies",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,  // e.g., "^1.0.0", "1.0", ">=0.5"
    pub section: DependencySection,
}

pub struct CargoTomlParser {
    document: Document,
    path: String,
}

impl CargoTomlParser {
    /// Parse a Cargo.toml file
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let content = std::fs::read_to_string(&path)
            .map_err(|e| CcuError::CargoTomlParse(format!("Failed to read file: {}", e)))?;

        let document = content
            .parse::<Document>()
            .map_err(|e| CcuError::CargoTomlParse(format!("Failed to parse TOML: {}", e)))?;

        Ok(Self {
            document,
            path: path_str,
        })
    }

    /// Get all dependencies from all sections
    pub fn get_all_dependencies(&self) -> Vec<Dependency> {
        let mut deps = Vec::new();

        deps.extend(self.get_dependencies_from_section(DependencySection::Dependencies));
        deps.extend(self.get_dependencies_from_section(DependencySection::DevDependencies));
        deps.extend(self.get_dependencies_from_section(DependencySection::BuildDependencies));

        deps
    }

    /// Get dependencies from a specific section
    fn get_dependencies_from_section(&self, section: DependencySection) -> Vec<Dependency> {
        let section_name = section.as_str();
        let mut dependencies = Vec::new();

        // Check if the section exists
        if let Some(deps_table) = self.document.get(section_name).and_then(|item| item.as_table()) {
            for (name, value) in deps_table.iter() {
                // Extract version from either string format ("1.0.0")
                // or table format ({ version = "1.0.0", features = [...] })
                let version = match value {
                    toml_edit::Item::Value(toml_edit::Value::String(s)) => {
                        s.value().to_string()
                    }
                    toml_edit::Item::Value(toml_edit::Value::InlineTable(table)) => {
                        if let Some(v) = table.get("version") {
                            if let Some(s) = v.as_str() {
                                s.to_string()
                            } else {
                                continue; // Skip if version is not a string
                            }
                        } else {
                            continue; // Skip if no version field
                        }
                    }
                    toml_edit::Item::Table(table) => {
                        if let Some(v) = table.get("version") {
                            if let Some(s) = v.as_str() {
                                s.to_string()
                            } else {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    }
                    _ => continue, // Skip other types (e.g., git dependencies, path dependencies)
                };

                dependencies.push(Dependency {
                    name: name.to_string(),
                    version,
                    section: section.clone(),
                });
            }
        }

        dependencies
    }

    /// Update a dependency version while preserving operators
    pub fn update_dependency(&mut self, name: &str, section: &DependencySection, new_version: &str) -> Result<()> {
        let section_name = section.as_str();

        // Get the section from the document
        let section_table = self.document
            .get_mut(section_name)
            .and_then(|item| item.as_table_mut())
            .ok_or_else(|| {
                CcuError::CargoTomlParse(format!("Section [{}] not found", section_name))
            })?;

        // Get the dependency entry
        let dep_entry = section_table
            .get_mut(name)
            .ok_or_else(|| {
                CcuError::CargoTomlParse(format!("Dependency '{}' not found in [{}]", name, section_name))
            })?;

        // Update the version based on the format
        match dep_entry {
            toml_edit::Item::Value(ref mut val) => {
                match val {
                    toml_edit::Value::String(ref mut s) => {
                        // Simple string format: preserve operator prefix
                        let old_version = s.value();
                        let new_version_with_operator = Self::preserve_version_operator(old_version, new_version);
                        *s = toml_edit::Formatted::new(new_version_with_operator);
                    }
                    toml_edit::Value::InlineTable(ref mut table) => {
                        // Inline table format: update the version field
                        if let Some(v) = table.get_mut("version") {
                            if let Some(old_str) = v.as_str() {
                                let new_version_with_operator = Self::preserve_version_operator(old_str, new_version);
                                *v = toml_edit::Value::from(new_version_with_operator);
                            }
                        }
                    }
                    _ => {
                        return Err(CcuError::CargoTomlParse(format!(
                            "Unsupported dependency format for '{}'",
                            name
                        )));
                    }
                }
            }
            toml_edit::Item::Table(ref mut table) => {
                // Table format: update the version field
                if let Some(toml_edit::Item::Value(ref mut v)) = table.get_mut("version") {
                    if let toml_edit::Value::String(ref mut s) = v {
                        let old_version = s.value();
                        let new_version_with_operator = Self::preserve_version_operator(old_version, new_version);
                        *s = toml_edit::Formatted::new(new_version_with_operator);
                    }
                }
            }
            _ => {
                return Err(CcuError::CargoTomlParse(format!(
                    "Unsupported dependency format for '{}'",
                    name
                )));
            }
        }

        Ok(())
    }

    /// Preserve version operators (^, ~, >=, etc.) from old version
    fn preserve_version_operator(old_version: &str, new_version: &str) -> String {
        // Extract operator prefix from old version
        let operator = if old_version.starts_with("^") {
            "^"
        } else if old_version.starts_with("~") {
            "~"
        } else if old_version.starts_with(">=") {
            ">="
        } else if old_version.starts_with("<=") {
            "<="
        } else if old_version.starts_with('>') {
            ">"
        } else if old_version.starts_with('<') {
            "<"
        } else if old_version.starts_with('=') {
            "="
        } else {
            ""
        };

        // Apply the operator to the new version
        format!("{}{}", operator, new_version)
    }

    /// Save changes back to file
    pub fn save(&self) -> Result<()> {
        std::fs::write(&self.path, self.document.to_string())?;
        Ok(())
    }
}
