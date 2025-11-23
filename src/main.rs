use cargo_check_updates::{
    cli::{Cli, Commands},
    commands::CheckCommand,
    Result,
};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle the command
    match cli.command {
        Some(Commands::Check { outdated: _ }) | None => {
            // Default command: check for updates
            let check_cmd = CheckCommand::new(
                cli.manifest_path,
                cli.upgrade,
                cli.interactive,
                cli.filter,
                cli.reject,
            );
            check_cmd.run().await?;
        }
    }

    Ok(())
}
