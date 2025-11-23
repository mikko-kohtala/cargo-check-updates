use colored::Colorize;
use semver::Version;

pub struct OutputFormatter;

impl OutputFormatter {
    /// Format a version update with color coding based on semver
    pub fn format_update(
        crate_name: &str,
        current: &Version,
        latest: &Version,
    ) -> String {
        let arrow = "→";

        let colored_latest = if latest.major > current.major {
            // Major version update - red
            latest.to_string().red().bold()
        } else if latest.minor > current.minor {
            // Minor version update - cyan
            latest.to_string().cyan()
        } else {
            // Patch version update - green
            latest.to_string().green()
        };

        format!(
            "{:<30} {} {} {}",
            crate_name.bold(),
            current,
            arrow,
            colored_latest
        )
    }

    /// Print a summary header
    pub fn print_header(total: usize, outdated: usize) {
        println!("\n{}", "Dependency Check Results".bold().underline());
        println!("Total dependencies: {}", total);
        println!("Outdated dependencies: {}\n", outdated.to_string().yellow());
    }

    /// Print update legend
    pub fn print_legend() {
        println!("\n{}", "Legend:".bold());
        println!("  {} Major version update", "Red".red());
        println!("  {} Minor version update", "Cyan".cyan());
        println!("  {} Patch version update", "Green".green());
    }
}
