use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    math::unit::NumLike,
    unit::{measure::Measure, time::time_unit::TimeUnit, unit::Unit},
};

pub struct Time<Num> {
    base: Num,
}

jayutil_unit_generate_measure_impl!(Time, TimeUnit);
jayutil_unit_generate_measure_traits!(Time);
