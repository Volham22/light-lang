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
fn empty_void_function() {
    let source = "fn dummy(): void {}";
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
fn number_return_function() {
    let source = "fn dummy(): number { return 42; }";
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
fn bool_return_function() {
    let source = "fn dummy(): bool { return false; }";
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
fn real_return_function() {
    let source = "fn pi(): real { return 3.14; }";
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
fn void_one_arg_function() {
    let source = "fn hole(n: number): void {}";
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
fn void_multi_args_function() {
    let source = "fn hole(a: number, b: real, c: bool): void {}";
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
fn return_number_arg_function() {
    let source = "fn identity(a: number): number { return a; }";
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
fn return_real_arg_function() {
    let source = "fn identity(a: real): real { return a; }";
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
fn return_bool_arg_function() {
    let source = "fn identity(a: bool): bool { return a; }";
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
fn wrong_return_type() {
    let source = "fn identity(): bool { return 42; }";
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
fn wrong_arg_type() {
    let source = "fn identity(b: bool): bool { return b; } identity(42);";
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
