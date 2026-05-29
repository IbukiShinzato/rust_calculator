use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum LexError {
    Null = -1,
    Unknown = -2,
}

impl Error for LexError {}
impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown symbols included")
    }
}

#[derive(Debug, Clone)]
pub enum TokenTypes {
    Number,
    OperatorAdd,
    OperatorSub,
    OperatorMul,
    OperatorDiv,
    OperatorOpen,
    OperatorClose,
    Enter,
    Invalid(LexError),
}

pub fn show_token_list(list: &[(String, TokenTypes)]) {
    println!(
        "{}",
        list.iter()
            .map(|(token, token_type)| format!("token: {token}, token_type: {token_type:?}"))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
