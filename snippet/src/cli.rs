use clap::{Parser, Subcommand};
use url::Url;

/// Command-line interface definition for the snippet application.
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Supported subcommands of the snippet CLI.
#[derive(Subcommand)]
pub enum Commands {
    /// Add a snippet, from stdin or downloaded from URL.
    Add {
        /// Name of the snippet.
        #[arg(long)]
        name: String,
        /// Optional URL for downloading snippet contents.
        #[arg(long)]
        download: Option<Url>,
    },
    /// Read a snippet by name.
    Read {
        /// Name of the snippet.
        #[arg(long)]
        name: String,
    },
    /// Remove a snippet from storage.
    Delete {
        /// Name of the snippet.
        #[arg(long)]
        name: String,
    },
}
