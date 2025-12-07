use std::{error::Error, fmt::Display};

use crate::time::time;
#[derive(Debug)]
pub enum PauseError {
    InvalidEpoch,
    EnabledWithoutTracking,
}

impl Error for PauseError {}

impl Display for PauseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidEpoch => {
                writeln!(f, "Epoch time was invalid!!!")
            }
            Self::EnabledWithoutTracking => {
                writeln!(f, "Enabled without tracking duration!!!")
            }
        }
    }
}

pub struct PauseHandler {
    enabled: bool,
    start_time: Option<f64>,
    end_time: Option<f64>,
    duration: f64,
}

impl Default for PauseHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl PauseHandler {
    pub fn new() -> Self {
        Self {
            enabled: false,
            start_time: Option::None,
            end_time: Option::None,
            duration: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.start_time = Option::None;
        self.end_time = Option::None;
        self.duration = 0.0;
    }

    pub fn enable(&mut self) -> Result<(), PauseError> {
        let attempt = time::get_epoch();
        match attempt {
            Ok(time) => {
                self.enabled = true;
                self.start_time = Option::Some(time);
                Result::Ok(())
            }
            Err(_) => Result::Err(PauseError::InvalidEpoch),
        }
    }

    pub fn disable(&mut self) -> Result<(), PauseError> {
        if !self.enabled {
            return Result::Ok(());
        }

        if self.end_time.is_none() || self.start_time.is_none() {
            return Result::Err(PauseError::EnabledWithoutTracking);
        }

        let attempt = time::get_epoch();
        match attempt {
            Ok(time) => {
                self.enabled = false;
                self.end_time = Option::Some(time);

                let end_time = self.end_time.unwrap();
                let start_time = self.start_time.unwrap();
                let total_duration = end_time - start_time;
                self.duration += total_duration;

                Result::Ok(())
            }
            Err(_) => Result::Err(PauseError::InvalidEpoch),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_duration(&self) -> f64 {
        self.duration
    }
}
