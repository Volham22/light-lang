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
fn declare_number() {
    let source = "let a: number = 42;";
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
fn declare_real() {
    let source = "let a: real = 3.14;";
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
fn declare_bool() {
    let source = "let a: bool = false;";
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
fn declare_init_type_mismatch() {
    let source = "let a: bool = 22;";
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
fn declare_no_init() {
    let source = "let a: bool;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    assert!(parser.parse().is_none());
}

#[test]
fn declare_no_type() {
    let source = "let a = 42;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    assert!(parser.parse().is_none());
}

#[test]
fn assign_valid_number() {
    let source = "let a: number = 42; a = 51;";
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
fn assign_valid_real() {
    let source = "let a: real = 42.45; a = 51.1;";
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
fn assign_valid_bool() {
    let source = "let a: bool = false; a = true;";
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
fn assign_type_mismatch() {
    let source = "let a: bool = false; a = 2;";
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
fn undeclared_variable() {
    let source = "a = 2;";
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
