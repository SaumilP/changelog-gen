use clap::{ArgGroup, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate and maintain changelogs from git history")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    New {
        #[arg(long, default_value = "CHANGELOG.md")]
        file: PathBuf,
        #[arg(long, default_value = "markdown")]
        format: String,
    },
    Validate {
        #[arg(long, default_value = "CHANGELOG.md")]
        file: PathBuf,
        #[arg(long)]
        strict: bool,
    },
    Generate {
        #[arg(long, default_value = "CHANGELOG.md")]
        file: PathBuf,
        #[arg(long)]
        since: Option<String>,
        #[arg(long)]
        until: Option<String>,
        #[arg(long)]
        specific: Option<String>,
        #[arg(long)]
        milestone: Option<String>,
        #[arg(long)]
        github: bool,
        #[arg(long)]
        template: Option<PathBuf>,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long)]
        map: Option<PathBuf>,
    },
    #[command(group(
        ArgGroup::new("versioning")
            .required(true)
            .args(["version", "bump"])
    ))]
    Release {
        #[arg(long)]
        version: Option<String>,
        #[arg(long)]
        bump: Option<String>,
        #[arg(long, default_value = "CHANGELOG.md")]
        file: PathBuf,
        #[arg(long, default_value = "default")]
        header: String,
        #[arg(long = "override")]
        override_existing: bool,
    },
    Show {
        #[arg(long, default_value = "CHANGELOG.md")]
        file: PathBuf,
        #[arg(long)]
        version: Option<String>,
        #[arg(long)]
        range: Option<String>,
        #[arg(long)]
        converge: bool,
    },
    Remove {
        #[arg(long)]
        version: String,
        #[arg(long, default_value = "CHANGELOG.md")]
        file: PathBuf,
        #[arg(long)]
        yes: bool,
    },
}
