use compiler::{lexer::Token, parser::parser::Parser, type_system::type_check::TypeChecker};

use logos::Logos;

#[test]
fn minimal_valid_if() {
    let source = "if true {}";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
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

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
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

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
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

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
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

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
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
