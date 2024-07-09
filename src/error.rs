use std::time::Duration;
use zayden_core::ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

const SECS_PER_MINUTE: u64 = 60;
const SECS_PER_HOUR: u64 = 3600;

#[derive(Debug)]
pub enum Error {
    // CommandNotInGuild,
    // InvalidMessageId(String),
    // InvalidEmoji(String),
    SelfStar,
    NoStars(Duration),

    // MemberNotFound(serenity::all::Reaction),
    // GuildNotFound(serenity::all::Reaction),
    // UserNotFound(serenity::all::Reaction),
    Sqlx(sqlx::Error),
}

impl ErrorResponse for Error {
    fn to_response(&self) -> String {
        match self {
            Self::SelfStar => "You can't give yourself a star.".to_string(),
            Self::NoStars(t) => {
                let hours = t.as_secs() / SECS_PER_HOUR;
                let minutes = (t.as_secs() % SECS_PER_HOUR) / SECS_PER_MINUTE;
                let seconds = t.as_secs() % SECS_PER_MINUTE;

                format!(
                    "You don't have any stars to give.\nNext free star in: {}:{}:{}.",
                    hours, minutes, seconds
                )
            }
            _ => String::new(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
    }
}
