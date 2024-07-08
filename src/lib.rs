use commands::{GiveStar, GoldStarCommand, Stars};
use serenity::all::CreateCommand;

mod commands;
mod error;
mod manager;

pub use error::{Error, Result};

pub fn register() -> Vec<CreateCommand> {
    vec![GiveStar::register(), Stars::register()]
}
