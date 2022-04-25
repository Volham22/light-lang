use logos::{Lexer, Logos};

use crate::type_system::value_type::ValueType;

/// Walks the source code until an other " is reached.
/// Then bump the lexer to second " location to resume lexing
/// it acts likes Flex sublexer
fn handle_quote(lex: &mut Lexer<Token>) -> Result<String, ()> {
    let mut inner_content: Vec<char> = Vec::new();
    let remainder_string = lex.remainder();

    for chr in remainder_string.chars() {
        if chr == '"' {
            // Bump to the literal's size + 1 to skip the closing quote
            lex.bump(inner_content.len() + 1);
            return Ok(inner_content.iter().collect());
        }

        inner_content.push(chr);
    }

    eprint!("Error: Unclosed string literal.");
    Err(())
}

#[derive(Logos, Debug)]
pub enum Token {
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("loop")]
    Loop,
    #[token("let")]
    Let,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("fn")]
    Function,
    #[token("return")]
    Return,
    #[token("import")]
    Import,
    #[token("print")]
    Print,
    #[token("=")]
    Equal,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("==")]
    Equality,
    #[token("!=")]
    NegEquality,
    #[token("<")]
    Less,
    #[token(">")]
    More,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    MoreEqual,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("(")]
    LeftParenthesis,
    #[token(")")]
    RightParenthesis,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("\"", handle_quote)]
    Quote(String),

    // Light types
    #[regex("(number)|(real)|(bool)|(string)|(void)", |lex| lex.slice().parse())]
    Type(ValueType),

    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(i64),
    #[regex(r"[0-9][0-9]*(\.[0-9]*)", |lex| lex.slice().parse())]
    Real(f64),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().parse())]
    Identifier(String),

    // End of file
    #[token("\0")]
    EndOfFile,

    // Skip spaces characters and handle error
    #[error]
    #[regex(r"[ \n\t\v\r]", logos::skip)]
    Error,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::If, Token::If) => true,
            (Token::Else, Token::Else) => true,
            (Token::While, Token::While) => true,
            (Token::For, Token::For) => true,
            (Token::Loop, Token::Loop) => true,
            (Token::Let, Token::Let) => true,
            (Token::Break, Token::Break) => true,
            (Token::Continue, Token::Continue) => true,
            (Token::Function, Token::Function) => true,
            (Token::Return, Token::Return) => true,
            (Token::Import, Token::Import) => true,
            (Token::Print, Token::Print) => true,
            (Token::Equal, Token::Equal) => true,
            (Token::Plus, Token::Plus) => true,
            (Token::Minus, Token::Minus) => true,
            (Token::Multiply, Token::Multiply) => true,
            (Token::Divide, Token::Divide) => true,
            (Token::Modulo, Token::Modulo) => true,
            (Token::Not, Token::Not) => true,
            (Token::And, Token::And) => true,
            (Token::Or, Token::Or) => true,
            (Token::Equality, Token::Equality) => true,
            (Token::NegEquality, Token::NegEquality) => true,
            (Token::Less, Token::Less) => true,
            (Token::More, Token::More) => true,
            (Token::LessEqual, Token::LessEqual) => true,
            (Token::MoreEqual, Token::MoreEqual) => true,
            (Token::LeftBracket, Token::LeftBracket) => true,
            (Token::RightBracket, Token::RightBracket) => true,
            (Token::LeftBrace, Token::LeftBrace) => true,
            (Token::RightBrace, Token::RightBrace) => true,
            (Token::LeftParenthesis, Token::LeftParenthesis) => true,
            (Token::RightParenthesis, Token::RightParenthesis) => true,
            (Token::Comma, Token::Comma) => true,
            (Token::Semicolon, Token::Semicolon) => true,
            (Token::Colon, Token::Colon) => true,
            (Token::True, Token::True) => true,
            (Token::False, Token::False) => true,
            (Token::Type(_), Token::Type(_)) => true,
            (Token::Number(_), Token::Number(_)) => true,
            (Token::Real(_), Token::Real(_)) => true,
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (Token::Quote(_), Token::Quote(_)) => true,
            (Token::EndOfFile, Token::EndOfFile) => true,
            (Token::Error, Token::Error) => true,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::type_system::value_type::ValueType;

    use super::Token;
    use logos::Logos;

    #[test]
    fn if_test() {
        let mut lexer = Token::lexer("if () {}");

        assert_eq!(lexer.next(), Some(Token::If));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
    }

    #[test]
    fn for_test() {
        let mut lexer = Token::lexer("for (;;) {}");

        assert_eq!(lexer.next(), Some(Token::For));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
    }

    #[test]
    fn while_test() {
        let mut lexer = Token::lexer("while () {}");

        assert_eq!(lexer.next(), Some(Token::While));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
    }

    #[test]
    fn loop_test() {
        let mut lexer = Token::lexer("loop {}");

        assert_eq!(lexer.next(), Some(Token::Loop));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
    }

    #[test]
    fn let_test() {
        let mut lexer = Token::lexer("let i = 5;");

        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(lexer.next(), Some(Token::Identifier("i".to_string())));
        assert_eq!(lexer.slice(), "i");
        assert_eq!(lexer.next(), Some(Token::Equal));
        assert_eq!(lexer.next(), Some(Token::Number(5)));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn break_test() {
        let mut lexer = Token::lexer("break;");

        assert_eq!(lexer.next(), Some(Token::Break));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn continue_test() {
        let mut lexer = Token::lexer("continue;");

        assert_eq!(lexer.next(), Some(Token::Continue));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn function_test() {
        let mut lexer = Token::lexer("fn hello() {}");

        assert_eq!(lexer.next(), Some(Token::Function));
        assert_eq!(lexer.next(), Some(Token::Identifier("hello".to_string())));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
    }

    #[test]
    fn return_test() {
        let mut lexer = Token::lexer("fn hello() { return 5; }");

        assert_eq!(lexer.next(), Some(Token::Function));
        assert_eq!(lexer.next(), Some(Token::Identifier("hello".to_string())));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::Return));
        assert_eq!(lexer.next(), Some(Token::Number(5)));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
    }

    #[test]
    fn import_test() {
        let mut lexer = Token::lexer("import my_module;");

        assert_eq!(lexer.next(), Some(Token::Import));
        assert_eq!(
            lexer.next(),
            Some(Token::Identifier("my_module".to_string()))
        );
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn print_test() {
        let mut lexer = Token::lexer("print hey;");

        assert_eq!(lexer.next(), Some(Token::Print));
        assert_eq!(lexer.next(), Some(Token::Identifier("hey".to_string())));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn left_bracket_test() {
        let mut lexer = Token::lexer("{{ {    {\t{");

        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
    }

    #[test]
    fn right_bracket_test() {
        let mut lexer = Token::lexer("}}\t\t}   }\n");

        assert_eq!(lexer.next(), Some(Token::RightBrace));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
    }

    #[test]
    fn left_parenthesis_test() {
        let mut lexer = Token::lexer("(( (    (\t(");

        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
    }

    #[test]
    fn right_parenthesis_test() {
        let mut lexer = Token::lexer("))\t\t)   )\n");

        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
    }

    #[test]
    fn true_test() {
        let mut lexer = Token::lexer("true true\ntrue");

        assert_eq!(lexer.next(), Some(Token::True));
        assert_eq!(lexer.next(), Some(Token::True));
        assert_eq!(lexer.next(), Some(Token::True));
    }

    #[test]
    fn false_test() {
        let mut lexer = Token::lexer("false false\nfalse");

        assert_eq!(lexer.next(), Some(Token::False));
        assert_eq!(lexer.next(), Some(Token::False));
        assert_eq!(lexer.next(), Some(Token::False));
    }

    #[test]
    fn plus_test() {
        let mut lexer = Token::lexer("1 + 3 + 42");

        assert_eq!(lexer.next(), Some(Token::Number(1)));
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Number(3)));
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Number(42)));
    }

    #[test]
    fn minus_test() {
        let mut lexer = Token::lexer("1 - 3-42");

        assert_eq!(lexer.next(), Some(Token::Number(1)));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Number(3)));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Number(42)));
    }

    #[test]
    fn multiply_test() {
        let mut lexer = Token::lexer("1 * 3*42");

        assert_eq!(lexer.next(), Some(Token::Number(1)));
        assert_eq!(lexer.next(), Some(Token::Multiply));
        assert_eq!(lexer.next(), Some(Token::Number(3)));
        assert_eq!(lexer.next(), Some(Token::Multiply));
        assert_eq!(lexer.next(), Some(Token::Number(42)));
    }

    #[test]
    fn divide_test() {
        let mut lexer = Token::lexer("1 / 3/42");

        assert_eq!(lexer.next(), Some(Token::Number(1)));
        assert_eq!(lexer.next(), Some(Token::Divide));
        assert_eq!(lexer.next(), Some(Token::Number(3)));
        assert_eq!(lexer.next(), Some(Token::Divide));
        assert_eq!(lexer.next(), Some(Token::Number(42)));
    }

    #[test]
    fn modulo_test() {
        let mut lexer = Token::lexer("1 % 3%42");

        assert_eq!(lexer.next(), Some(Token::Number(1)));
        assert_eq!(lexer.next(), Some(Token::Modulo));
        assert_eq!(lexer.next(), Some(Token::Number(3)));
        assert_eq!(lexer.next(), Some(Token::Modulo));
        assert_eq!(lexer.next(), Some(Token::Number(42)));
    }

    #[test]
    fn and_test() {
        let mut lexer = Token::lexer("true and false");

        assert_eq!(lexer.next(), Some(Token::True));
        assert_eq!(lexer.next(), Some(Token::And));
        assert_eq!(lexer.next(), Some(Token::False));
    }

    #[test]
    fn or_test() {
        let mut lexer = Token::lexer("true or false");

        assert_eq!(lexer.next(), Some(Token::True));
        assert_eq!(lexer.next(), Some(Token::Or));
        assert_eq!(lexer.next(), Some(Token::False));
    }

    #[test]
    fn not_test() {
        let mut lexer = Token::lexer("not (true or not false)");

        assert_eq!(lexer.next(), Some(Token::Not));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::True));
        assert_eq!(lexer.next(), Some(Token::Or));
        assert_eq!(lexer.next(), Some(Token::Not));
        assert_eq!(lexer.next(), Some(Token::False));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
    }

    #[test]
    fn identifier_test() {
        let mut lexer = Token::lexer("let my_super_variable");

        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(
            lexer.next(),
            Some(Token::Identifier("my_super_variable".to_string()))
        );
    }

    #[test]
    fn number_test() {
        let mut lexer = Token::lexer("1442");

        assert_eq!(lexer.next(), Some(Token::Number(1442)));
    }

    #[test]
    fn real_test() {
        let mut lexer = Token::lexer("3.14");

        assert_eq!(lexer.next(), Some(Token::Real(3.14)));
    }

    #[test]
    fn colon_test() {
        let mut lexer = Token::lexer("let my_var: number = 4;");
        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(lexer.next(), Some(Token::Identifier("my_var".to_string())));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Type(ValueType::Number)));
        assert_eq!(lexer.next(), Some(Token::Equal));
        assert_eq!(lexer.next(), Some(Token::Number(4)));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn float_test() {
        let mut lexer = Token::lexer("let pi: real = 3.14;");
        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(lexer.next(), Some(Token::Identifier("pi".to_string())));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Type(ValueType::Real)));
        assert_eq!(lexer.next(), Some(Token::Equal));
        assert_eq!(lexer.next(), Some(Token::Real(3.14)));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn bool_test() {
        let mut lexer = Token::lexer("let truth: bool = true;");
        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(lexer.next(), Some(Token::Identifier("truth".to_string())));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Type(ValueType::Bool)));
        assert_eq!(lexer.next(), Some(Token::Equal));
        assert_eq!(lexer.next(), Some(Token::True));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn fn_test() {
        let mut lexer = Token::lexer("fn hey(mom: string): void {}");
        assert_eq!(lexer.next(), Some(Token::Function));
        assert_eq!(lexer.next(), Some(Token::Identifier("hey".to_string())));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::Identifier("mom".to_string())));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Type(ValueType::String)));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Type(ValueType::Void)));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
    }

    #[test]
    fn quote_test() {
        let mut lexer = Token::lexer("\"\"");
        assert_eq!(lexer.next(), Some(Token::Quote(String::new())));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn quote_test_statement() {
        let mut lexer = Token::lexer("\"word!\";");
        let quote_tk = lexer.next();
        assert_eq!(quote_tk, Some(Token::Quote(String::new())));
        assert_eq!(
            if let Token::Quote(s) = quote_tk.unwrap() {
                s
            } else {
                unreachable!()
            },
            "word!"
        );

        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn quote_with_content_test() {
        let mut lexer = Token::lexer("\"hey mom *&*(^)\"");
        let tk = lexer.next();

        assert!(tk.is_some());
        assert_eq!(
            if let Token::Quote(s) = tk.unwrap() {
                s
            } else {
                unreachable!()
            },
            "hey mom *&*(^)"
        );
    }

    #[test]
    fn unclosed_quote_with_content_test() {
        let mut lexer = Token::lexer("\"hey mom *&*(^)");
        assert_eq!(lexer.next().unwrap(), Token::Error);
    }
}
