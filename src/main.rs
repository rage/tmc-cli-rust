#![deny(clippy::unwrap_used, clippy::panic, clippy::todo)]

use clap::Parser;
use flexi_logger::{FileSpec, Logger, WriteMode};
use termcolor::{BufferWriter, ColorChoice};
use tmc::{Cli, IoProduction};

fn main() {
    // writes logs into a file if RUST_LOG is set
    Logger::try_with_env()
        .expect("Failed to initialize logger")
        .log_to_file(FileSpec::default())
        .write_mode(WriteMode::Direct)
        .start()
        .expect("Failed to initialize logging");

    let cli = Cli::parse();
    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();
    let mut bufferwriter = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufferwriter.buffer();
    let mut io = IoProduction::new(
        &mut bufferwriter,
        &mut buffer,
        &mut stdout,
        &mut stdin,
        cli.testmode,
    );
    tmc::run(cli, &mut io);
}
