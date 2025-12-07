use std::fmt::{Display, Formatter};

use crate::math::unit::NumLike;


pub trait Unit: /*  Display */ {

    fn from_base<Num>(&self, base: Num) -> Num where Num: NumLike;
    fn to_base<Num>(&self, value: Num) -> Num where Num: NumLike;

    fn name(&self) -> &'static str;
    fn symbol(&self) -> &'static str;

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.symbol())
    }

}
