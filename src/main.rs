use std::io::{Write, stdin, stdout};

use rust_calculator::lex::lex_analysis;
use rust_calculator::syntax::syntax_analysis;

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

        match lex_analysis(buf.as_str()) {
            Ok(token_list) => match syntax_analysis(token_list) {
                Ok(_) => continue,
                Err(e) => {
                    eprintln!("{e}");
                }
            },
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }

    Ok(())
}
