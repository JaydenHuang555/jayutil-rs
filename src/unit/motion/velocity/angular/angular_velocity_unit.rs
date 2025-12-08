use crate::unit::geom::angle::angle_unit::AngleUnit;
use crate::unit::time::time_unit::TimeUnit;
use crate::unit::unit::Unit;
use crate::{
    jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits,
    jayutil_unit_motion_generate_impl,
};

pub struct AngularVelocityUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayutil_unit_generate_unit_impl!(AngularVelocityUnit);
jayutil_unit_generate_unit_traits!(AngularVelocityUnit);

jayutil_unit_motion_generate_impl!(AngularVelocityUnit, AngleUnit, TimeUnit);

pub const RADIANS_PER_SECOND: AngularVelocityUnit =
    AngularVelocityUnit::from(1.0, "Radians per Second", "rad / s");
pub const RADIANS_PER_MINUTE: AngularVelocityUnit =
    AngularVelocityUnit::from(60.0, "Radians per Minute", "rad / min");
pub const DEGREES_PER_SECOND: AngularVelocityUnit =
    AngularVelocityUnit::from(std::f64::consts::PI / 180.0, "Degrees per Second", "° / s");
pub const GRADIANS_PER_SECOND: AngularVelocityUnit = AngularVelocityUnit::from(
    std::f64::consts::PI / 200.0,
    "Gradians per Seconds",
    "gon / s",
);
pub const ARCMINUTES_PER_SECOND: AngularVelocityUnit = AngularVelocityUnit::from(
    std::f64::consts::PI / 10800.0,
    "Arcminutes per Seconds",
    "′",
);
pub const ARCSECONDS_PER_SECOND: AngularVelocityUnit = AngularVelocityUnit::from(
    std::f64::consts::PI / 648_000.0,
    "Arcseconds per Seconds",
    "″ / s",
);
pub const REVOLUTIONS_PER_SECOND: AngularVelocityUnit =
    AngularVelocityUnit::from(2.0 * std::f64::consts::PI, "Revolutions per s", "rev / s");
pub const ROTATIONS_PER_SECOND: AngularVelocityUnit =
    AngularVelocityUnit::from(2.0 * std::f64::consts::PI, "Rotations per s", "rot / s");
