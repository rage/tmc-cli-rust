use clap;
use test_command::test;
mod test_command;

pub fn handle(matches: &clap::ArgMatches) {
    // if the subcommand is chosen, extract it from the ArgMatches
    if let Some(ref matches) = matches.subcommand_matches("test") {
        // if the path to the exercise was given (which should have been), the test function is
        // called
        if let Some(exercise_path) = matches.value_of("exercise") {
            test(exercise_path);
        }
    }
}
