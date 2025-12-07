use crate::{
    jayutil_unit_generate_measure_traits, jayutil_unit_generate_unit_traits,
    math::unit::NumLike,
    unit::{geom::distance::distance_unit::DistanceUnit, measure::Measure, unit::Unit},
};

pub struct Distance<Num> {
    meters: Num,
}

impl<Num> Distance<Num>
where
    Num: NumLike,
{
    pub fn from(value: Num, unit: DistanceUnit) -> Self {
        Self {
            meters: unit.to_base(value),
        }
    }

    pub fn new() -> Self {
        Self {
            meters: Num::from_f64(0.0),
        }
    }
}

impl<Num> Measure<Num, DistanceUnit> for Distance<Num>
where
    Num: NumLike,
{
    fn get_base(&self) -> Num {
        self.meters.clone()
    }

    fn set_base(&mut self, base: Num) {
        self.meters = base;
    }
}

jayutil_unit_generate_measure_traits!(Distance);
