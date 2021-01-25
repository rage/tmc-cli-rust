use clap::{App, Arg, SubCommand};

fn main() {
    println!("Hello");
    let _matches = App::new("Test My Code client written in Rust")
        .about("Does awesome things")
        .subcommand(
            SubCommand::with_name("config")
                .help("Set/unset TMC-CLI properties and change settings"),
        )
        .subcommand(SubCommand::with_name("courses").help("List the available courses"))
        .subcommand(SubCommand::with_name("download").help("Sets the level of verbosity"))
        .subcommand(
            SubCommand::with_name("exercises").help("List the exercises for a specific course"),
        )
        .subcommand(SubCommand::with_name("help").help("List every command"))
        .subcommand(SubCommand::with_name("info").help("Show info about the current directory"))
        .subcommand(SubCommand::with_name("login").help("Login to TMC server"))
        .subcommand(SubCommand::with_name("logout").help("Logout from TMC server"))
        .subcommand(SubCommand::with_name("organization").help("Change organization"))
        .subcommand(SubCommand::with_name("paste").help("Submit exercise to TMC pastebin"))
        .subcommand(SubCommand::with_name("submit").help("Submit exercises to TMC server"))
        .subcommand(SubCommand::with_name("test").help("Run local exercise tests"))
        .subcommand(SubCommand::with_name("update").help("Update exercises"))
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
        .get_matches();
}
