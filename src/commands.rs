use login_command::login;
mod login_command;

pub fn handle(matches: &clap::ArgMatches) {
    println!("{:#?}", matches.subcommand());

    match matches.subcommand() {
        ("login", _) => login(),
        (_, None) => (), // No subcommand was given
        _ => unreachable!(),
    }
}
