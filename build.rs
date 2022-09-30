use clap::CommandFactory;
use clap_complete::Shell;

include!("src/cli.rs");

fn main() {
    let outdir = std::env::var("OUT_DIR").unwrap();
    let mut app = Cli::command();
    clap_complete::generate_to(Shell::Bash, &mut app, "tmc", &outdir).unwrap();
    clap_complete::generate_to(Shell::PowerShell, &mut app, "tmc", &outdir).unwrap();
    clap_complete::generate_to(Shell::Zsh, &mut app, "tmc", &outdir).unwrap();
}
