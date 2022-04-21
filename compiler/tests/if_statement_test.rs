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
    let mut generator = create_generator(&context, "tmp.lht");
    let engine = generator
        .module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    generate_ir_code_jit(&mut generator, &engine, &ast);
}

#[test]
fn minimal_valid_if() {
    let source = "if true {}";
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
fn minimal_valid_if_else() {
    let source = "if true {} else {}";
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
fn return_if_function() {
    let source = "fn f(): number { \
                    if 51 > 42 { \
                        return 51; \
                    } \
                    return 1; \
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
fn return_if_else_function() {
    let source = "fn f(): number { \
                    if 51 > 42 { \
                        return 51; \
                    } else { \
                        return 42; \
                    } \
                    return 1; \
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
fn valid_nested_if() {
    let source = "fn f(): number { \
                    if 51 > 42 { \
                        if true {} else {} \
                    } else { \
                        return 42; \
                    } \
                    return 1; \
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
fn no_condition_if() {
    let source = "if  {}";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    assert!(parser.parse().is_none());
}

#[test]
fn no_body_then_if() {
    let source = "if true ";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    assert!(parser.parse().is_none());
}

#[test]
fn no_body_else_if() {
    let source = "if true {} else";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    assert!(parser.parse().is_none());
}
