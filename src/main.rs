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

use clap::ArgMatches;
use clap_complete::Shell;
use io::{Io, IoProduction, PrintColor};
use termcolor::{BufferWriter, ColorChoice};

fn main() {
    let cli = cli::build_cli();
    let matches = cli.get_matches();
    if let Some(matches) = matches.subcommand_matches("generate-completions") {
        generate_completions(matches);
        return;
    }
    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();

    let mut bufferwriter = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufferwriter.buffer();

    let mut io = IoProduction::new(
        &mut bufferwriter,
        &mut buffer,
        &mut stdout,
        &mut stdin,
        matches.contains_id("testmode"),
    );
    if let Err(err) = run(&mut io, &matches) {
        let error_string = err
            .chain()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n  caused by: ");
        if let Err(err) = io.println(&error_string, PrintColor::Failed) {
            println!("Failed to print error: {err}");
        }
    }
}

fn run(io: &mut IoProduction, matches: &ArgMatches) -> anyhow::Result<()> {
    match matches.get_count("no-update") {
        0 => {
            let os = std::env::consts::OS;
            if os == "windows" {
                #[cfg(target_os = "windows")]
                updater::check_for_update(matches.is_present("force-update"))?;
            }
        }
        _ => println!("No Auto-Updates"),
    }
    commands::handle(matches, io)
}

fn generate_completions(matches: &ArgMatches) {
    let shell = if matches.contains_id("bash") {
        Shell::Bash
    } else if matches.contains_id("zsh") {
        Shell::Zsh
    } else if matches.contains_id("powershell") {
        Shell::PowerShell
    } else {
        return;
    };

    let mut cmd = cli::build_cli();
    clap_complete::generate(shell, &mut cmd, "tmc", &mut std::io::stdout());
}
