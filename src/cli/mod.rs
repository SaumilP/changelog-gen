use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Generate {
        #[arg(long)]
        release: Option<String>,

        #[arg(long)]
        github: bool,
    },
    Release {
        #[arg(long)]
        bump: Option<String>, // major/minor/patch
    },
}
