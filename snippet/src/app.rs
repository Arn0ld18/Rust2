use crate::cli::{Cli, Commands};
use crate::storage::get_storage;
use crate::error::AppError;
use chrono::Utc;
use std::io::{self, Read};
use clap::Parser;

pub fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    let storage = get_storage()?;

    match cli.command {
        Commands::Add { name } => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            let mut all = storage.load_all()?;
            all.insert(
                name.clone(),
                crate::models::Snippet { code: buffer.trim().to_string(), created_at: Utc::now() },
            );
            storage.save_all(&all)?;
            println!("Snippet '{}' saved.", name);
        }
        Commands::Read { name } => {
            let all = storage.load_all()?;
            if let Some(snippet) = all.get(&name) {
                println!("-- created_at: {}", snippet.created_at);
                println!("{}", snippet.code);
            } else {
                return Err(AppError::NotFound(name));
            }
        }
        Commands::Delete { name } => {
            storage.delete(&name)?;
            println!("Snippet '{}' deleted.", name);
        }
    }

    Ok(())
}
