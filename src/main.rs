use std::io::{Write, stdin, stdout};

use rust_calculator::lex::lex_analysis;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let mut buf = String::new();

    loop {
        buf.clear();

        print!("> ");
        stdout().flush()?;
        stdin().read_line(&mut buf)?;

        if buf.is_empty() {
            break;
        }

        let list = lex_analysis(buf.as_str())?;

        if cfg!(debug_assertions) && !list.is_empty() {
            rust_calculator::token::show_token_list(&list);
        }

        // syntax_analysis();
    }

    Ok(())
}
