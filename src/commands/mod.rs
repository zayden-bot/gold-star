use async_trait::async_trait;
use serenity::all::{CommandInteraction, CreateCommand};
use sqlx::{Database, Pool};

use crate::{manager::GoldStarManager, Result};

mod give_star;
mod stars;

pub use give_star::GiveStar;
pub use stars::Stars;

#[async_trait]
pub trait GoldStarCommand<T> {
    async fn run<Db: Database, Manager: GoldStarManager<Db>>(
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<T>;

    fn register() -> CreateCommand;
}
