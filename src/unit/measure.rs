use crate::{math::unit::NumLike, unit::unit::Unit};

pub trait Measure<Num: NumLike, U: Unit> {
    fn set_base(&mut self, base: Num);
    fn get_base(&self) -> Num;

    fn from(value: Num, unit: U) -> Self;

    fn set(&mut self, value: Num, unit: U) -> &Self {
        self.set_base(unit.to_base(value));
        self
    }

    fn to(&self, unit: U) -> Num {
        unit.from_base(self.get_base())
    }
}
