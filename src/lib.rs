mod cli;
mod client;
mod commands;
mod config;
mod interactive;
mod io;
mod progress_reporting;
#[cfg(test)]
mod test_helper;
#[cfg(target_os = "windows")]
// Updater is used only for windows
// Updates for linux and macos are handled
// via package managers
mod updater;

pub use cli::Cli;
use config::TmcCliConfig;
pub use io::{Io, PrintColor};

pub const PLUGIN: &str = "tmc_cli_rust";
pub const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn run(cli: Cli, io: &mut Io) {
    if let Err(err) = run_inner(io, cli) {
        let error_string = format!("{err:#}");
        log::error!("{error_string}");
        if let Err(err) = io.println(&error_string, PrintColor::Failed) {
            println!(
                "Failed to print error due to error {err}\nThe underlying error was\n{error_string}"
            );
        }
    }
}

fn run_inner(io: &mut Io, cli: Cli) -> anyhow::Result<()> {
    if cli.no_update {
        let os = std::env::consts::OS;
        if os == "windows" {
            #[cfg(target_os = "windows")]
            updater::check_for_update(cli.force_update)?;
        }
    } else {
        println!("No Auto-Updates");
    }
    let config_path = TmcCliConfig::location()?;
    let config = TmcCliConfig::load(config_path)?;
    commands::handle(cli, io, config)
}
