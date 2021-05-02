use clap;
//use clap_generate::{generate_to, generators::Zsh};
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let _outdir = match std::env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut _app = build_cli();
    //generate_to::<Zsh, _, _>(&mut app, "tmc", outdir);

    Ok(())
}
