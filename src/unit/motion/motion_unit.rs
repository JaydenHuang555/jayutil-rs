use crate::{math::unit::NumLike, unit::{measure::Measure, unit::Unit}};


pub trait MotionUnit<TU, RU>: Unit 
where 
    TU: Unit,
    RU: Unit,
{
    fn derive_units(travel: TU, rate: RU) -> Self;
}

#[macro_export]
macro_rules! jayutil_unit_motion_generate_impl {
    ($($t:ident, $tu:ident, $ru:ident)*) => {
       $(
            impl crate::unit::motion::motion_unit::MotionUnit<$tu, $ru> for $t {

                fn derive_units(travel: $tu, rate: $ru) -> Self {
                    let format_name = format!("{} per {}", travel.name(), rate.name());
                    let format_symbol = format!("{} / {}", travel.symbol(), rate.symbol());
                    Self {
                        scale_to_base: travel.get_scale_to_base() / rate.get_scale_to_base(),
                        name: Box::leak(format_name.into_boxed_str()),
                        symbol: Box::leak(format_symbol.into_boxed_str())
                    }
                }
            }
       )* 
    };
}