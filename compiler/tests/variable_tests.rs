use compiler::{lexer::Token, parser::parser::Parser, type_system::type_check::TypeChecker};
use logos::Logos;

#[test]
fn declare_number() {
    let source = "let a: number = 42;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn declare_real() {
    let source = "let a: real = 3.14;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn declare_bool() {
    let source = "let a: bool = false;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn declare_init_type_mismatch() {
    let source = "let a: bool = 22;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_err());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn declare_no_init() {
    let source = "let a: bool;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    assert!(parser.parse().is_none());
}

#[test]
fn declare_no_type() {
    let source = "let a = 42;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    assert!(parser.parse().is_none());
}

#[test]
fn assign_valid_number() {
    let source = "let a: number = 42; a = 51;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn assign_valid_real() {
    let source = "let a: real = 42.45; a = 51.1;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn assign_valid_bool() {
    let source = "let a: bool = false; a = true;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn assign_type_mismatch() {
    let source = "let a: bool = false; a = 2;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_err());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn undeclared_variable() {
    let source = "a = 2;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_err());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn parse_valid_int_array_declaration() {
    let source = "let arr: [number; 10] = 0;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    assert!(parser.parse().is_some());
}

#[test]
fn parse_invalid_type_position_array_declaration() {
    let source = "let arr: [10; number] = 0;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    assert!(parser.parse().is_none());
}

#[test]
fn parse_unclosed_array_declaration() {
    let source = "let arr: [number; 10 = 0;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    assert!(parser.parse().is_none());
}

#[test]
fn parse_missing_semicolon_array_declaration() {
    let source = "let arr: [number 10] = 0;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    assert!(parser.parse().is_none());
}

#[test]
fn parse_valid_array_access() {
    let source = "let arr: [number; 10] = 0; arr[0];";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    assert!(parser.parse().is_some());
}

#[test]
fn parse_unclosed_bracket_array_access() {
    let source = "let arr: [number; 10] = 0; arr[0;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    assert!(parser.parse().is_none());
}

#[test]
fn array_init_valid_type_check() {
    let source = "let arr: [number; 10] = 10;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn array_init_valid_type_check_real() {
    let source = "let arr: [real; 10] = 10.2;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn array_init_invalid_type_check() {
    let source = "let arr: [real; 10] = false;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_err());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn array_valid_type_check_access() {
    let source = "let arr: [real; 10] = 10.2; let b: real = arr[0];";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn array_valid_type_check_assign() {
    let source = "let arr: [real; 10] = 10.2; arr[0] = 3.14;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn array_invalid_type_check_access() {
    let source = "let arr: [real; 10] = 10.2; let b: bool = arr[0];";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_err());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn array_invalid_type_check_assign() {
    let source = "let arr: [real; 10] = 10.2; arr[0] = false;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_err());
    } else {
        assert!(false, "Parser failed!");
    }
}

#[test]
fn string_declaration_valid() {
    let source = "let words: string = \"word!\";";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "");

    if let Some(mut ast) = parser.parse() {
        let mut type_check = TypeChecker::new();
        assert!(type_check.check_ast_type(&mut ast).is_ok());
    } else {
        assert!(false, "Parser failed!");
    }
}
