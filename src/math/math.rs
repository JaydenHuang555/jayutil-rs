use std::ops::Neg;

use crate::math::unit::NumLike;

pub fn epsilon_equals<Comparable>(
    input: Comparable,
    expected: Comparable,
    epsilon: Comparable,
) -> bool
where
    // Comparable: PartialOrd + Add<Output = Comparable> + Sub<Output = Comparable> + Copy + Debug,
    Comparable: NumLike + Copy,
{
    (input - epsilon <= expected) && (input + epsilon >= expected)
}

pub fn hypot<Num>(x_input: Num, y_input: Num) -> Num
where
    Num: Into<f64> + From<f64>,
{
    let x = x_input.into();
    let y = y_input.into();
    Num::from(((x * x) + (y * y)).sqrt())
}

pub fn max<Comparable>(input1: Comparable, input2: Comparable) -> Comparable
where
    Comparable: PartialOrd,
{
    if input2 > input1 {
        return input2;
    }
    input1
}

pub fn min<Comparable>(input1: Comparable, input2: Comparable) -> Comparable
where
    Comparable: PartialOrd,
{
    if input2 < input1 {
        return input2;
    }
    input1
}

pub fn limit<Comparable>(
    input: Comparable,
    min_input: Comparable,
    max_input: Comparable,
) -> Comparable
where
    Comparable: PartialOrd,
{
    min(max_input, max(input, min_input))
}

pub fn limit_abs<Comparable>(input: Comparable, abs_range: Comparable) -> Comparable
where
    Comparable: PartialOrd + Neg<Output = Comparable> + Clone,
{
    let low = -abs_range.clone();
    let high = abs_range;
    limit(input, low, high)
}
