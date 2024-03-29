use std::io::{self, BufRead, Write};

use compiler::desugar::desugar_ast;
use compiler::desugar::import_resolver::ImportResolver;
use inkwell::context::Context;
use inkwell::OptimizationLevel;

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

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(str) = line {
            let tokens = Token::lex_string(&str);
            let mut parser = Parser::new(tokens, "./module.lht", "./module.lht");
            let mut import_resolver = ImportResolver::new();

            if let Some(mut stmts) = parser.parse() {
                print_ast(&stmts);
                match import_resolver.resolve_imports(&mut stmts, "./module.lht") {
                    Ok(r) => stmts = r,
                    Err(msg) => {
                        eprintln!("{}", msg);
                        continue;
                    }
                }

                if let Err(msg) = type_check.check_ast_type(&mut stmts) {
                    println!("Error: {}", msg);
                } else {
                    let mut generator =
                        create_generator(&context, "main", &type_check.get_type_table());
                    let engine = generator
                        .module
                        .create_jit_execution_engine(OptimizationLevel::None)
                        .unwrap();
                    desugar_ast(&mut stmts);
                    print_ast(&stmts);
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
