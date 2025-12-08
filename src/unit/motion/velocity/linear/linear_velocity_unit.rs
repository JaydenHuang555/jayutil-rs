
use crate::{jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits, jayutil_unit_motion_generate_impl, math::unit::NumLike, unit::{geom::distance::{distance_measure::Distance, distance_unit::{self, DistanceUnit}}, time::{time_measure::Time, time_unit::{self, TimeUnit}}}};
use crate::unit::unit::Unit;

pub struct LinearVelocityUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str
}

jayutil_unit_generate_unit_impl!(LinearVelocityUnit);
jayutil_unit_generate_unit_traits!(LinearVelocityUnit);

jayutil_unit_motion_generate_impl!(LinearVelocityUnit, DistanceUnit, TimeUnit);

pub const METERS_PER_SECOND: LinearVelocityUnit = LinearVelocityUnit::from(1.0, "Meters per Second", "m/s");
pub const KILOMETERS_PER_HOUR: LinearVelocityUnit = LinearVelocityUnit::from(1000.0 / 3600.0, "Kilometers per Hour", "km/h");
pub const MILES_PER_HOUR: LinearVelocityUnit = LinearVelocityUnit::from(1609.344 / 3600.0, "Miles per Hour", "mph");
pub const FEET_PER_SECOND: LinearVelocityUnit = LinearVelocityUnit::from(0.3048, "Feet per Second", "ft/s");
