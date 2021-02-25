use clap::{App, Arg, ArgMatches, SubCommand};
//use std::io::{Write, Read};
use self_update::cargo_crate_version;
use std::io::{stdin, stdout};

pub mod config;

pub mod io_module;
use io_module::{Io, IoProduction};

pub mod commands;

fn main() {
    let mut stdin = stdin();
    //let mut input = stdin.lock();
    let mut output = stdout();

    let mut io = IoProduction::new(&mut output, &mut stdin);

    if let Err(_err) = check_for_update() {
        io.println("TMC CLI could not be updated".to_string());
    }

    let matches = get_matches();
    commands::handle(&matches, &mut io);
}

fn get_matches() -> ArgMatches<'static> {
    let matches = App::new("Test My Code client written in Rust")
        .version("0.1.0")
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
        .subcommand(SubCommand::with_name("login").about("Login to TMC server"))
        .subcommand(SubCommand::with_name("logout").about("Logout from TMC server"))
        .subcommand(SubCommand::with_name("organization").about("Change organization"))
        .subcommand(SubCommand::with_name("paste").about("Submit exercise to TMC pastebin"))
        .subcommand(SubCommand::with_name("submit").about("Submit exercises to TMC server"))
        .subcommand(
            SubCommand::with_name("test")
                .about("Run local exercise tests")
                .arg(
                    Arg::with_name("exercise")
                        .value_name("exercise")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("update").about("Update exercises"))
        .subcommand(
            SubCommand::with_name("adssada")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
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
fn check_for_update() -> Result<(), Box<dyn (::std::error::Error)>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("rage")
        .repo_name("tmc-cli-rust")
        .bin_name("github")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}
