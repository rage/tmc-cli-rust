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
    #[command(subcommand)]
    pub subcommand: Command,

    /// Disable auto-update temporarily.
    #[arg(short = 'd', long, hide = !cfg!(windows))]
    pub no_update: bool,
    /// Force auto-update to run.
    #[arg(short = 'u', long, hide = !cfg!(windows))]
    pub force_update: bool,

    /// Only for internal testing, disables server connection.
    #[arg(long, hide = true)]
    pub testmode: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Download exercises for a course.
    Download {
        /// If set, this organization is pre-selected for downloading TMC exercises.
        #[arg(short, long, value_name = "TMC organization")]
        organization: Option<String>,
        /// If set, the exercises of this course are downloaded. If not set, the selection is done from an interactive menu.
        #[arg(short, long, value_name = "course name")]
        course: Option<String>,
        /// If set, exercises are downloaded to the current working directory.
        #[arg(short = 'd', long)]
        currentdir: bool,
    },
    /// Login to TMC server.
    Login,
    /// Logout from TMC server.
    Logout,
    /// Submit exercise to TMC pastebin.
    Paste { exercise: Option<String> },
    /// Submit exercises to TMC server.
    Submit { exercise: Option<String> },
    /// Run local exercise tests.
    Test { exercise: Option<String> },
    /// Updates course exercises.
    Update {
        /// If set, exercises in the current working directory are updated.
        #[arg(short = 'd', long)]
        currentdir: bool,
    },

    // hidden commands
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
