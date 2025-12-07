
pub struct Distance {
    meters: f64
}

impl From<f64> for Distance {
    fn from(value: f64) -> Self {
        Self {
            meters: value
        }
    }
}

impl Distance {

    pub fn new() -> Self {
        Self {
            meters: 0.0
        }
    }

}