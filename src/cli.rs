use clap::{AppSettings, Arg, Command};

const PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub fn build_cli() -> Command<'static> {
    Command::new("Test My Code client written in Rust")
        .arg_required_else_help(true)
        .version(PKG_VERSION.unwrap())
        .about("Client for downloading, testing and submitting exercises through the Test My Code system")
        .subcommand(Command::new("courses").about("List the available courses"))
        .subcommand(
            Command::new("download")
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
            Command::new("exercises")
            .about("List the exercises for a specific course")
            .arg(Arg::new("course").value_name("course").required(true)),
            )
        .subcommand(
            Command::new("login")
            .about("Login to TMC server")
            .arg(
                Arg::new("non-interactive")
                .short('n')
                .help("Initiates the non-interactive mode.")
                .long("non-interactive"),
                ),
                )
        .subcommand(Command::new("logout").about("Logout from TMC server"))
        .subcommand(
            Command::new("organization")
            .about("Change organization")
            .arg(
                Arg::new("non-interactive")
                .short('n')
                .help("Initiates the non-interactive mode.")
                .long("non-interactive"),
                ),
                )
        .subcommand(
            Command::new("paste")
            .about("Submit exercise to TMC pastebin")
            .arg(
                Arg::new("exercise")
                .value_name("exercise")
                .required(false),
                ),
                )
        .subcommand(
            Command::new("submit")
            .about("Submit exercises to TMC server")
            .arg(
                Arg::new("exercise")
                .value_name("exercise")
                .required(false),
                ),
                )
        .subcommand(
            Command::new("test")
            .about("Run local exercise tests")
            .arg(
                Arg::new("exercise")
                .value_name("exercise")
                .required(false),
                ),
                )
        .subcommand(
            Command::new("fetchupdate")
            .hide(true)
            .about("Finishes the autoupdater. Administator rights needed."),
            )
        .subcommand(
            Command::new("cleartemp")
            .hide(true)
            .about("Removes tempfiles. Administator rights needed."),
            )
        .subcommand(
            Command::new("elevateddownload")
                .hide(true)
                .about("Downloads course from the tempfile. Administator rights needed."),
        )
        .subcommand(
            Command::new("elevatedupdate")
                .hide(true)
                .about("updates course from the tempfile. Administator rights needed."),
        )
        .subcommand(
            Command::new("update")
                .about("Updates course exercises")
                .arg(
                    Arg::new("currentdir")
                        .short('d')
                        .long("currentdir")
                        .required(false),
                ),
        )
        .subcommand(Command::new("update").about("Update exercises"))
        .arg(
            Arg::new("no-update")
            .short('d')
            .long("no-update")
            .help("Disable auto update temporarily"),
            )
        .arg(
            Arg::new("testmode")
            .long("testmode")
            .help("Only for internal testing, disables server connection"),
            )
        .subcommand(Command::new("generate-completions")
                    .override_usage("tmc generate_completions --[your shell] > /path/to/your/completions/folder")
                    .about("Generate completion scripts for command line usage.")
                    .disable_version_flag(true)
                    .hide(true)
                    .setting(AppSettings::DeriveDisplayOrder)
                    .arg(
                        Arg::new("bash")
                        .short('b')
                        .long("bash"))
                    .arg(
                        Arg::new("zsh")
                        .short('z')
                        .long("zsh")
                        )
                    .arg(
                        Arg::new("powershell")
                        .short('p')
                        .long("powershell"))
                    )
}
