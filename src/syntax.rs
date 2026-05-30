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
    let ret = exp_tail(iter, l)?;

    Ok(ret)
}

fn exp_tail<I>(iter: &mut Peekable<I>, l: i32) -> Result<i32, ParseError>
where
    I: Iterator<Item = (String, TokenTypes)>,
{
    if let Some((_, token_type)) = iter.peek() {
        match token_type {
            TokenTypes::OperatorAdd => {
                iter.next();

                let r = term(iter)?;
                let ret = exp_tail(iter, l + r)?;

                Ok(ret)
            }
            TokenTypes::OperatorSub => {
                iter.next();

                let r = term(iter)?;
                let ret = exp_tail(iter, l - r)?;

                Ok(ret)
            }
            _ => Ok(l),
        }
    } else {
        Ok(l)
    }
}

fn term<I>(iter: &mut Peekable<I>) -> Result<i32, ParseError>
where
    I: Iterator<Item = (String, TokenTypes)>,
{
    let l = factor(iter)?;
    let ret = term_tail(iter, l)?;

    Ok(ret)
}

fn term_tail<I>(iter: &mut Peekable<I>, l: i32) -> Result<i32, ParseError>
where
    I: Iterator<Item = (String, TokenTypes)>,
{
    if let Some((_, token_type)) = iter.peek() {
        match token_type {
            TokenTypes::OperatorMul => {
                iter.next();

                let r = factor(iter)?;
                let ret = term_tail(iter, l * r)?;

                Ok(ret)
            }
            TokenTypes::OperatorDiv => {
                iter.next();

                let r = factor(iter)?;
                if r == 0 {
                    return Err(ParseError::ZeroDivide);
                }
                let ret = term_tail(iter, l / r)?;

                Ok(ret)
            }
            _ => Ok(l),
        }
    } else {
        Ok(l)
    }
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
            TokenTypes::Number => match token.parse() {
                Ok(ret) => {
                    iter.next();
                    Ok(ret)
                }
                Err(_) => Err(ParseError::Syntax),
            },
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

    let ret = exp(&mut iter)?;

    if iter.peek().is_none() {
        println!("{ret}");
        Ok(())
    } else {
        Err(ParseError::Syntax)
    }
}
