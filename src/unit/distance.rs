use std::fmt::Formatter;

use crate::{math::unit::NumLike, unit::unit::Unit};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DistanceUnit {
    Meters,
    Kilometers,
    Centimeters,
    Millimeters,
    Miles,
    Yards,
    Feet,
    Inches,
}

impl Unit for DistanceUnit {

    fn from_base<Num>(&self, base: Num) -> Num
    where
        Num: NumLike,
    {
        match self {
            DistanceUnit::Meters => base.clone(),

            DistanceUnit::Kilometers => base / Num::from_f64(1000.0),
            DistanceUnit::Centimeters => base * Num::from_f64(100.0),
            DistanceUnit::Millimeters => base * Num::from_f64(1000.0),

            DistanceUnit::Miles => base / Num::from_f64(1609.344),
            DistanceUnit::Yards => base / Num::from_f64(0.9144),
            DistanceUnit::Feet => base / Num::from_f64(0.3048),
            DistanceUnit::Inches => base / Num::from_f64(0.0254),
        }
    }

    fn to_base<Num>(&self, value: Num) -> Num
    where
        Num: NumLike,
    {
        match self {
            DistanceUnit::Meters => value.clone(),

            DistanceUnit::Kilometers => value * Num::from_f64(1000.0),
            DistanceUnit::Centimeters => value / Num::from_f64(100.0),
            DistanceUnit::Millimeters => value / Num::from_f64(1000.0),

            DistanceUnit::Miles => value * Num::from_f64(1609.344),
            DistanceUnit::Yards => value * Num::from_f64(0.9144),
            DistanceUnit::Feet => value * Num::from_f64(0.3048),
            DistanceUnit::Inches => value * Num::from_f64(0.0254),
        }
    }
    
    fn name(&self) -> &'static str {
        match self {
            DistanceUnit::Meters => "meters",
            DistanceUnit::Kilometers => "kilometers",
            DistanceUnit::Centimeters => "centimeters",
            DistanceUnit::Millimeters => "millimeters",
            DistanceUnit::Miles => "miles",
            DistanceUnit::Yards => "yards",
            DistanceUnit::Feet => "feet",
            DistanceUnit::Inches => "inches",
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            DistanceUnit::Meters => "m",
            DistanceUnit::Kilometers => "km",
            DistanceUnit::Centimeters => "cm",
            DistanceUnit::Millimeters => "mm",
            DistanceUnit::Miles => "mi",
            DistanceUnit::Yards => "yd",
            DistanceUnit::Feet => "ft",
            DistanceUnit::Inches => "in",
        }
    }

}


pub struct Distance<Num> {
    meters: Num
}

impl<Num> Distance<Num> where Num: NumLike {

    pub fn from(value: Num, unit: DistanceUnit) -> Self {
        Self {
            meters: unit.to_base(value)
        }
    }

    pub fn new() -> Self {
        Self::from(Num::from_f64(0.0), DistanceUnit::Meters)
    }

    pub fn set(&mut self, value: Num, unit: DistanceUnit) -> &Self {
        self.meters = unit.to_base(value);
        self
    }

    pub fn to(&self, unit: DistanceUnit) -> Num {
        unit.from_base(self.meters.clone())
    }

}