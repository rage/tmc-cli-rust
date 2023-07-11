use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

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
    // tmc commands
    /// List the available courses.
    Courses,
    /// Download exercises for a course.
    Download {
        /// If set, the exercises of this course are downloaded. If not set, the selection is done from an interactive menu.
        #[arg(short, long, value_name = "course name")]
        course: Option<String>,
        /// If set, exercises are downloaded to the current working directory.
        #[arg(short = 'd', long)]
        currentdir: bool,
    },
    /// List the exercises for a specific course.
    Exercises {
        /// If set, the exercises of this course are listed. If not set, the selection is done from an interactive menu.
        course: Option<String>,
    },
    /// Login to TMC server.
    Login {
        /// Initiates the non-interactive mode.
        #[arg(short, long)]
        non_interactive: bool,
    },
    /// Logout from TMC server.
    Logout,
    /// Change organization.
    Organization {
        /// Initiates the non-interactive mode.
        #[arg(short, long)]
        non_interactive: bool,
    },
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

    // MOOC commands
    /// Currently enrolled courses.mooc.fi courses.
    MoocCourses,
    /// Active exercises of the selected course.
    MoocCourseExercises {
        /// If set, the exercises of this course are listed. If not set, the selection is done from an interactive menu.
        course: Option<String>,
    },
    /// Downloads active exercises for the selected course.
    MoocDownloadExercises {
        /// If set, the exercises of this course are downloaded. If not set, the selection is done from an interactive menu.
        course: Option<String>,
        /// If set, exercises are downloaded to the current working directory.
        #[arg(short = 'd', long)]
        currentdir: bool,
    },
    /// Updates local exercises for the selected course.
    MoocUpdateExercises {
        /// If set, the exercises of this course are downloaded. If not set, the selection is done from an interactive menu.
        course: Option<String>,
    },
    /// Submits an exercise.
    MoocSubmitExercise {
        /// If set, the exercise at this path is submitted. If not set, the selection is done from an interactive menu.
        path: Option<PathBuf>,
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

impl Command {
    pub fn requires_organization_set(&self) -> bool {
        matches!(self, Command::Download { .. } | Command::Courses { .. })
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ShellArg {
    Bash,
    Zsh,
    Powershell,
}
