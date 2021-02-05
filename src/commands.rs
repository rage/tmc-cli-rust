use login_command::login;
mod login_command;

use crate::io_module::IO;

pub fn handle(matches: &clap::ArgMatches, io: &mut IO) {
    //println!("{:#?}", matches.subcommand());

    match matches.subcommand() {
        ("login", _) => login(io),
        (_, Some(_)) => (), // Not implemented yet
        (_, None) => (),    // No subcommand was given
    }
}
