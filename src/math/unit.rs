use core::fmt::{Debug, Display};

pub trait NumLike:
    core::ops::Add<Output = Self>
    + core::ops::AddAssign
    + core::ops::Sub<Output = Self>
    + core::ops::SubAssign
    + core::ops::Mul<Output = Self>
    + core::ops::MulAssign
    + core::ops::Div<Output = Self>
    + core::ops::DivAssign
    + core::ops::Neg<Output = Self>
    + PartialOrd
    + PartialEq
    + Clone
    + Debug
    + Display
{
}

impl<T> NumLike for T where
    T: core::ops::Add<Output = Self>
        + core::ops::AddAssign
        + core::ops::Sub<Output = Self>
        + core::ops::SubAssign
        + core::ops::Mul<Output = Self>
        + core::ops::MulAssign
        + core::ops::Div<Output = Self>
        + core::ops::DivAssign
        + core::ops::Neg<Output = Self>
        + PartialOrd
        + PartialEq
        + Clone
        + Debug
        + Display
{
}
