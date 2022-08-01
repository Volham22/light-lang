use compiler::{lexer::Token, parser::parser::Parser};
use logos::Logos;

#[test]
fn import_statement_parser() {
    let source = "import \"module\";";
    let lexer = Token::lexer(source);
    let tokens = lexer.collect();
    let mut parser = Parser::new(tokens, "./module.lht");

    let ast_opt = parser.parse();
    assert!(ast_opt.is_some());
}
