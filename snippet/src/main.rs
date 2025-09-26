use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

const FILE_PATH: &str = "snippets.json";

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// A set of commands for managing items.
#[derive(Subcommand)]
enum Commands {
    /// Add a new item.
    Add {
        /// The name of the item to add.
        #[arg(long)]
        name: String,
    },
    /// Read (view) an existing item.
    Read {
        /// The name of the item to read.
        #[arg(long)]
        name: String,
    },
    /// Delete an existing item.
    Delete {
        /// The name of the item to delete.
        #[arg(long)]
        name: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Snippets(HashMap<String, String>);

impl Snippets {
    fn load() -> Self {
        if Path::new(FILE_PATH).exists() {
            let data = fs::read_to_string(FILE_PATH).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_else(|_| Snippets(HashMap::new()))
        } else {
            Snippets(HashMap::new())
        }
    }

    fn save(&self) {
        let data = serde_json::to_string_pretty(&self).unwrap();
        fs::write(FILE_PATH, data).unwrap();
    }
}

pub fn part3() {
    let cli = Cli::parse();
    let mut snippets = Snippets::load();

    match cli.command {
        Commands::Add { name } => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            snippets.0.insert(name.clone(), buffer.trim().to_string());
            snippets.save();
            println!("Snippet '{}' saved.", name);
        }
        Commands::Read { name } => {
            match snippets.0.get(&name) {
                Some(code) => println!("{}", code),
                None => eprintln!("Snippet '{}' not found.", name),
            }
        }
        Commands::Delete { name } => {
            if snippets.0.remove(&name).is_some() {
                snippets.save();
                println!("Snippet '{}' deleted.", name);
            } else {
                eprintln!("Snippet '{}' not found.", name);
            }
        }
    }
}

fn main(){
    part3();
}