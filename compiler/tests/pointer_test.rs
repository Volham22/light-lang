use compiler::{lexer::Token, parser::parser::Parser, type_system::type_check::TypeChecker};
use logos::Logos;

#[test]
fn simple_pointer_declaration() {
    let source = "let my_ptr: ptr number = null;";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens);
    let mut type_check = TypeChecker::new();

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
    let tc_result = type_check.check_ast_type(&ast_opt.unwrap());
    assert!(tc_result.is_ok(), "Type error: {}", tc_result.unwrap_err());
}
