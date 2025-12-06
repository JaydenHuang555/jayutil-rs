use std::{alloc::System, time::{SystemTime, UNIX_EPOCH}};


pub fn get_epoch() -> Result<f64, ()> {
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(time) => {
            Result::Ok(time.as_secs_f64())
        }
        Err(_) => {
            Result::Err(())
        },
    }
}