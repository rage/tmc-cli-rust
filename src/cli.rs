use clap::{App, AppSettings, Arg};

const PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub fn build_cli() -> App<'static> {
    App::new("Test My Code client written in Rust")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(PKG_VERSION.unwrap())
        .about("Client for downloading, testing and submitting exercises through the Test My Code system")
        .subcommand(App::new("courses").about("List the available courses"))
        .subcommand(
            App::new("download")
                .about("Downloads course exercises")
                .arg(
                    Arg::new("course")
                        .short('c')
                        .long("course")
                        .value_name("course name")
                        .required(false),
                )
                .arg(
                    Arg::new("currentdir")
                        .short('d')
                        .long("currentdir")
                        .required(false),
                ),
        )
        .subcommand(
            App::new("exercises")
                .about("List the exercises for a specific course")
                .arg(Arg::new("course").value_name("course").required(true)),
        )
        .subcommand(
            App::new("login")
                .about("Login to TMC server")
                .arg(
                    Arg::new("non-interactive")
                        .short('n')
                        .about("Initiates the non-interactive mode.")
                        .long("non-interactive"),
                ),
        )
        .subcommand(App::new("logout").about("Logout from TMC server"))
        .subcommand(
            App::new("organization")
                .about("Change organization")
                .arg(
                    Arg::new("non-interactive")
                        .short('n')
                        .about("Initiates the non-interactive mode.")
                        .long("non-interactive"),
                ),
        )
        .subcommand(
            App::new("paste")
                .about("Submit exercise to TMC pastebin")
                .arg(
                    Arg::new("exercise")
                        .value_name("exercise")
                        .required(false),
                ),
        )
        .subcommand(
            App::new("submit")
                .about("Submit exercises to TMC server")
                .arg(
                    Arg::new("exercise")
                        .value_name("exercise")
                        .required(false),
                ),
        )
        .subcommand(
            App::new("test")
                .about("Run local exercise tests")
                .arg(
                    Arg::new("exercise")
                        .value_name("exercise")
                        .required(false),
                ),
        )
        .subcommand(
            App::new("fetchupdate")
                .setting(AppSettings::Hidden)
                .about("Finishes the autoupdater. Administator rights needed."),
        )
        .subcommand(
            App::new("cleartemp")
                .setting(AppSettings::Hidden)
                .about("Removes tempfiles. Administator rights needed."),
        )
        .subcommand(
            App::new("elevateddownload")
                .setting(AppSettings::Hidden)
                .about("Downloads course from the tempfile. Administator rights needed."),
        )
        .subcommand(App::new("update").about("Update exercises"))
        .arg(
            Arg::new("no-update")
                .short('d')
                .long("no-update")
                .about("Disable auto update temporarily"),
        )
        .arg(
            Arg::new("testmode")
                .long("testmode")
                .about("Only for internal testing, disables server connection"),
        )
}
