use crate::cli::{Cli, Commands};
use crate::storage::get_storage;
use crate::error::AppError;
use crate::config::AppConfig;
use chrono::Utc;
use std::io::{self, Read};
use clap::Parser;
use tracing::{info, error};
use tracing_subscriber::{fmt, prelude::*};
use tracing_subscriber::{ EnvFilter};
use std::sync::Mutex;

pub fn init_logging(config: &AppConfig) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.log_level));

    if let Some(path) = &config.log_path {
        let file = std::fs::File::create(path).expect("Cannot create log file");
        let file = Mutex::new(file);

        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .with_writer(file)
            )
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer())
            .init();
    }
}

pub fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    let config_file = std::env::var("CONF_FILE").unwrap_or_else(|_| "config.toml".to_string());
    let config = AppConfig::load(&config_file).unwrap_or_default();
    init_logging(&config);

    let storage = get_storage()?;

    match cli.command {
         Commands::Add { name, download } => {
        let code = if let Some(url) = download {
            info!("Downloading snippet from {}", url);
            reqwest::blocking::get(url.as_str())?.text()?
        } else {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        };

        let mut all = storage.load_all()?;
        all.insert(
            name.clone(),
            crate::models::Snippet {
                code: code.trim().to_string(),
                created_at: Utc::now(),
            },
        );
        storage.save_all(&all)?;
        info!("Snippet '{}' saved.", name);
        println!("Snippet '{}' saved.", name);
    }
        Commands::Read { name } => {
            let all = storage.load_all()?;
            if let Some(snippet) = all.get(&name) {
                println!("-- created_at: {}", snippet.created_at);
                println!("{}", snippet.code);
            } else {
                error!("Snippet '{}' not found", name);
                return Err(AppError::NotFound(name));
            }
        }
        Commands::Delete { name } => {
            storage.delete(&name)?;
            info!("Snippet '{}' deleted.", name);
            println!("Snippet '{}' deleted.", name);
        }
    }

    Ok(())
}
