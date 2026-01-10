use crate::unit::unit::Unit;
use crate::{jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits};

pub struct TimeUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayutil_unit_generate_unit_impl!(TimeUnit);
jayutil_unit_generate_unit_traits!(TimeUnit);

pub const SECONDS: &TimeUnit = &TimeUnit::from(1.0, "Seconds", "s");
pub const MILLISECONDS: &TimeUnit = &TimeUnit::from(0.001, "Milliseconds", "ms");
pub const MICROSECONDS: &TimeUnit = &TimeUnit::from(0.000_001, "Microseconds", "µs");
pub const NANOSECONDS: &TimeUnit = &TimeUnit::from(0.000_000_001, "Nanoseconds", "ns");

pub const MINUTES: &TimeUnit = &TimeUnit::from(60.0, "Minutes", "min");
pub const HOURS: &TimeUnit = &TimeUnit::from(3600.0, "Hours", "h");
pub const DAYS: &TimeUnit = &TimeUnit::from(86_400.0, "Days", "d");
pub const WEEKS: &TimeUnit = &TimeUnit::from(604_800.0, "Weeks", "wk");
pub const YEARS: &TimeUnit = &TimeUnit::from(31_536_000.0, "Years", "yr"); // 365 days
