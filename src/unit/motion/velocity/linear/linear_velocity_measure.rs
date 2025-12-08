use crate::unit::measure::Measure;
use crate::unit::unit::Unit;
use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    unit::motion::velocity::linear::linear_velocity_unit::LinearVelocityUnit,
};

pub struct LinearVelocity<Num> {
    base: Num,
}

jayutil_unit_generate_measure_impl!(LinearVelocity, LinearVelocityUnit);
jayutil_unit_generate_measure_traits!(LinearVelocity);
