use std::str::FromStr;

use regex::Regex;
use thiserror::Error;

use crate::{
    token::{Operator, Token},
    traits::{Constants, Numeric},
    tree::BinaryTree,
};

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Tree was empty or missing after parsing")]
    EmptyTree,
    #[error("Input {0} is not allowed")]
    InvalidInput(String),
}

fn tokenize_term<T: Numeric>(
    term: &str,
    constants: &Constants<T>,
) -> Result<Vec<Token<T>>, ParsingError> {
    let regex =
        Regex::new(r"(?m)(0[bB][01]*(?:[iu]\d{1,2})?)|(0[xX][0-9A-Fa-f]*(?:[iu]\d{1,2})?)|(?:-?\d+(?:\.|,\d+)?)|[()+*-=]|([A-Za-z]+)")
            .unwrap();
    let term_lower = term.to_lowercase();
    let result = regex.find_iter(&term_lower);

    let mut tokens: Vec<Token<T>> = vec![];

    for m in result {
        println!("{:#?}", m);
        let token = match m.as_str() {
            "e" => Token::Value(constants.e),
            "pi" => Token::Value(constants.pi),
            other => match Token::<T>::from_str(other) {
                Ok(t) => t,
                Err(err) => {
                    println!("Token not allowed: {}, {err:?}", other);
                    return Err(ParsingError::InvalidInput(other.to_owned()));
                }
            },
        };
        tokens.push(token);
    }

    return Ok(tokens);
}

pub fn validate_term<T: Numeric>(tokens: &Vec<Token<T>>) -> bool {
    let mut in_count = 0;
    let mut out_count = 0;
    for t in tokens.iter() {
        if t.is_in() {
            in_count += 1;
        }
        if t.is_out() {
            out_count += 1;
        }
    }

    in_count == out_count
}

pub fn rectify_term<T: Numeric>(tokens: &mut Vec<Token<T>>) {
    println!("{tokens:#?}");
    let mut tokens_len = tokens.len();
    let mut idx = 0;
    while idx < tokens_len {
        let t = tokens[idx];
        if t == Operator::SUB {
            tokens[idx] = Token::add();

            tokens.push(Token::Empty);
            tokens.push(Token::Empty);

            for i in idx + 1..tokens_len {
                tokens[i + 2] = tokens[i];
            }

            tokens[idx + 1] = Token::Value(T::one().neg());
            tokens[idx + 2] = Token::mul();
            tokens_len += 2;
            idx += 2;
        }
        idx += 1;
    }
    if tokens.last().is_none_or(|t| !t.is_eq()) {
        tokens.push(Token::eq());
    }
}

pub fn parse_term<T: Numeric>(
    term: &str,
    constants: &Constants<T>,
) -> Result<BinaryTree<Token<T>>, ParsingError> {
    let mut tokens: Vec<Token<T>> = tokenize_term(term, constants)?;
    rectify_term(&mut tokens);

    let valid = validate_term(&tokens);
    if !valid {
        panic!("Invalid term");
    } else {
        println!("Valid term");
    }

    // TODO: Append missing =
    // TODO: Implement for multi digit inputs -> Parse tokens with regex and use Strings instead of chars.
    // Idea: Use custom operator struct: struct { operator: Option<OperatorEnum>, value: i32 }

    let mut s1: Vec<Token<T>> = Vec::new();
    let mut s2: Vec<BinaryTree<Token<T>>> = Vec::new();

    println!("{tokens:#?}");

    let mut idx = 0;
    while idx < tokens.len() {
        let t = tokens[idx]; // Safe to call, as there is always at least on &str in the string

        if t.is_in() || t.is_mul() {
            s1.push(t);
            idx += 1;
            continue;
        }

        if t.is_add() && s1.last().is_some_and(|x| x.is_mul()) && s2.len() >= 2 {
            s1.pop();
            let l = s2.len();
            let y = s2.pop().unwrap();
            let x = s2.get_mut(l - 2).unwrap();
            x.bin(Token::mul(), y);
            continue;
        }

        if t.is_add() {
            s1.push(t);
            idx += 1;
            continue;
        }

        if t.is_out() && s1.last().is_some_and(|x| x.is_add()) && s2.len() >= 2 {
            s1.pop();
            let l = s2.len();
            let y = s2.pop().unwrap();
            let x = s2.get_mut(l - 2).unwrap();
            x.bin(Token::add(), y);
            continue;
        }

        if t.is_out() && s1.last().is_some_and(|x| x.is_mul()) && s2.len() >= 2 {
            s1.pop();
            let l = s2.len();
            let y = s2.pop().unwrap();
            let x = s2.get_mut(l - 2).unwrap();
            x.bin(Token::mul(), y);
            continue;
        }

        if t.is_out() && s1.last().is_some_and(|x| x.is_in()) {
            s1.pop();
            idx += 1;
            continue;
        }

        if t.is_eq() && s1.last().is_some_and(|x| x.is_add()) && s2.len() >= 2 {
            s1.pop();
            let l = s2.len();
            let y = s2.pop().unwrap();
            let x = s2.get_mut(l - 2).unwrap();
            x.bin(Token::add(), y);
            continue;
        }

        if t.is_eq() && s1.last().is_some_and(|x| x.is_mul()) && s2.len() >= 2 {
            s1.pop();
            let l = s2.len();
            let y = s2.pop().unwrap();
            let x = s2.get_mut(l - 2).unwrap();
            x.bin(Token::mul(), y);
            continue;
        }

        if t.is_eq() && s1.is_empty() && s2.len() == 1 {
            return s2.last().ok_or(ParsingError::EmptyTree).cloned();
        }

        idx += 1;
        s2.push(BinaryTree::new_with_root(t, Token::Empty));
    }
    Err(ParsingError::EmptyTree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_term() {
        let s = "0xffi16 + 1";
        let exp = vec![Token::Value(255.0), Token::add(), Token::Value(1.0)];

        let constants = Constants { e: 2.7, pi: 3.14 };

        let tokens = tokenize_term(s, &constants).unwrap();
        println!("{:#?}", tokens);

        assert!(exp[0] == tokens[0]);
    }
}
