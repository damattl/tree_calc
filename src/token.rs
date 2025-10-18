use std::{
    fmt::{self},
    str::FromStr,
};

use anyhow::anyhow;

use crate::traits::{FromStrError, Numeric};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    EQ,
    IN,
    OUT,
    ADD,
    MUL,
    SUB,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Operator::EQ => "=",
            Operator::IN => "(",
            Operator::OUT => ")",
            Operator::ADD => "+",
            Operator::MUL => "*",
            Operator::SUB => "-",
        };
        f.write_str(symbol)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<T>
where
    T: Numeric,
{
    Empty,
    Op(Operator),
    Value(T),
}

impl<T> PartialEq<Operator> for Token<T>
where
    T: Numeric,
{
    fn eq(&self, other: &Operator) -> bool {
        self == &Self::Op(*other)
    }
}
impl<T> PartialEq<Operator> for &Token<T>
where
    T: Numeric,
{
    fn eq(&self, other: &Operator) -> bool {
        self == &&Token::Op(*other)
    }
}

impl<T: Numeric> fmt::Display for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Op(op) => write!(f, "{}", op),
            Token::Value(v) => write!(f, "{}", v.to_string()),
            Token::Empty => write!(f, "empty"),
        }
    }
}

impl<T: Numeric> FromStr for Token<T> {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token = match s {
            "*" => Token::<T>::mul(),
            "+" => Token::<T>::add(),
            "-" => Token::<T>::sub(),
            "(" => Token::<T>::i(),
            ")" => Token::<T>::out(),
            "=" => Token::<T>::eq(),
            el => {
                if el.len() >= 2 {
                    match &el[..2] {
                        "0x" => T::from_hex_str(el).map(|n| Token::<T>::Value(n))?,
                        "0b" => T::from_binary_str(el).map(|n| Token::<T>::Value(n))?,
                        _ => el
                            .parse::<T>()
                            .map(|n| Token::<T>::Value(n))
                            .map_err(|_| anyhow!("parsing {} failed", el))?,
                    }
                } else {
                    el.parse::<T>()
                        .map(|n| Token::<T>::Value(n))
                        .map_err(|_| anyhow!("parsing {} failed", el))?
                }
            }
        };

        Ok(token)
    }
}

impl<T: Numeric> Token<T> {
    pub fn mul() -> Self {
        Self::Op(Operator::MUL)
    }
    pub fn add() -> Self {
        Self::Op(Operator::ADD)
    }
    pub fn eq() -> Self {
        Self::Op(Operator::EQ)
    }
    pub fn i() -> Self {
        Self::Op(Operator::IN)
    }
    pub fn out() -> Self {
        Self::Op(Operator::OUT)
    }
    pub fn sub() -> Self {
        Self::Op(Operator::SUB)
    }

    pub fn is_add(&self) -> bool {
        self == Operator::ADD
    }
    pub fn is_mul(&self) -> bool {
        self == Operator::MUL
    }
    pub fn is_in(&self) -> bool {
        self == Operator::IN
    }
    pub fn is_out(&self) -> bool {
        self == Operator::OUT
    }
    pub fn is_eq(&self) -> bool {
        self == Operator::EQ
    }
}
