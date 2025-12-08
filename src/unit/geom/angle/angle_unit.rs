use crate::{
    jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits, math::unit::NumLike,
    unit::unit::Unit,
};

pub struct AngleUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayutil_unit_generate_unit_impl!(AngleUnit);
jayutil_unit_generate_unit_traits!(AngleUnit);

pub const RADIANS: AngleUnit = AngleUnit::from(1.0, "Radians", "rad");
pub const DEGREES: AngleUnit = AngleUnit::from(std::f64::consts::PI / 180.0, "Degrees", "°");
pub const GRADIANS: AngleUnit = AngleUnit::from(std::f64::consts::PI / 200.0, "Gradians", "gon");
pub const ARCMINUTES: AngleUnit =
    AngleUnit::from(std::f64::consts::PI / 10800.0, "Arcminutes", "′");
pub const ARCSECONDS: AngleUnit =
    AngleUnit::from(std::f64::consts::PI / 648_000.0, "Arcseconds", "″");
pub const REVOLUTIONS: AngleUnit =
    AngleUnit::from(2.0 * std::f64::consts::PI, "Revolutions", "rev");
pub const ROTATIONS: AngleUnit = AngleUnit::from(2.0 * std::f64::consts::PI, "Revolutions", "rev");
