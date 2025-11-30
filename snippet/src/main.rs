mod cli;
mod models;
mod storage;
mod app;
mod error;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("Error: {}", e);
    }
}
