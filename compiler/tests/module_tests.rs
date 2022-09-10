use compiler::{lexer::Token, parser::parser::Parser};

#[test]
fn import_statement_parser() {
    let source = "import \"module\";";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "./module.lht");

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
}

#[test]
fn import_statement_parser_missing_quote() {
    let source = "import module;";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "./module.lht");

    let ast_opt = parser.parse();
    assert!(ast_opt.is_none());
}

#[test]
fn import_statement_parser_missing_semicolon() {
    let source = "import \"module\"";
    let tokens = Token::lex_string(source);

    let mut parser = Parser::new(tokens, "./module.lht");

    let ast_opt = parser.parse();
    assert!(ast_opt.is_none());
}

// #[test]
// fn module_access_parsing() {
//     let source = "module::function();";
//     let tokens = Token::lex_string(source);
//
//     let mut parser = Parser::new(tokens, "./module.lht");
//
//     let ast_opt = parser.parse();
//     assert!(ast_opt.is_some());
// }
