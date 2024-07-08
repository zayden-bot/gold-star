use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption, ResolvedValue,
};
use sqlx::{Database, Pool};
use zayden_core::parse_options;

use crate::manager::{GoldStarManager, GoldStarRow};
use crate::Result;

use super::GoldStarCommand;

pub struct Stars;

#[async_trait]
impl GoldStarCommand<(String, GoldStarRow)> for Stars {
    async fn run<Db: Database, Manager: GoldStarManager<Db>>(
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<(String, GoldStarRow)> {
        let options = interaction.data.options();
        let options = parse_options(&options);

        let user = match options.get("user") {
            Some(ResolvedValue::User(user, _)) => *user,
            _ => &interaction.user,
        };

        let row = match Manager::get_row(pool, user.id).await? {
            Some(row) => row,
            None => GoldStarRow::new(user.id),
        };

        let username = user.global_name.as_deref().unwrap_or(&user.name);

        Ok((username.to_string(), row))
    }

    fn register() -> CreateCommand {
        CreateCommand::new("stars")
            .description("Get the number of stars a user has.")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "user",
                    "The user to get the stars for.",
                )
                .required(false),
            )
    }
}
