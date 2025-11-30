mod cli;
mod models;
mod storage;
mod app;
mod error;
mod config;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("Error: {}", e);
    }
}
