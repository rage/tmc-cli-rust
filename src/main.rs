#![deny(clippy::unwrap_used, clippy::panic, clippy::todo)]

mod cli;
mod commands;
mod interactive;
mod io;
mod progress_reporting;
// Updater is used only for windows
// Updates for linux and macos are handled
// via package managers
#[cfg(target_os = "windows")]
mod updater;

use clap::Parser;
use cli::Cli;
use io::{Io, IoProduction, PrintColor};
use termcolor::{BufferWriter, ColorChoice};

fn main() {
    let cli = Cli::parse();

    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();

    let mut bufferwriter = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufferwriter.buffer();

    let mut io = IoProduction::new(
        &mut bufferwriter,
        &mut buffer,
        &mut stdout,
        &mut stdin,
        cli.testmode,
    );
    if let Err(err) = run(&mut io, cli) {
        let error_string = format!("{err:?}");
        if let Err(err) = io.println(&error_string, PrintColor::Failed) {
            println!(
                "Failed to print error due to error {err}\nThe underlying error was {error_string}"
            );
        }
    }
}

fn run(io: &mut IoProduction, cli: Cli) -> anyhow::Result<()> {
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
