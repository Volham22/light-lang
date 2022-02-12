use std::io::{self, BufRead, Write};

use logos::Logos;

use crate::generation::ir_generator::generate_ir_code_jit;
use crate::lexer::Token;
use crate::parser::ast_printer::print_expression;
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

            if let Some(expr) = parser.parse() {
                print_expression(&expr);
                generate_ir_code_jit(&expr);
                println!("OK");
            } else {
                println!("Error!");
            }
        } else {
            panic!("{}", line.unwrap_err());
        }

        show_repl();
    }
}
