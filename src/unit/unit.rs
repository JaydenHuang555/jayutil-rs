use std::fmt::Formatter;

use crate::math::unit::NumLike;

pub trait Unit /*  Display */ {
    fn from_base<Num>(&self, base: Num) -> Num
    where
        Num: NumLike;
    fn to_base<Num>(&self, value: Num) -> Num
    where
        Num: NumLike;

    fn name(&self) -> &'static str;
    fn symbol(&self) -> &'static str;

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.symbol())
    }
}

#[macro_export]
macro_rules! jayutil_unit_generate_unit_traits {
    ($($t:ident), *) => {
        $(
        impl std::fmt::Display for $t {

            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::writeln!(f, "{}", self.symbol())
            }
        }
        )*
    };
}

#[macro_export]
macro_rules! jayutil_unit_generate_from_static_function {
    ($($t: ident), *) => {
       $(
            impl $t {
                pub const fn from(scale_to_base: f64, name: &'static str, symbol: &'static str) -> Self {
                    Self {
                        scale_to_base: scale_to_base,
                        name: name,
                        symbol: symbol
                    }
                }
            }
       )* 
    };
}
