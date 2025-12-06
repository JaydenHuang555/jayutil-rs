
use crate::time::{stopwatch::pause::{self, PauseError, PauseHandler}, time::{self, TimeError}};

pub struct Stopwatch {
    start_time: Option<f64>,
    pause_handler: PauseHandler
}

impl Stopwatch {

    pub fn new() -> Self {
        Self {
            start_time: Option::None,
            pause_handler: PauseHandler::new()
        }
    }

    pub fn is_started(&self) -> bool {
        self.start_time.is_some()
    }

    pub fn start(&mut self) -> Result<f64, TimeError> {
        let attempt = time::get_epoch();
        match attempt {
            Ok(_) => {
                let start_time = attempt.unwrap();
                self.start_time = Option::Some(start_time);
                return Result::Ok(start_time);
            },
            Err(_) => {
                return attempt;
            },
        }
    }

    pub fn start_if_not_started(&mut self) -> Result<f64, TimeError> {
        if self.start_time.is_none() {
            return self.start();
        }
        Result::Ok(0.0)
    }

    pub fn get_sec(&self) -> Result<f64, TimeError> {
        if self.start_time.is_none() {
            return Result::Err(TimeError::Empty);
        }
        let start_time = self.start_time.unwrap();
        let attempt = time::get_epoch();
        match attempt {
            Ok(time) => {
                let total_duration = time - start_time;
                let duration_without_pause = total_duration - self.pause_handler.get_duration();
                return Result::Ok(duration_without_pause);
            },
            Err(_) => {
                return attempt;
            },
        }
    }

    pub fn pause(&mut self) -> Result<(), PauseError> {
        self.pause_handler.enable()
    }

    pub fn unpause(&mut self) -> Result<(), PauseError> {
        self.pause_handler.disable()
    }

    pub fn is_paused(&self) -> bool {
        return self.pause_handler.is_enabled();
    }

    pub fn pause_if_not_paused(&mut self) -> Result<bool, PauseError> {
        let want_pause = self.is_paused();
        if want_pause {
            match self.pause() {
                Result::Ok(_) => {
                    return Result::Ok(want_pause);
                },
                Result::Err(error) => {
                    return Result::Err(error);
                }
            }
        }
        return Result::Ok(want_pause);
    }

}