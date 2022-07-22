use compiler::{
    desugar::desugar_ast,
    lexer::Token,
    parser::{parser::Parser, visitors::Statement},
    type_system::type_check::TypeChecker,
};

use logos::Logos;

#[test]
fn minimal_while() {
    let source = "while false {}";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    if let Some(ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&ast).is_ok());
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
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn simple_for_loop() {
    let source = "for let i: number = 0; i < 10; i = i + 1; {}";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    if let Some(ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn simple_for_loop_with_body() {
    let source = "for let i: number = 0; i < 10; i = i + 1; { i == 1; }";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    if let Some(ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn for_desugar_with_body() {
    let source = "for let i: number = 0; i < 10; i = i + 1; { i == 1; }";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&ast).is_ok());
        desugar_ast(&mut ast);

        if let Statement::ForStatement(_) = &ast[0] {
            assert!(false);
        }
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn for_desugar_no_body() {
    let source = "for let i: number = 0; i < 10; i = i + 1; {}";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&ast).is_ok());
        desugar_ast(&mut ast);

        if let Statement::ForStatement(_) = &ast[0] {
            assert!(false);
        }
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn for_missing_semicolon() {
    let source = "for let i: number = 0 i < 10; i = i + 1; {}";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    assert!(parser.parse().is_none());
}

#[test]
fn for_no_loop_condition() {
    let source = "for let i: number = 0; i = i + 1; {}";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    assert!(parser.parse().is_none());
}

#[test]
fn for_no_next_statement() {
    let source = "for let i: number = 0; i < 10; {}";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);

    assert!(parser.parse().is_none());
}

#[test]
fn two_for_statement() {
    let source =
        "for let i: number = 0; i < 10; i = i + 1; {} for let i: number = 0; i < 10; i = i + 1; {}";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let mut checker = TypeChecker::new();
    let stmts = parser.parse();

    assert!(stmts.is_some());
    assert!(checker.check_ast_type(&stmts.unwrap()).is_ok());
}

#[test]
fn parse_loop_statement() {
    let source = "loop {}";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let mut checker = TypeChecker::new();
    let stmts = parser.parse();

    assert!(stmts.is_some());
    assert!(checker.check_ast_type(&stmts.unwrap()).is_ok());
}

#[test]
fn parse_loop_statement_with_break() {
    let source = "loop { break; }";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let mut checker = TypeChecker::new();
    let stmts = parser.parse();

    assert!(stmts.is_some());
    assert!(checker.check_ast_type(&stmts.unwrap()).is_ok());
}

#[test]
fn parse_break_outside_loop() {
    let source = "loop {} break;";

    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let mut checker = TypeChecker::new();
    let stmts = parser.parse();

    assert!(stmts.is_some());
    assert!(checker.check_ast_type(&stmts.unwrap()).is_err());
}
