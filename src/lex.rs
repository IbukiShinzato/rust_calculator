use std::iter::Peekable;

use crate::token::{LexError, TokenTypes};

fn is_white_space(c: char) -> bool {
    ['\t', ' '].contains(&c)
}

fn is_numbers(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_operators(c: char) -> bool {
    ['+', '-', '*', '/', '(', ')'].contains(&c)
}

fn operator_type(c: char) -> TokenTypes {
    match c {
        '+' => TokenTypes::OperatorAdd,
        '-' => TokenTypes::OperatorSub,
        '*' => TokenTypes::OperatorMul,
        '/' => TokenTypes::OperatorDiv,
        '(' => TokenTypes::OperatorOpen,
        ')' => TokenTypes::OperatorClose,
        _ => TokenTypes::Enter,
    }
}

fn get_token<I>(token: &mut String, iter: &mut Peekable<I>) -> TokenTypes
where
    I: Iterator<Item = char>,
{
    while let Some(&c) = iter.peek() {
        if is_white_space(c) {
            iter.next();
            continue;
        }
        break;
    }

    if let Some(&c) = iter.peek() {
        if c == '\n' {
            return TokenTypes::Enter;
        }

        if is_operators(c) {
            iter.next();
            token.push(c);
            return operator_type(c);
        }

        if is_numbers(c) {
            while let Some(&c) = iter.peek().filter(|&&c| is_numbers(c)) {
                token.push(c);
                iter.next();
            }
            return TokenTypes::Number;
        }

        return TokenTypes::Invalid(LexError::Unknown);
    }

    TokenTypes::Invalid(LexError::Null)
}

pub fn lex_analysis(buf: &str) -> Result<Vec<(String, TokenTypes)>, LexError> {
    let mut token = String::new();
    let mut iter = buf.chars().peekable();

    let mut token_list = vec![];
    loop {
        match get_token(&mut token, &mut iter) {
            TokenTypes::Invalid(e) => return Err(e),
            TokenTypes::Enter => break,
            token_type => {
                token_list.push((token.clone(), token_type.clone()));
                token.clear();
            }
        }
    }

    Ok(token_list)
}
