use compiler::{lexer::Token, parser::parser::Parser, type_system::type_check::TypeChecker};
use logos::Logos;

#[test]
fn simple_struct_declaration() {
    let source = "struct S { count: number; }";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn simple_struct_init() {
    let source = "struct S { count: number; } fn main(): void { let s: S = struct S { 0 }; }";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn struct_init_undeclared_type() {
    let source = "struct S { count: number; } fn main(): void { let s: S = struct Client { 0 }; }";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail!");
}

#[test]
fn struct_init_bad_type() {
    let source = "struct S { count: number; } fn main(): void { let s: S = false; }";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail!");
}

#[test]
fn struct_init_no_init_exps() {
    let source = "struct S { count: number; } fn main(): void { let s: S = struct S {}; }";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail!");
}

#[test]
fn struct_init_bad_type_init_exp() {
    let source = "struct S { count: number; } fn main(): void { let s: S = struct S { false }; }";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail!");
}

#[test]
fn struct_init_too_much_exps() {
    let source =
        "struct S { count: number; } fn main(): void { let s: S = struct S { false, 17 }; }";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail!");
}
