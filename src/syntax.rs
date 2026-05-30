use std::{fmt, iter::Peekable};

use super::token::TokenTypes;

#[derive(Debug)]
pub enum ParseError {
    Syntax = -1,
    ZeroDivide = -2,
}

impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Syntax => {
                write!(f, "Syntax error")
            }
            ParseError::ZeroDivide => {
                write!(f, "Zero divide error")
            }
        }
    }
}

fn exp<I>(iter: &mut Peekable<I>) -> Result<i32, ParseError>
where
    I: Iterator<Item = (String, TokenTypes)>,
{
    if iter.peek().is_none() {
        return Err(ParseError::Syntax);
    }

    let l = term(iter)?;

    Ok(l)
}

#[allow(unused)]
fn exp_tail() {
    unimplemented!();
}

fn term<I>(iter: &mut Peekable<I>) -> Result<i32, ParseError>
where
    I: Iterator<Item = (String, TokenTypes)>,
{
    let l = factor(iter)?;

    Ok(l)
}

#[allow(unused)]
fn term_tail() {
    unimplemented!();
}

fn factor<I>(iter: &mut Peekable<I>) -> Result<i32, ParseError>
where
    I: Iterator<Item = (String, TokenTypes)>,
{
    if iter.peek().is_none() {
        return Err(ParseError::Syntax);
    }

    match iter.peek() {
        Some((token, token_type)) => match token_type {
            TokenTypes::Number => {
                let ret = token.parse().unwrap();
                iter.next();
                Ok(ret)
            }
            TokenTypes::OperatorOpen => {
                iter.next();
                let ret = exp(iter)?;

                match iter.peek() {
                    Some(&(_, TokenTypes::OperatorClose)) => {
                        iter.next();
                        Ok(ret)
                    }
                    _ => Err(ParseError::Syntax),
                }
            }
            _ => Err(ParseError::Syntax),
        },
        None => Err(ParseError::Syntax),
    }
}

pub fn syntax_analysis(token_list: Vec<(String, TokenTypes)>) -> Result<(), ParseError> {
    let mut iter = token_list.into_iter().peekable();
    let num = exp(&mut iter)?;
    println!("{num}");

    Ok(())
}
