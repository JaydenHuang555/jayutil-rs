use crate::{
    jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits, math::unit::NumLike,
    unit::unit::Unit,
};

pub struct DistanceUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayutil_unit_generate_unit_traits!(DistanceUnit);
jayutil_unit_generate_unit_impl!(DistanceUnit);

pub const METERS: DistanceUnit = DistanceUnit::from(1.0, "Meters", "m");
pub const KILOMETERS: DistanceUnit = DistanceUnit::from(1000.0, "Kilometers", "km");
pub const CENTIMETERS: DistanceUnit = DistanceUnit::from(0.01, "Centimeters", "cm");
pub const MILLIMETERS: DistanceUnit = DistanceUnit::from(0.001, "Millimeters", "mm");
pub const MICROMETERS: DistanceUnit = DistanceUnit::from(0.000_001, "Micrometers", "µm");
pub const NANOMETERS: DistanceUnit = DistanceUnit::from(0.000_000_001, "Nanometers", "nm");

pub const MILES: DistanceUnit = DistanceUnit::from(1609.344, "Miles", "mi");
pub const YARDS: DistanceUnit = DistanceUnit::from(0.9144, "Yards", "yd");
pub const FEET: DistanceUnit = DistanceUnit::from(0.3048, "Feet", "ft");
pub const INCHES: DistanceUnit = DistanceUnit::from(0.0254, "Inches", "in");

pub const NAUTICAL_MILES: DistanceUnit = DistanceUnit::from(1852.0, "Nautical Miles", "nmi");

pub const ASTRONOMICAL_UNITS: DistanceUnit =
    DistanceUnit::from(149_597_870_700.0, "Astronomical Units", "AU");
pub const LIGHT_YEARS: DistanceUnit =
    DistanceUnit::from(9_460_730_472_580_800.0, "Light Years", "ly");
pub const PARSECS: DistanceUnit = DistanceUnit::from(30_856_775_814_913_672.0, "Parsecs", "pc");
