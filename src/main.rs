#![deny(clippy::unwrap_used, clippy::panic, clippy::todo)]

use clap::Parser;
use flexi_logger::{FileSpec, Logger, WriteMode};

use termcolor::{ColorChoice, StandardStream};
use tmc::{Cli, Io};

fn main() {
    // writes logs into a file if RUST_LOG is set
    Logger::try_with_env()
        .expect("Failed to initialize logger")
        .log_to_file(FileSpec::default())
        .write_mode(WriteMode::Direct)
        .start()
        .expect("Failed to initialize logging");

    let cli = Cli::parse();
    let mut stdin = std::io::stdin();
    let color = if cli.testmode {
        ColorChoice::Never
    } else {
        ColorChoice::Always
    };
    let mut output = StandardStream::stderr(color);
    let mut io = Io::new(&mut output, &mut stdin);
    tmc::run(cli, &mut io);
}
