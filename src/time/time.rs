use std::{
    alloc::System,
    error::Error,
    fmt::Display,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
pub enum TimeError {
    InvalidEpoch,
    Empty,
}

impl Error for TimeError {}

impl Display for TimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidEpoch => writeln!(f, "fetched epoch value is invalid"),
            Self::Empty => writeln!(f, "fetched value was empty"),
        }
    }
}

pub fn get_epoch() -> Result<f64, TimeError> {
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(time) => Result::Ok(time.as_secs_f64()),
        Err(_) => Result::Err(TimeError::InvalidEpoch),
    }
}
