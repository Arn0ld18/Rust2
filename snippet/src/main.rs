pub mod cli;
pub mod models;
pub mod storage;
pub mod app;
pub mod error;
pub mod config;

fn main() {
    if let Err(e) = snippet::app::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
