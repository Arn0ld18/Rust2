use clap::{Parser, Subcommand};
use url::Url;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(long)]
        name: String,
          #[arg(long)]
        download: Option<Url>,
    },
    Read { #[arg(long)] name: String },
    Delete { #[arg(long)] name: String },
}
