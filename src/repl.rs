use std::io::{self, BufRead, Write};

use logos::Logos;

use crate::lexer::Token;
use crate::parser::parser::Parser;

fn show_repl() {
    print!("=> ");
    io::stdout().flush().unwrap();
}

pub fn repl_loop() {
    show_repl();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(str) = line {
            let lexer = Token::lexer(str.as_str());
            let tokens = lexer.collect();
            let mut parser = Parser::new(tokens);

            if parser.parse() {
                println!("OK");
            }
        } else {
            panic!("{}", line.unwrap_err());
        }

        show_repl();
    }
}
