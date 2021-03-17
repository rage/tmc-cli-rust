use clap::{App, Arg, ArgMatches, SubCommand};
use std::io::{stdin, stdout};

pub mod config;

pub mod io_module;
use io_module::IoProduction;
pub mod commands;
pub mod interactive;
mod updater;

const PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    let mut stdin = stdin();
    let mut output = stdout();

    let mut io = IoProduction::new(&mut output, &mut stdin);

    let matches = get_matches();
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

fn get_matches() -> ArgMatches<'static> {
    let matches = App::new("Test My Code client written in Rust")
        .version(PKG_VERSION.unwrap())
        .about("Does awesome things")
        .subcommand(
            SubCommand::with_name("config")
                .about("Set/unset TMC-CLI properties and change settings"),
        )
        .subcommand(SubCommand::with_name("courses").about("List the available courses"))
        .subcommand(
            SubCommand::with_name("download")
                .about("Sets the level of verbosity")
                .arg(Arg::with_name("course").value_name("course").required(true))
                .arg(
                    Arg::with_name("download_folder")
                        .value_name("download_folder")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("exercises")
                .about("List the exercises for a specific course")
                .arg(Arg::with_name("course").value_name("course").required(true)),
        )
        .subcommand(SubCommand::with_name("help").about("List every command"))
        .subcommand(SubCommand::with_name("info").about("Show info about the current directory"))
        .subcommand(
            SubCommand::with_name("login")
                .about("Login to TMC server")
                .arg(
                    Arg::with_name("non-interactive")
                        .short("n")
                        .help("Initiates the non-interactive mode.")
                        .long("non-interactive"),
                ),
        )
        .subcommand(SubCommand::with_name("logout").about("Logout from TMC server"))
        .subcommand(
            SubCommand::with_name("organization")
                .about("Change organization")
                .arg(
                    Arg::with_name("non-interactive")
                        .short("n")
                        .help("Initiates the non-interactive mode.")
                        .long("non-interactive"),
                ),
        )
        .subcommand(SubCommand::with_name("paste").about("Submit exercise to TMC pastebin"))
        .subcommand(
            SubCommand::with_name("submit")
                .about("Submit exercises to TMC server")
                .arg(
                    Arg::with_name("dont-block")
                        .value_name("dont-block")
                        .help("Set to avoid blocking.")
                        .long("dont-block"),
                )
                .arg(
                    Arg::with_name("locale")
                        .value_name("locale")
                        .help("Language as a three letter ISO 639-3 code, e.g. 'eng' or 'fin'.")
                        .long("locale")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("submission-path")
                        .value_name("submission-path")
                        .help("Path to the directory where the exercise resides.")
                        .long("submission-path")
                        //.required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("submission-url")
                        .value_name("submission-url")
                        .help("URL where the submission should be posted.")
                        .long("submission-url")
                        //.required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("Run local exercise tests")
                .arg(
                    Arg::with_name("exercise")
                        .value_name("exercise")
                        .required(false),
                ),
        )
        .subcommand(SubCommand::with_name("paste").about("Submit exercise to TMC pastebin"))
        .subcommand(SubCommand::with_name("update").about("Update exercises"))
        .arg(
            Arg::with_name("no-update")
                .short("d")
                .long("no-update")
                .help("Disable auto update temporarily"),
        )
        .arg(
            Arg::with_name("testmode")
                .long("testmode")
                .help("Only for internal testing, disables server connection"),
        )
        .get_matches();

    matches
}
