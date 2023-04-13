use std::ops::Mul;

pub mod aabb;
pub mod point;
pub mod stats;
pub mod vec;

#[inline]
pub const fn const_pow2<X, X2>(x: X) -> X2
where
    X: Copy,
    X: ~const Mul<Output = X2>,
{
    x * x
}
