use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

pub trait One
where
    Self: Sized,
{
    fn one() -> Self;
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::one()
    }
}

impl One for f32 {
    fn one() -> Self {
        1.0
    }
}

pub trait Numeric:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Default
    + Neg<Output = Self>
    + One
    + PartialEq
    + Display
    + Debug
    + FromStr
{
}

impl<T> Numeric for T where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Default
        + Neg<Output = T>
        + One
        + PartialEq
        + Display
        + Debug
        + FromStr
{
}

pub struct Constants<T> {
    pub e: T,
    pub pi: T,
}
