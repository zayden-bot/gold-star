use async_trait::async_trait;
use chrono::Utc;
use serenity::all::{
    CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption, ResolvedValue,
    UserId,
};
use sqlx::{Database, Pool};
use zayden_core::parse_options;

use crate::manager::GoldStarRow;
use crate::{error::Result, manager::GoldStarManager, Error};

use super::GoldStarCommand;

const HOURS_24: i64 = 86400;

pub struct GiveStarResponse {
    target_user: UserId,
    target_user_stars: i32,
    reason: Option<String>,
}

pub struct GiveStar;

#[async_trait]
impl GoldStarCommand<GiveStarResponse> for GiveStar {
    async fn run<Db: Database, Manager: GoldStarManager<Db>>(
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<GiveStarResponse> {
        let options = interaction.data.options();
        let options = parse_options(&options);

        let target_user = match options.get("member") {
            Some(ResolvedValue::User(user, _)) => *user,
            _ => unreachable!("User option is required"),
        };

        if interaction.user.id == target_user.id {
            return Err(Error::SelfStar);
        }

        let mut author_row = match Manager::get_row(pool, interaction.user.id).await? {
            Some(stars) => stars,
            None => GoldStarRow::new(interaction.user.id),
        };
        let mut target_row = match Manager::get_row(pool, target_user.id).await? {
            Some(stars) => stars,
            None => GoldStarRow::new(target_user.id),
        };

        let free_star =
            author_row.last_free_star.and_utc().timestamp() + HOURS_24 <= Utc::now().timestamp();

        if author_row.number_of_stars < 1 && !free_star {
            return Err(Error::NoStars);
        }

        if free_star {
            author_row.give_free_star(&mut target_row);
        } else {
            author_row.give_star(&mut target_row);
        }

        author_row.save::<Db, Manager>(pool).await?;
        target_row.save::<Db, Manager>(pool).await?;

        let reason = if let Some(ResolvedValue::String(r)) = options.get("reason") {
            Some(r.to_string())
        } else {
            None
        };

        Ok(GiveStarResponse {
            target_user: target_user.id,
            target_user_stars: target_row.number_of_stars,
            reason,
        })
    }

    fn register() -> CreateCommand {
        CreateCommand::new("gold_star")
            .description("Give a user a star")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "member",
                    "The member to give a star to",
                )
                .required(true),
            )
            .add_option(CreateCommandOption::new(
                CommandOptionType::String,
                "reason",
                "The reason for giving a star",
            ))
    }
}
