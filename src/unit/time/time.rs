use crate::{jayutil_unit_generate_measure_traits, math::unit::NumLike, unit::{measure::Measure, unit::Unit}};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeUnit {
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

impl Unit for TimeUnit {
    fn from_base<Num>(&self, base: Num) -> Num
    where
        Num: NumLike,
    {
        match self {
            TimeUnit::Milliseconds => base * Num::from_f64(1000.0),
            TimeUnit::Seconds => base.clone(),
            TimeUnit::Minutes => base / Num::from_f64(60.0),
            TimeUnit::Hours => base / Num::from_f64(3600.0),
            TimeUnit::Days => base / Num::from_f64(86400.0),
            TimeUnit::Weeks => base / Num::from_f64(604800.0),
            TimeUnit::Months => base / Num::from_f64(2629800.0), // 30.4375 days * 86400 seconds
            TimeUnit::Years => base / Num::from_f64(31557600.0), // 365.25 days * 86400 seconds
        }
    }

    fn to_base<Num>(&self, value: Num) -> Num
    where
        Num: NumLike,
    {
        match self {
            TimeUnit::Milliseconds => value / Num::from_f64(1000.0),
            TimeUnit::Seconds => value.clone(),
            TimeUnit::Minutes => value * Num::from_f64(60.0),
            TimeUnit::Hours => value * Num::from_f64(3600.0),
            TimeUnit::Days => value * Num::from_f64(86400.0),
            TimeUnit::Weeks => value * Num::from_f64(604800.0),
            TimeUnit::Months => value * Num::from_f64(2629800.0),
            TimeUnit::Years => value * Num::from_f64(31557600.0),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            TimeUnit::Milliseconds => "milliseconds",
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Weeks => "weeks",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            TimeUnit::Milliseconds => "ms",
            TimeUnit::Seconds => "s",
            TimeUnit::Minutes => "min",
            TimeUnit::Hours => "h",
            TimeUnit::Days => "d",
            TimeUnit::Weeks => "wk",
            TimeUnit::Months => "mo",
            TimeUnit::Years => "yr",
        }
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::writeln!(f, "{}", self.symbol())
    }
}

pub struct Time<Num> where Num: NumLike {
    seconds: Num
}

impl<Num> Time<Num> where Num: NumLike {
    pub fn new() -> Self {
        Self {
            seconds: Num::from_f64(0.0)
        }
    }
}

impl<Num> Measure<Num, TimeUnit> for Time<Num> where Num: NumLike {
    fn set_base(&mut self, base: Num) {
        self.seconds = base;
    }

    fn get_base(&self) -> Num {
        self.seconds.clone()
    }
}

jayutil_unit_generate_measure_traits!(
    Time
);