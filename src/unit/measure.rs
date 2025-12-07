use std::ops::Add;

use crate::{math::unit::NumLike, unit::unit::Unit};

pub trait Measure<Num: NumLike, U: Unit> {
    fn set_base(&mut self, base: Num);
    fn get_base(&self) -> Num;

    fn set(&mut self, value: Num, unit: U) -> &Self {
        self.set_base(unit.to_base(value));
        self
    }

    fn to(&self, unit: U) -> Num {
        unit.from_base(self.get_base())
    }
}

#[macro_export]
macro_rules! jayutil_unit_generate_measure_traits {
    ($($t:ident),*) => {
        $(

            impl<Num> Default for $t<Num> where Num: NumLike {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl<Num> std::ops::Add for $t<Num> where Num: NumLike,
            {
                type Output = Self;

                fn add(self, other: Self) -> Self {
                    let base_param  = self.get_base() * other.get_base();
                    let mut ret = Self::default();
                    ret.set_base(base_param);
                    ret
                }
            }

            impl<Num> std::ops::Sub for $t<Num> where Num: NumLike, {
                type Output = Self;

                fn sub(self, other: Self) -> Self {
                    let base_param = self.get_base() - other.get_base();
                    let mut ret = Self::default();
                    ret.set_base(base_param);
                    ret
                }
            }

            impl<Num> std::ops::Mul for $t<Num> where Num: NumLike, {
                type Output = Self;

                fn mul(self, other: Self) -> Self {
                    let base_param = self.get_base() * other.get_base();
                    let mut ret = Self::default();
                    ret.set_base(base_param);
                    ret
                }
            }

            impl<Num> std::ops::Div for $t<Num> where Num: NumLike, {
                type Output = Self;

                fn div(self, other: Self) -> Self {
                    let base_param = self.get_base() / other.get_base();
                    let mut ret = Self::default();
                    ret.set_base(base_param);
                    ret
                }
            }

        )*
    };
}
