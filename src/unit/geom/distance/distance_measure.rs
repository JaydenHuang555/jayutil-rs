use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    jayutil_unit_generate_unit_traits,
    math::unit::NumLike,
    unit::{geom::distance::distance_unit::DistanceUnit, measure::Measure, unit::Unit},
};

pub struct Distance<Num> {
    base: Num,
}

jayutil_unit_generate_measure_impl!(Distance, DistanceUnit);
jayutil_unit_generate_measure_traits!(Distance);
