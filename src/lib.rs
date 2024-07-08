use commands::{GiveStar, GoldStarCommand, Stars};
use serenity::all::CreateCommand;

pub mod commands;
pub mod error;
pub mod manager;

pub use error::{Error, Result};

pub fn register() -> Vec<CreateCommand> {
    vec![GiveStar::register(), Stars::register()]
}
