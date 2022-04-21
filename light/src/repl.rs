use std::io::{self, BufRead, Write};

use inkwell::context::Context;
use inkwell::OptimizationLevel;
use logos::Logos;

use compiler::generation::ir_generator::{create_generator, generate_ir_code_jit};
use compiler::lexer::Token;
use compiler::parser::ast_printer::print_ast;
use compiler::parser::parser::Parser;
use compiler::type_system::type_check::TypeChecker;

fn show_repl() {
    print!("=> ");
    io::stdout().flush().unwrap();
}

pub fn repl_loop() {
    show_repl();
    let mut type_check = TypeChecker::new();
    let context = Context::create();
    let mut generator = create_generator(&context, "main");
    let engine = generator
        .module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(str) = line {
            let lexer = Token::lexer(str.as_str());
            let tokens = lexer.collect();
            let mut parser = Parser::new(tokens);

            if let Some(stmts) = parser.parse() {
                print_ast(&stmts);

                if let Err(msg) = type_check.check_ast_type(&stmts) {
                    println!("Error: {}", msg);
                } else {
                    generate_ir_code_jit(&mut generator, &engine, &stmts);
                    println!("OK");
                }
            } else {
                println!("Error!");
            }
        } else {
            panic!("{}", line.unwrap_err());
        }

        show_repl();
    }
}
