use core::fmt::{Debug, Display};
use std::ops::Neg;

pub trait NumLike:
    core::ops::Add<Output = Self>
    + core::ops::AddAssign
    + core::ops::Sub<Output = Self>
    + core::ops::SubAssign
    + core::ops::Mul<Output = Self>
    + core::ops::MulAssign
    + core::ops::Div<Output = Self>
    + core::ops::DivAssign
    // + core::ops::Neg<Output = Self>
    + From<Self>
    + PartialOrd
    + PartialEq
    + Clone
    + Debug
    + Display
{
    fn from_f64( base: f64) -> Self;
    fn to_f64(&self) -> f64;
}

macro_rules! impl_numlike {
    ($($t:ty),*) => {
        $(
            impl NumLike for $t {
                #[inline]
                fn from_f64(base: f64) -> Self {
                    base as $t
                }

                #[inline]
                fn to_f64(&self) -> f64 {
                    *self as f64
                }
            }
        )*
    };
}

impl_numlike!(
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
    f32, f64
);

