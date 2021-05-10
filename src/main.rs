use clap::{ArgMatches, Shell};
use termcolor::{BufferWriter, ColorChoice};

use std::{
    io,
    io::{stdin, stdout},
};

pub mod io_module;
use io_module::IoProduction;
mod cli;
pub mod commands;
pub mod interactive;
pub mod progress_reporting;

fn main() {
    let cli = cli::build_cli();
    let matches = cli.get_matches();
    if matches.is_present("generate-completions") {
        generate_completions(&matches);
        return;
    }
    let mut stdout = stdout();
    let mut stdin = stdin();

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
            if os == "windows" {}
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

    cli::build_cli().gen_completions_to("tmc", shell, &mut io::stdout());
}
