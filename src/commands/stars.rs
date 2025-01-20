use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, EditInteractionResponse, ResolvedValue,
};
use sqlx::{Database, Pool};
use zayden_core::parse_options;

use crate::{GoldStarManager, GoldStarRow, Result, Stars};

impl Stars {
    pub async fn run<Db: Database, Manager: GoldStarManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let options = interaction.data.options();
        let mut options = parse_options(options);

        let user = match options.remove("user") {
            Some(ResolvedValue::User(user, _)) => user,
            _ => &interaction.user,
        };

        let row = match Manager::get_row(pool, user.id).await.unwrap() {
            Some(row) => row,
            None => GoldStarRow::new(user.id),
        };

        let username = user.global_name.as_deref().unwrap_or(&user.name);

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().embed(
                    CreateEmbed::new()
                        .title(format!("{}'s Stars", username))
                        .field("Number of Stars", row.number_of_stars.to_string(), true)
                        .field("Given Stars", row.given_stars.to_string(), true)
                        .field("Received Stars", row.received_stars.to_string(), true),
                ),
            )
            .await
            .unwrap();

        Ok(())
    }

    pub fn register() -> CreateCommand {
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
