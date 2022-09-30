use crate::cli::{Cli, ShellArg};
use clap::CommandFactory;
use clap_complete::Shell;

pub fn generate(shell: ShellArg) {
    let shell = match shell {
        ShellArg::Bash => Shell::Bash,
        ShellArg::Zsh => Shell::Zsh,
        ShellArg::Powershell => Shell::PowerShell,
    };

    let mut cli = Cli::command();
    clap_complete::generate(shell, &mut cli, "tmc", &mut std::io::stdout());
}
