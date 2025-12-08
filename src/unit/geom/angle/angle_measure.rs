use crate::unit::unit::Unit;
use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    math::unit::NumLike,
    unit::{geom::angle::angle_unit::AngleUnit, measure::Measure},
};

pub struct Angle<Num>
where
    Num: NumLike,
{
    base: Num,
}

jayutil_unit_generate_measure_impl!(Angle, AngleUnit);
jayutil_unit_generate_measure_traits!(Angle);
