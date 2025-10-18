use std::{
    fmt::{Debug, Display},
    num::{ParseFloatError, ParseIntError},
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum FromStrError {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error(transparent)]
    ParseFloat(#[from] ParseFloatError),

    #[error(transparent)]
    Any(#[from] anyhow::Error),

    #[error("Invalid suffix {0}")]
    InvalidSuffix(String),
}

pub trait FromBinaryStr: Sized {
    fn from_binary_str(s: &str) -> Result<Self, FromStrError>;
}

fn split_at_first_non_digit(s: &str) -> (&str, &str) {
    let idx = s
        .chars()
        .position(|c| c == 'i' || c == 'u')
        .unwrap_or(s.len());
    s.split_at(idx)
}

impl FromBinaryStr for f32 {
    fn from_binary_str(s: &str) -> Result<Self, FromStrError> {
        let (num_part, suffix) = split_at_first_non_digit(s.trim_start_matches("0b"));

        match suffix {
            "i16" => {
                let bits = u16::from_str_radix(num_part, 2)?;
                Ok((bits as i16) as f32)
            }
            "u16" => {
                let bits = u16::from_str_radix(num_part, 2)?;
                Ok(bits as f32)
            }
            "u8" => {
                let bits = u8::from_str_radix(num_part, 2)?;
                Ok(bits as f32)
            }
            "i8" => {
                let bits = u8::from_str_radix(num_part, 2)?;
                Ok((bits as i8) as f32)
            }
            "" => {
                let bits = u32::from_str_radix(s, 2)?;
                Ok(f32::from_bits(bits))
            }
            other => Err(FromStrError::InvalidSuffix(other.to_owned())),
        }
    }
}

pub trait FromHexStr: Sized {
    fn from_hex_str(s: &str) -> Result<Self, FromStrError>;
}
impl FromHexStr for f32 {
    fn from_hex_str(s: &str) -> Result<Self, FromStrError> {
        let (num_part, suffix) = split_at_first_non_digit(s.trim_start_matches("0x"));
        println!("{:#?}", num_part.bytes());
        match suffix {
            "i16" => {
                let bits = u16::from_str_radix(num_part, 16)?;
                Ok((bits as i16) as f32)
            }
            "u16" => {
                let bits = u16::from_str_radix(num_part, 16)?;
                Ok(bits as f32)
            }
            "u8" => {
                let bits = u8::from_str_radix(num_part, 16)?;
                Ok(bits as f32)
            }
            "i8" => {
                let bits = u8::from_str_radix(num_part, 16)?;
                Ok((bits as i8) as f32)
            }
            "" => {
                let bits = u32::from_str_radix(num_part, 16)?;
                Ok(f32::from_bits(bits))
            }
            other => Err(FromStrError::InvalidSuffix(other.to_owned())),
        }
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
    + FromBinaryStr
    + FromHexStr
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
        + FromHexStr
        + FromBinaryStr
        + FromStr
{
}

pub struct Constants<T> {
    pub e: T,
    pub pi: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_str_to_f32() {
        let s = "0xffi16";
        let hex = f32::from_hex_str(s).unwrap();
        assert_eq!(255.0, hex);

        let s = "0xfi8";
        let hex = f32::from_hex_str(s).unwrap();
        assert_eq!(15.0, hex);

        let s = "0xffu16";
        let hex = f32::from_hex_str(s).unwrap();
        assert_eq!(255.0, hex);
        let s = "0xffu8";
        let hex = f32::from_hex_str(s).unwrap();
        assert_eq!(255.0, hex);
    }
}
