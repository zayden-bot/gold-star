use std::time::Duration;
use zayden_core::ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

const SECS_PER_MINUTE: u64 = 60;
const SECS_PER_HOUR: u64 = 3600;

#[derive(Debug)]
pub enum Error {
    SelfStar,
    NoStars(String),
}

impl Error {
    pub fn no_stars(t: Duration) -> Self {
        let hours = t.as_secs() / SECS_PER_HOUR;
        let minutes = (t.as_secs() % SECS_PER_HOUR) / SECS_PER_MINUTE;
        let seconds = t.as_secs() % SECS_PER_MINUTE;

        Error::NoStars(format!(
            "You don't have any stars to give.\nNext free star in: {}h {}m {}s.",
            hours, minutes, seconds
        ))
    }
}

impl ErrorResponse for Error {
    fn to_response(&self) -> &str {
        match self {
            Self::SelfStar => "You can't give yourself a star.",
            Self::NoStars(s) => s,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
