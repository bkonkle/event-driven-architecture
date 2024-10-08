//! A demo project for a simple CQRS/ES workflow

/// Event domains
pub mod domains;

/// Event publishers
pub mod publishers;

/// Event projectors
pub mod projectors;

/// Utils
pub mod utils;

#[macro_use]
extern crate log;
