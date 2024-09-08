#![deny(clippy::unwrap_used, clippy::panic, clippy::todo)]

use clap::Parser;
use std::fs::File;
use termcolor::{ColorChoice, StandardStream};
use tmc::{Cli, Io};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

fn main() {
    let cli = Cli::parse();
    let mut stdin = std::io::stdin();
    let color = if cli.testmode {
        ColorChoice::Never
    } else {
        ColorChoice::Always
    };
    let mut output = StandardStream::stderr(color);
    let mut io = Io::new(&mut output, &mut stdin);

    let _guard_storage;
    // env_filter should be OFF if RUST_LOG is not set
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::OFF.into())
        .from_env()
        .expect("Default directive should be set");
    // we don't want to create a log file if logging is not enabled
    if env_filter
        .max_level_hint()
        .map(|mlh| mlh != LevelFilter::OFF)
        .unwrap_or_default()
    {
        match File::create("./tmc-cli-rust.log") {
            Ok(file) => {
                let (non_blocking, guard) = tracing_appender::non_blocking(file);
                _guard_storage = guard;
                tracing_subscriber::fmt()
                    .with_writer(non_blocking)
                    .with_env_filter(env_filter)
                    .pretty()
                    .with_ansi(false)
                    .init();
            }
            Err(err) => {
                io.println(
                    &format!("Failed to create log file: {err}"),
                    tmc::PrintColor::Failed,
                )
                .ok();
            }
        }
    }
    let mut io = Io::new(&mut output, &mut stdin);
    tmc::run(cli, &mut io);
}
