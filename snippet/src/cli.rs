use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { #[arg(long)] name: String },
    Read { #[arg(long)] name: String },
    Delete { #[arg(long)] name: String },
}