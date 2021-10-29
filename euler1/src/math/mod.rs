use num::Zero;
use std::ops::Rem;

/// Determine whether or not `a` is divisible by `b`.
pub(super) fn is_divisible<T>(a: T, b: T) -> bool
where
    T: PartialEq + Rem<Output = T> + Zero + Copy,
{
    let zero: T = Zero::zero();
    a % b == zero
}
