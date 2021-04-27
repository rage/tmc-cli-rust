use clap::{App, AppSettings, Arg, SubCommand};

const PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub fn build_cli() -> App<'static, 'static> {
    App::new("Test My Code client written in Rust")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(PKG_VERSION.unwrap())
        .about("Client for downloading, testing and submitting exercises through the Test My Code system")
        .subcommand(SubCommand::with_name("courses").about("List the available courses"))
        .subcommand(
            SubCommand::with_name("download")
                .about("Downloads course exercises")
                .arg(
                    Arg::with_name("course")
                        .short("c")
                        .long("course")
                        .value_name("course name")
                        .required(false),
                )
                .arg(
                    Arg::with_name("currentdir")
                        .short("d")
                        .long("currentdir")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("exercises")
                .about("List the exercises for a specific course")
                .arg(Arg::with_name("course").value_name("course").required(true)),
        )
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
        .subcommand(
            SubCommand::with_name("paste")
                .about("Submit exercise to TMC pastebin")
                .arg(
                    Arg::with_name("exercise")
                        .value_name("exercise")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("submit")
                .about("Submit exercises to TMC server")
                .arg(
                    Arg::with_name("exercise")
                        .value_name("exercise")
                        .required(false),
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
        .subcommand(
            SubCommand::with_name("fetchupdate")
                .setting(AppSettings::Hidden)
                .about("Finishes the autoupdater. Administator rights needed."),
        )
        .subcommand(
            SubCommand::with_name("cleartemp")
                .setting(AppSettings::Hidden)
                .about("Removes tempfiles. Administator rights needed."),
        )
        .subcommand(
            SubCommand::with_name("elevateddownload")
                .setting(AppSettings::Hidden)
                .about("Downloads course from the tempfile. Administator rights needed."),
        )
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
        
}
