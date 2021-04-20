use std::io::{stdin, stdout};

pub mod io_module;
use io_module::IoProduction;
mod cli;
pub mod commands;
pub mod interactive;
pub mod progress_reporting;
mod updater;

fn main() {
    let mut stdin = stdin();
    let mut output = stdout();

    let mut io = IoProduction::new(&mut output, &mut stdin);

    let matches = cli::build_cli().get_matches();
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
