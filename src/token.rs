use std::{
    fmt::{self},
    str::FromStr,
};

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
    T: PartialEq + Copy + FromStr + ToString,
{
    Empty,
    Op(Operator),
    Value(T),
}

impl<T> PartialEq<Operator> for Token<T>
where
    T: PartialEq + Copy + FromStr + ToString,
{
    fn eq(&self, other: &Operator) -> bool {
        self == &Self::Op(*other)
    }
}
impl<T> PartialEq<Operator> for &Token<T>
where
    T: PartialEq + Copy + FromStr + ToString,
{
    fn eq(&self, other: &Operator) -> bool {
        self == &&Token::Op(*other)
    }
}

impl<T: PartialEq + Copy + FromStr + ToString> fmt::Display for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Op(op) => write!(f, "{}", op),
            Token::Value(v) => write!(f, "{}", v.to_string()),
            Token::Empty => write!(f, "empty"),
        }
    }
}

impl<T: PartialEq + Copy + FromStr + ToString> FromStr for Token<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token = match s {
            "*" => Token::<T>::mul(),
            "+" => Token::<T>::add(),
            "-" => Token::<T>::sub(),
            "(" => Token::<T>::i(),
            ")" => Token::<T>::out(),
            "=" => Token::<T>::eq(),
            el => match el.parse::<T>() {
                Ok(n) => Token::<T>::Value(n),
                Err(_) => return Err(()),
            },
        };

        Ok(token)
    }
}

impl<T: PartialEq + Copy + FromStr + ToString> Token<T> {
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
