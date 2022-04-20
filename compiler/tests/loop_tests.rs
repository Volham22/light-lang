use compiler::{
    generation::ir_generator::{create_generator, generate_ir_code_jit},
    lexer::Token,
    parser::{parser::Parser, visitors::Statement},
    type_system::type_check::TypeChecker,
};

use inkwell::{context::Context, OptimizationLevel};
use logos::Logos;

fn assert_ir_generation(ast: &Vec<Statement>) {
    // LLVM setup
    let context = Context::create();
    let mut generator = create_generator(&context);
    let engine = generator
        .module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    generate_ir_code_jit(&mut generator, &engine, &ast);
}

#[test]
fn minimal_while() {
    let source = "while false {}";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    if let Some(ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&ast).is_ok());
        assert_ir_generation(&ast);
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn while_10_iteration() {
    let source = "let i: number = 0; \
                  while i < 10 { \
                    i = i + 1; \
                  }";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    if let Some(ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&ast).is_ok());
        assert_ir_generation(&ast);
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn while_no_condition() {
    let source = "while {}";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    assert!(parser.parse().is_none());
}

#[test]
fn while_no_body() {
    let source = "while true";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    assert!(parser.parse().is_none());
}

#[test]
fn while_condition_type_mismatch() {
    let source = "while 3.14 {}";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    if let Some(ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&ast).is_err());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn nested_while_10_iteration() {
    let source = "let i: number = 0; \
                  while i < 10 { \
                    i = i + 1; \
                    while false {} \
                  }";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    if let Some(ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&ast).is_ok());
        assert_ir_generation(&ast);
    } else {
        assert!(false, "Parser failed!");
    }
}
