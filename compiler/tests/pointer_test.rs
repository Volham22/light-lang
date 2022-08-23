use compiler::{lexer::Token, parser::parser::Parser, type_system::type_check::TypeChecker};
use logos::Logos;

#[test]
fn simple_pointer_declaration() {
    let source = "let my_ptr: ptr number = null;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn simple_pointer_addrof() {
    let source = "let answer: number = 42; let ans_ptr: ptr number = addrof answer;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn wrong_pointer_addrof() {
    let source = "let answer: number = 42; let ans_ptr: ptr real = addrof answer;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail.");
}

#[test]
fn addrof_of_non_pointer_type() {
    let source = "let answer: number = 42; let ans_ptr: number = addrof answer;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail.");
}

#[test]
fn pointer_dereference_assignment() {
    let source =
        "let answer: number = 42; let ans_ptr: ptr number = addrof answer; deref ans_ptr = 32;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn wrong_type_pointer_dereference_assignment() {
    let source =
        "let answer: number = 42; let ans_ptr: ptr bool = addrof answer; deref ans_ptr = 32;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type checking should fail.");
}

#[test]
fn pointer_array_subscript() {
    // This is valid in type checker and parser side
    // Homewer this will segfault at runtime. This test is just to make
    // sure pointers can be used as arrays
    let source = "let ans_ptr: ptr number = null; ans_ptr[1];";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn void_pointer_init() {
    let source = "let answer: number = 42; let ans_ptr: ptr void = addrof answer;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn pointer_init_from_void() {
    let source = "fn malloc(size: number): ptr void; let dyn_arr: ptr number = malloc(8 * 3);";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn malloc_array_and_assign() {
    let source = "fn malloc(size: number): ptr void; let dyn_arr: ptr number = malloc(8 * 3); dyn_arr[0] = 2;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn valid_deref_lvalue_number() {
    let source = "let a: number = 3; let ptr_a: ptr number = addrof a; (deref ptr_a) = 42;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn dynamic_string_allocation() {
    let source =
        "fn malloc(size: number): ptr void; fn main(): void { let str: string = malloc(100); }";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}
