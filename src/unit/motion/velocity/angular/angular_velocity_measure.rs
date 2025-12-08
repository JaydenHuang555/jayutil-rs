use crate::{jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits, unit::motion::velocity::angular::angular_velocity_unit::AngularVelocityUnit};

use crate::unit::measure::Measure;
use crate::unit::unit::Unit;

pub struct AngularVelocity<Num> {
    base: Num
}

jayutil_unit_generate_measure_impl!(AngularVelocity, AngularVelocityUnit);
jayutil_unit_generate_measure_traits!(AngularVelocity);