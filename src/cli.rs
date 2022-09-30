use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version,
    author,
    about,
    subcommand_required(true),
    arg_required_else_help(true)
)]
pub struct Cli {
    /// Disable auto update temporarily
    #[arg(short = 'd', long, hide = !cfg!(windows))]
    pub no_update: bool,
    /// Force auto update to run
    #[arg(short = 'u', long, hide = !cfg!(windows))]
    pub force_update: bool,
    /// Only for internal testing, disables server connection
    #[arg(long, hide = true)]
    pub testmode: bool,
    #[command(subcommand)]
    pub subcommand: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// List the available courses
    Courses,
    /// Downloads course exercises
    Download {
        #[arg(short, long, value_name = "course name")]
        course: Option<String>,
        #[arg(short = 'd', long)]
        currentdir: bool,
    },
    /// List the exercises for a specific course
    Exercises { course: String },
    /// Login to TMC server
    Login {
        /// Initiates the non-interactive mode.
        #[arg(short, long)]
        non_interactive: bool,
    },
    /// Logout from TMC server
    Logout,
    /// Change organization
    Organization {
        /// Initiates the non-interactive mode.
        #[arg(short, long)]
        non_interactive: bool,
    },
    /// Submit exercise to TMC pastebin
    Paste { exercise: Option<String> },
    /// Submit exercises to TMC server
    Submit { exercise: Option<String> },
    /// Run local exercise tests
    Test { exercise: Option<String> },
    /// Finishes the autoupdater. Administator rights needed.
    #[clap(hide = true)]
    Fetchupdate,
    /// Removes tempfiles. Administator rights needed.
    #[clap(hide = true)]
    Cleartemp,
    /// Downloads course from the tempfile. Administator rights needed.
    #[clap(hide = true)]
    Elevateddownload,
    /// updates course from the tempfile. Administator rights needed.
    #[clap(hide = true)]
    Elevatedupdate,
    /// Updates course exercises
    Update {
        #[arg(short = 'd', long)]
        currentdir: bool,
    },
    /// Generate completion scripts for command line usage.
    #[clap(
        hide = true,
        disable_version_flag = true,
        override_usage = "tmc generate_completions --[your shell] > /path/to/your/completions/folder"
    )]
    GenerateCompletions { shell: ShellArg },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ShellArg {
    Bash,
    Zsh,
    Powershell,
}
