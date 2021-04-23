use termcolor::{BufferWriter, ColorChoice};

use std::io::{stdin, stdout};

pub mod io_module;
use io_module::IoProduction;
mod cli;
pub mod commands;
pub mod interactive;
pub mod progress_reporting;
mod updater;

fn main() {
    let matches = cli::build_cli().get_matches();

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
            if os == "windows" {
                updater::check_for_update();
            }
        }
        _ => println!("No Auto-Updates"),
    }
    commands::handle(&matches, &mut io);
}
