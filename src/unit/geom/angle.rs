use crate::{
    jayutil_unit_generate_measure_traits, math::unit::NumLike, unit::{measure::Measure, unit::Unit}
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngleUnit {
    Radians,
    Degrees,
    Gradians,
    Revolutions,
    Rotations,
}

impl Unit for AngleUnit {
    fn from_base<Num>(&self, base: Num) -> Num
    where
        Num: NumLike,
    {
        match self {
            AngleUnit::Radians => base.clone(), // base is radians
            AngleUnit::Degrees => base * Num::from_f64(180.0 / std::f64::consts::PI),
            AngleUnit::Gradians => base * Num::from_f64(200.0 / std::f64::consts::PI),
            AngleUnit::Revolutions => base / Num::from_f64(2.0 * std::f64::consts::PI),
            AngleUnit::Rotations => base / Num::from_f64(2.0 * std::f64::consts::PI),
        }
    }

    fn to_base<Num>(&self, value: Num) -> Num
    where
        Num: NumLike,
    {
        match self {
            AngleUnit::Radians => value.clone(), // base is radians
            AngleUnit::Degrees => value * Num::from_f64(std::f64::consts::PI / 180.0),
            AngleUnit::Gradians => value * Num::from_f64(std::f64::consts::PI / 200.0),
            AngleUnit::Revolutions => value * Num::from_f64(2.0 * std::f64::consts::PI),
            AngleUnit::Rotations => value * Num::from_f64(2.0 * std::f64::consts::PI),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            AngleUnit::Radians => "radians",
            AngleUnit::Degrees => "degrees",
            AngleUnit::Gradians => "gradians",
            AngleUnit::Revolutions => "revolutions",
            AngleUnit::Rotations => "rotations",
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            AngleUnit::Radians => "rad",
            AngleUnit::Degrees => "°",
            AngleUnit::Gradians => "gon",
            AngleUnit::Revolutions => "rev",
            AngleUnit::Rotations => "rot",
        }
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::writeln!(f, "{}", self.symbol())
    }
}

pub struct Angle<Num>
where
    Num: NumLike,
{
    radians: Num,
}


impl<Num> Angle<Num>
where
    Num: NumLike,
{
    pub fn new() -> Self {
        Self {
            radians: Num::from_f64(0.0),
        }
    }

    pub fn from(value: Num, unit: AngleUnit) -> Self {
        Self {
            radians: unit.to_base(value),
        }
    }
}

impl<Num> Measure<Num, AngleUnit> for Angle<Num>
where
    Num: NumLike,
{
    fn set_base(&mut self, base: Num) {
        self.radians = base;
    }

    fn get_base(&self) -> Num {
        self.radians.clone()
    }
}

jayutil_unit_generate_measure_traits!(
    Angle
);
