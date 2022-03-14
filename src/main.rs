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
use io::IoProduction;
use termcolor::{BufferWriter, ColorChoice};

fn main() {
    let cli = cli::build_cli();
    let matches = cli.get_matches();
    if matches.subcommand_name() == Some("generate-completions") {
        generate_completions(&matches);
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
        matches.is_present("testmode"),
    );

    match matches.occurrences_of("no-update") {
        0 => {
            let os = std::env::consts::OS;
            if os == "windows" {
                #[cfg(target_os = "windows")]
                updater::check_for_update();
            }
        }
        _ => println!("No Auto-Updates"),
    }
    commands::handle(&matches, &mut io);
}

fn generate_completions(matches: &ArgMatches) {
    let matches = matches.subcommand_matches("generate-completions").unwrap();
    let shell = if matches.is_present("bash") {
        Shell::Bash
    } else if matches.is_present("zsh") {
        Shell::Zsh
    } else if matches.is_present("powershell") {
        Shell::PowerShell
    } else {
        return;
    };

    let mut cmd = cli::build_cli();
    clap_complete::generate(shell, &mut cmd, "tmc", &mut std::io::stdout());
}
