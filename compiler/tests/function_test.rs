use compiler::{lexer::Token, parser::parser::Parser, type_system::type_check::TypeChecker};

use logos::Logos;

#[test]
fn empty_void_function() {
    let source = "fn dummy(): void {}";
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
fn number_return_function() {
    let source = "fn dummy(): number { return 42; }";
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
fn bool_return_function() {
    let source = "fn dummy(): bool { return false; }";
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
fn real_return_function() {
    let source = "fn pi(): real { return 3.14; }";
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
fn void_one_arg_function() {
    let source = "fn hole(n: number): void {}";
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
fn void_multi_args_function() {
    let source = "fn hole(a: number, b: real, c: bool): void {}";
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
fn return_number_arg_function() {
    let source = "fn identity(a: number): number { return a; }";
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
fn return_real_arg_function() {
    let source = "fn identity(a: real): real { return a; }";
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
fn return_bool_arg_function() {
    let source = "fn identity(a: bool): bool { return a; }";
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

#[test]
fn main_function() {
    let source = "fn main(): number { return 0; }";
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
fn simple_program_with_args() {
    let source =
        "fn add(a: number, b: number): number { return a + b; } fn main(): number { add(1, 1); return 0; }";
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
fn simple_program_without_args() {
    let source = "fn two(): number { return 2; } fn main(): number { two(); return 0; }";
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
fn function_declaration_no_args() {
    let source = "fn do_something(): void;";
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
fn function_declaration_with_args() {
    let source = "fn add(a: number, b: number): number;";
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
fn exported_function_no_args() {
    let source = "export fn do_something(): void {}";
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
fn exported_function_with_args() {
    let source = "export fn add(a: number, b: number): number { return a + b; }";
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
fn exported_function_missing_body() {
    let source = "export fn add(a: number, b: number): number;";
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
fn pass_array_as_parameter() {
    let source = "fn f(a: [number; 10]): void {} fn main(): number {let array: [number; 10] = 42; f(array); return 0;}";
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
fn pass_array_as_parameter_wrong_size() {
    let source = "fn f(a: [number; 5]): void {} fn main(): number {let array: [number; 10] = 42; f(array); return 0;}";
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
fn pass_array_element_as_parameter() {
    let source = "fn f(a: number): void {} fn main(): number {let array: [number; 10] = 42; f(array[1]); return 0;}";
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
