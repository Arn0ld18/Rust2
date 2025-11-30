pub mod app;
pub mod cli;
pub mod config;
pub mod error;
pub mod models;
pub mod storage;

fn main() {
    if let Err(e) = snippet::app::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
