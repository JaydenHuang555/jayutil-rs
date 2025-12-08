use crate::{jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits, math::unit::NumLike, unit::{geom::angle::angle_unit::AngleUnit, measure::Measure}};
use crate::unit::unit::Unit;



pub struct Angle<Num>
where
    Num: NumLike,
{
    base: Num,
}


// impl<Num> Angle<Num>
// where
//     Num: NumLike,
// {
//     pub fn from(value: Num, unit: AngleUnit) -> Self {
//         Self {
//             base: unit.to_base(value),
//         }
//     }

//     pub fn new() -> Self {
//         Self {
//             base: Num::from_f64(0.0),
//         }
//     }
// }


jayutil_unit_generate_measure_impl!(Angle, AngleUnit);
jayutil_unit_generate_measure_traits!(Angle);
