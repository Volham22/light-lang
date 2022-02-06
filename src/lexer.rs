use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("if")]
    If,
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
    #[token("{")]
    LeftBracket,
    #[token("}")]
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

    // Light types
    #[token("int")]
    Integer,
    #[token("float")]
    Float,
    #[token("bool")]
    Bool,
    #[token("string")]
    String,

    #[regex(r"[0-9]+")]
    Number,
    #[regex(r"[0-9][0-9]*(\.[0-9]*)")]
    Real,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Skip spaces characters and handle error
    #[error]
    #[regex(r"[ \t\n\f]", logos::skip)]
    Error,
}

#[cfg(test)]
mod tests {
    use super::Token;
    use logos::Logos;

    #[test]
    fn if_test() {
        let mut lexer = Token::lexer("if () {}");

        assert_eq!(lexer.next(), Some(Token::If));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftBracket));
        assert_eq!(lexer.next(), Some(Token::RightBracket));
    }

    #[test]
    fn for_test() {
        let mut lexer = Token::lexer("for (;;) {}");

        assert_eq!(lexer.next(), Some(Token::For));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftBracket));
        assert_eq!(lexer.next(), Some(Token::RightBracket));
    }

    #[test]
    fn while_test() {
        let mut lexer = Token::lexer("while () {}");

        assert_eq!(lexer.next(), Some(Token::While));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftBracket));
        assert_eq!(lexer.next(), Some(Token::RightBracket));
    }

    #[test]
    fn loop_test() {
        let mut lexer = Token::lexer("loop {}");

        assert_eq!(lexer.next(), Some(Token::Loop));
        assert_eq!(lexer.next(), Some(Token::LeftBracket));
        assert_eq!(lexer.next(), Some(Token::RightBracket));
    }

    #[test]
    fn let_test() {
        let mut lexer = Token::lexer("let i = 5;");

        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "i");
        assert_eq!(lexer.next(), Some(Token::Equal));
        assert_eq!(lexer.next(), Some(Token::Number));
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
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "hello");
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftBracket));
        assert_eq!(lexer.next(), Some(Token::RightBracket));
    }

    #[test]
    fn return_test() {
        let mut lexer = Token::lexer("fn hello() { return 5; }");

        assert_eq!(lexer.next(), Some(Token::Function));
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "hello");
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), Some(Token::LeftBracket));
        assert_eq!(lexer.next(), Some(Token::Return));
        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "5");
        assert_eq!(lexer.next(), Some(Token::Semicolon));
        assert_eq!(lexer.next(), Some(Token::RightBracket));
    }

    #[test]
    fn import_test() {
        let mut lexer = Token::lexer("import my_module;");

        assert_eq!(lexer.next(), Some(Token::Import));
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "my_module");
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn print_test() {
        let mut lexer = Token::lexer("print hey;");

        assert_eq!(lexer.next(), Some(Token::Print));
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "hey");
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn left_bracket_test() {
        let mut lexer = Token::lexer("{{ {    {\t{");

        assert_eq!(lexer.next(), Some(Token::LeftBracket));
        assert_eq!(lexer.next(), Some(Token::LeftBracket));
        assert_eq!(lexer.next(), Some(Token::LeftBracket));
        assert_eq!(lexer.next(), Some(Token::LeftBracket));
        assert_eq!(lexer.next(), Some(Token::LeftBracket));
    }

    #[test]
    fn right_bracket_test() {
        let mut lexer = Token::lexer("}}\t\t}   }\n");

        assert_eq!(lexer.next(), Some(Token::RightBracket));
        assert_eq!(lexer.next(), Some(Token::RightBracket));
        assert_eq!(lexer.next(), Some(Token::RightBracket));
        assert_eq!(lexer.next(), Some(Token::RightBracket));
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

        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "1");
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "3");
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "42");
    }

    #[test]
    fn minus_test() {
        let mut lexer = Token::lexer("1 - 3-42");

        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "1");
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "3");
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "42");
    }

    #[test]
    fn multiply_test() {
        let mut lexer = Token::lexer("1 * 3*42");

        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "1");
        assert_eq!(lexer.next(), Some(Token::Multiply));
        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "3");
        assert_eq!(lexer.next(), Some(Token::Multiply));
        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "42");
    }

    #[test]
    fn divide_test() {
        let mut lexer = Token::lexer("1 / 3/42");

        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "1");
        assert_eq!(lexer.next(), Some(Token::Divide));
        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "3");
        assert_eq!(lexer.next(), Some(Token::Divide));
        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "42");
    }

    #[test]
    fn modulo_test() {
        let mut lexer = Token::lexer("1 % 3%42");

        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "1");
        assert_eq!(lexer.next(), Some(Token::Modulo));
        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "3");
        assert_eq!(lexer.next(), Some(Token::Modulo));
        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "42");
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
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "my_super_variable");
    }

    #[test]
    fn number_test() {
        let mut lexer = Token::lexer("1442");

        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "1442");
    }

    #[test]
    fn real_test() {
        let mut lexer = Token::lexer("3.14");

        assert_eq!(lexer.next(), Some(Token::Real));
        assert_eq!(lexer.slice(), "3.14");
    }

    #[test]
    fn colon_test() {
        let mut lexer = Token::lexer("let my_var: int = 4;");
        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "my_var");
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Integer));
        assert_eq!(lexer.next(), Some(Token::Equal));
        assert_eq!(lexer.next(), Some(Token::Number));
        assert_eq!(lexer.slice(), "4");
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn float_test() {
        let mut lexer = Token::lexer("let pi: float = 3.14;");
        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "pi");
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Float));
        assert_eq!(lexer.next(), Some(Token::Equal));
        assert_eq!(lexer.next(), Some(Token::Real));
        assert_eq!(lexer.slice(), "3.14");
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }

    #[test]
    fn bool_test() {
        let mut lexer = Token::lexer("let truth: bool = true;");
        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(lexer.next(), Some(Token::Identifier));
        assert_eq!(lexer.slice(), "truth");
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Bool));
        assert_eq!(lexer.next(), Some(Token::Equal));
        assert_eq!(lexer.next(), Some(Token::True));
        assert_eq!(lexer.next(), Some(Token::Semicolon));
    }
}
