mod cli;
mod commands;
mod config;
mod interactive;
mod io;
mod progress_reporting;
// Updater is used only for windows
// Updates for linux and macos are handled
// via package managers
#[cfg(target_os = "windows")]
mod updater;

pub use cli::Cli;
pub use io::{Io, IoProduction, PrintColor};

pub fn run(cli: Cli, io: &mut dyn Io) {
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

fn run_inner(io: &mut dyn Io, cli: Cli) -> anyhow::Result<()> {
    if cli.no_update {
        let os = std::env::consts::OS;
        if os == "windows" {
            #[cfg(target_os = "windows")]
            updater::check_for_update(cli.force_update)?;
        }
    } else {
        println!("No Auto-Updates");
    }
    commands::handle(cli, io)
}
