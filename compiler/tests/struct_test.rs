use compiler::{lexer::Token, parser::parser::Parser, type_system::type_check::TypeChecker};

#[test]
fn simple_struct_declaration() {
    let source = "struct S { count: number; }";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "", "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn simple_struct_init() {
    let source = "struct S { count: number; } fn main(): void { let s: S = struct S { 0 }; }";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "", "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn struct_init_undeclared_type() {
    let source = "struct S { count: number; } fn main(): void { let s: S = struct Client { 0 }; }";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "", "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail!");
}

#[test]
fn struct_init_bad_type() {
    let source = "struct S { count: number; } fn main(): void { let s: S = false; }";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "", "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail!");
}

#[test]
fn struct_init_no_init_exps() {
    let source = "struct S { count: number; } fn main(): void { let s: S = struct S {}; }";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "", "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail!");
}

#[test]
fn struct_init_bad_type_init_exp() {
    let source = "struct S { count: number; } fn main(): void { let s: S = struct S { false }; }";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "", "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail!");
}

#[test]
fn struct_init_too_much_exps() {
    let source =
        "struct S { count: number; } fn main(): void { let s: S = struct S { false, 17 }; }";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "", "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail!");
}

#[test]
fn valid_struct_member_access() {
    let source =
        "struct S { count: number; } fn main(): void { let s: S = struct S { 0 }; s.count; }";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "", "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}

#[test]
fn struct_member_access_wrong_field() {
    let source =
        "struct S { count: number; } fn main(): void { let s: S = struct S { 0 }; s.age; }";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "", "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_err(), "Type check should fail!");
}

#[test]
fn struct_member_access_wrong_type() {
    let source = "struct S { count: number; } fn main(): void { let s: S = struct S { 0 }; s.42; }";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "", "");

    let ast_opt = parser.parse();
    assert!(ast_opt.is_none());
}

#[test]
fn struct_member_assign() {
    let source =
        "struct S { count: number; } fn main(): void { let s: S = struct S { 0 }; s.count = 3; }";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "", "");
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&mut ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}
