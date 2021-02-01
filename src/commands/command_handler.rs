use clap;

pub fn handle(matches: &clap::ArgMatches) {
    if let Some(ref matches) = matches.subcommand_matches("test") {
        println!("Testing {:?}", matches.usage());
    }
}
