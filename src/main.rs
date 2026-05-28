use std::io::{Write, stdin, stdout};

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let mut buf = String::new();

    loop {
        print!("> ");
        stdout().flush()?;
        stdin().read_line(&mut buf)?;

        if buf.is_empty() {
            break;
        }

        if lex_analysis(buf.trim_end()) < 0 {
            eprintln!("Unknown symbols included");
            buf.clear();
            continue;
        }

        syntax_analysis();
        buf.clear();
    }

    Ok(())
}

fn lex_analysis(buf: &str) -> i32 {
    println!("buf: {buf}");
    1
}

fn syntax_analysis() -> i32 {
    1
}
