#![deny(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    rustdoc::broken_intra_doc_links,
    unreachable_pub
)]

//! Snippet manager CLI application.
//!
//! This crate provides all functionality for storing, reading,
//! deleting, and downloading small code snippets using JSON or SQLite
//! storage backends.

/// Command-line interface definitions and argument parsing.
pub mod cli;

/// Data models used by the application (e.g., `Snippet`).
pub mod models;

/// Storage backends for snippets (JSON and SQLite).
pub mod storage;

/// Main application logic: command handling and execution.
pub mod app;

/// Error types used across the application.
pub mod error;

/// Application configuration loading and parsing.
pub mod config;
