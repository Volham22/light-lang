use logos::{Lexer, Logos, Skip};

use crate::type_system::value_type::ValueType;

/// Walks the source code until an other " is reached.
/// Then bump the lexer to second " location to resume lexing
/// it acts likes Flex sublexer
fn handle_quote(lex: &mut Lexer<LogosToken>) -> Result<String, ()> {
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

fn handle_single_quote(lex: &mut Lexer<LogosToken>) -> Result<char, ()> {
    let mut remainder_string = lex.remainder().chars();

    if let Some(content) = remainder_string.next() {
        if let Some(end_quote) = remainder_string.next() {
            if end_quote == '\'' {
                lex.bump(2);
                Ok(content)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

fn handle_comment(lex: &mut Lexer<LogosToken>) -> Skip {
    let mut chars_to_bump: usize = 0;

    for chr in lex.remainder().chars() {
        if chr == '\n' {
            lex.bump(chars_to_bump + 1);
            return Skip {};
        }

        chars_to_bump += 1;
    }

    Skip {}
}

fn handle_newline(lex: &mut Lexer<LogosToken>) -> Skip {
    // We increase the line count and set its position in the source code
    lex.extras.increase_line_number(lex.span().end);

    Skip {}
}

pub struct TokenInfo {
    pub line_count: usize,
    last_newline_position: usize,
}

impl TokenInfo {
    pub fn new() -> Self {
        TokenInfo {
            line_count: 0,
            last_newline_position: 0,
        }
    }

    pub fn increase_line_number(&mut self, position: usize) {
        self.line_count += 1;
        self.last_newline_position = position;
    }

    pub fn last_newline_index(&self, tk_begin_position: usize) -> usize {
        tk_begin_position - self.last_newline_position
    }
}

impl Default for TokenInfo {
    fn default() -> Self {
        Self {
            line_count: Default::default(),
            last_newline_position: Default::default(),
        }
    }
}

pub struct Token {
    pub logos_tk: LogosToken,
    pub line_number: usize,
    pub column_number: usize,
}

impl Token {
    pub fn lex_string(string: &str) -> Vec<Self> {
        let mut lexer = LogosToken::lexer_with_extras(string, TokenInfo::new());
        let mut result: Vec<Self> = Vec::new();

        while let Some(tk) = lexer.next() {
            let line_number = lexer.extras.line_count + 1;
            result.push(Self {
                logos_tk: tk,
                line_number,
                column_number: lexer.extras.last_newline_index(lexer.span().start),
            })
        }

        result
    }
}

#[derive(Logos, Debug)]
#[logos(extras = TokenInfo)]
pub enum LogosToken {
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
    #[token("struct")]
    Struct,
    #[token("export")]
    Export,
    #[token("return")]
    Return,
    #[token("import")]
    Import,
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
    #[token(".")]
    Dot,
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
    #[token("::")]
    DoubleColon,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("\"", handle_quote)]
    Quote(String),
    #[token("'", handle_single_quote)]
    CharLiteral(char),

    // Pointer keywords
    #[token("ptr")]
    Pointer,
    #[token("deref")]
    Dereference,
    #[token("addrof")]
    AddressOf,
    #[token("null")]
    Null,

    // Light types
    #[regex("(number)|(real)|(bool)|(string)|(void)|(char)", |lex| lex.slice().parse())]
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

    #[token("//", handle_comment)]
    Comment,

    // Used to count lines in source code, this token is skipped but the callback
    // is used to increase the line count
    #[token("\n", handle_newline)]
    NewLine,

    // Skip spaces characters and handle error
    #[error]
    #[regex(r"[ \t\v\r]", logos::skip)]
    Error,
}

impl PartialEq for LogosToken {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LogosToken::If, LogosToken::If) => true,
            (LogosToken::DoubleColon, LogosToken::DoubleColon) => true,
            (LogosToken::Else, LogosToken::Else) => true,
            (LogosToken::While, LogosToken::While) => true,
            (LogosToken::For, LogosToken::For) => true,
            (LogosToken::Loop, LogosToken::Loop) => true,
            (LogosToken::Let, LogosToken::Let) => true,
            (LogosToken::Break, LogosToken::Break) => true,
            (LogosToken::Continue, LogosToken::Continue) => true,
            (LogosToken::Function, LogosToken::Function) => true,
            (LogosToken::Export, LogosToken::Export) => true,
            (LogosToken::Return, LogosToken::Return) => true,
            (LogosToken::Import, LogosToken::Import) => true,
            (LogosToken::Equal, LogosToken::Equal) => true,
            (LogosToken::Plus, LogosToken::Plus) => true,
            (LogosToken::Minus, LogosToken::Minus) => true,
            (LogosToken::Multiply, LogosToken::Multiply) => true,
            (LogosToken::Divide, LogosToken::Divide) => true,
            (LogosToken::Modulo, LogosToken::Modulo) => true,
            (LogosToken::Not, LogosToken::Not) => true,
            (LogosToken::And, LogosToken::And) => true,
            (LogosToken::Or, LogosToken::Or) => true,
            (LogosToken::Equality, LogosToken::Equality) => true,
            (LogosToken::NegEquality, LogosToken::NegEquality) => true,
            (LogosToken::Less, LogosToken::Less) => true,
            (LogosToken::More, LogosToken::More) => true,
            (LogosToken::LessEqual, LogosToken::LessEqual) => true,
            (LogosToken::MoreEqual, LogosToken::MoreEqual) => true,
            (LogosToken::LeftBracket, LogosToken::LeftBracket) => true,
            (LogosToken::RightBracket, LogosToken::RightBracket) => true,
            (LogosToken::LeftBrace, LogosToken::LeftBrace) => true,
            (LogosToken::RightBrace, LogosToken::RightBrace) => true,
            (LogosToken::LeftParenthesis, LogosToken::LeftParenthesis) => true,
            (LogosToken::RightParenthesis, LogosToken::RightParenthesis) => true,
            (LogosToken::Comma, LogosToken::Comma) => true,
            (LogosToken::Semicolon, LogosToken::Semicolon) => true,
            (LogosToken::Colon, LogosToken::Colon) => true,
            (LogosToken::True, LogosToken::True) => true,
            (LogosToken::False, LogosToken::False) => true,
            (LogosToken::Type(_), LogosToken::Type(_)) => true,
            (LogosToken::Number(_), LogosToken::Number(_)) => true,
            (LogosToken::Real(_), LogosToken::Real(_)) => true,
            (LogosToken::Identifier(_), LogosToken::Identifier(_)) => true,
            (LogosToken::Quote(_), LogosToken::Quote(_)) => true,
            (LogosToken::CharLiteral(_), LogosToken::CharLiteral(_)) => true,
            (LogosToken::EndOfFile, LogosToken::EndOfFile) => true,
            (LogosToken::Pointer, LogosToken::Pointer) => true,
            (LogosToken::AddressOf, LogosToken::AddressOf) => true,
            (LogosToken::Dereference, LogosToken::Dereference) => true,
            (LogosToken::Struct, LogosToken::Struct) => true,
            (LogosToken::Dot, LogosToken::Dot) => true,
            (LogosToken::Error, LogosToken::Error) => true,
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

    use super::LogosToken;
    use logos::Logos;

    #[test]
    fn if_test() {
        let mut lexer = LogosToken::lexer("if () {}");

        assert_eq!(lexer.next(), Some(LogosToken::If));
        assert_eq!(lexer.next(), Some(LogosToken::LeftParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::RightParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::LeftBrace));
        assert_eq!(lexer.next(), Some(LogosToken::RightBrace));
    }

    #[test]
    fn for_test() {
        let mut lexer = LogosToken::lexer("for (;;) {}");

        assert_eq!(lexer.next(), Some(LogosToken::For));
        assert_eq!(lexer.next(), Some(LogosToken::LeftParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::Semicolon));
        assert_eq!(lexer.next(), Some(LogosToken::Semicolon));
        assert_eq!(lexer.next(), Some(LogosToken::RightParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::LeftBrace));
        assert_eq!(lexer.next(), Some(LogosToken::RightBrace));
    }

    #[test]
    fn while_test() {
        let mut lexer = LogosToken::lexer("while () {}");

        assert_eq!(lexer.next(), Some(LogosToken::While));
        assert_eq!(lexer.next(), Some(LogosToken::LeftParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::RightParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::LeftBrace));
        assert_eq!(lexer.next(), Some(LogosToken::RightBrace));
    }

    #[test]
    fn loop_test() {
        let mut lexer = LogosToken::lexer("loop {}");

        assert_eq!(lexer.next(), Some(LogosToken::Loop));
        assert_eq!(lexer.next(), Some(LogosToken::LeftBrace));
        assert_eq!(lexer.next(), Some(LogosToken::RightBrace));
    }

    #[test]
    fn let_test() {
        let mut lexer = LogosToken::lexer("let i = 5;");

        assert_eq!(lexer.next(), Some(LogosToken::Let));
        assert_eq!(lexer.next(), Some(LogosToken::Identifier("i".to_string())));
        assert_eq!(lexer.slice(), "i");
        assert_eq!(lexer.next(), Some(LogosToken::Equal));
        assert_eq!(lexer.next(), Some(LogosToken::Number(5)));
        assert_eq!(lexer.next(), Some(LogosToken::Semicolon));
    }

    #[test]
    fn break_test() {
        let mut lexer = LogosToken::lexer("break;");

        assert_eq!(lexer.next(), Some(LogosToken::Break));
        assert_eq!(lexer.next(), Some(LogosToken::Semicolon));
    }

    #[test]
    fn continue_test() {
        let mut lexer = LogosToken::lexer("continue;");

        assert_eq!(lexer.next(), Some(LogosToken::Continue));
        assert_eq!(lexer.next(), Some(LogosToken::Semicolon));
    }

    #[test]
    fn function_test() {
        let mut lexer = LogosToken::lexer("fn hello() {}");

        assert_eq!(lexer.next(), Some(LogosToken::Function));
        assert_eq!(
            lexer.next(),
            Some(LogosToken::Identifier("hello".to_string()))
        );
        assert_eq!(lexer.next(), Some(LogosToken::LeftParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::RightParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::LeftBrace));
        assert_eq!(lexer.next(), Some(LogosToken::RightBrace));
    }

    #[test]
    fn return_test() {
        let mut lexer = LogosToken::lexer("fn hello() { return 5; }");

        assert_eq!(lexer.next(), Some(LogosToken::Function));
        assert_eq!(
            lexer.next(),
            Some(LogosToken::Identifier("hello".to_string()))
        );
        assert_eq!(lexer.next(), Some(LogosToken::LeftParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::RightParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::LeftBrace));
        assert_eq!(lexer.next(), Some(LogosToken::Return));
        assert_eq!(lexer.next(), Some(LogosToken::Number(5)));
        assert_eq!(lexer.next(), Some(LogosToken::Semicolon));
        assert_eq!(lexer.next(), Some(LogosToken::RightBrace));
    }

    #[test]
    fn import_test() {
        let mut lexer = LogosToken::lexer("import my_module;");

        assert_eq!(lexer.next(), Some(LogosToken::Import));
        assert_eq!(
            lexer.next(),
            Some(LogosToken::Identifier("my_module".to_string()))
        );
        assert_eq!(lexer.next(), Some(LogosToken::Semicolon));
    }

    #[test]
    fn left_bracket_test() {
        let mut lexer = LogosToken::lexer("{{ {    {\t{");

        assert_eq!(lexer.next(), Some(LogosToken::LeftBrace));
        assert_eq!(lexer.next(), Some(LogosToken::LeftBrace));
        assert_eq!(lexer.next(), Some(LogosToken::LeftBrace));
        assert_eq!(lexer.next(), Some(LogosToken::LeftBrace));
        assert_eq!(lexer.next(), Some(LogosToken::LeftBrace));
    }

    #[test]
    fn right_bracket_test() {
        let mut lexer = LogosToken::lexer("}}\t\t}   }\n");

        assert_eq!(lexer.next(), Some(LogosToken::RightBrace));
        assert_eq!(lexer.next(), Some(LogosToken::RightBrace));
        assert_eq!(lexer.next(), Some(LogosToken::RightBrace));
        assert_eq!(lexer.next(), Some(LogosToken::RightBrace));
    }

    #[test]
    fn left_parenthesis_test() {
        let mut lexer = LogosToken::lexer("(( (    (\t(");

        assert_eq!(lexer.next(), Some(LogosToken::LeftParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::LeftParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::LeftParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::LeftParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::LeftParenthesis));
    }

    #[test]
    fn right_parenthesis_test() {
        let mut lexer = LogosToken::lexer("))\t\t)   )\n");

        assert_eq!(lexer.next(), Some(LogosToken::RightParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::RightParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::RightParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::RightParenthesis));
    }

    #[test]
    fn true_test() {
        let mut lexer = LogosToken::lexer("true true\ntrue");

        assert_eq!(lexer.next(), Some(LogosToken::True));
        assert_eq!(lexer.next(), Some(LogosToken::True));
        assert_eq!(lexer.next(), Some(LogosToken::True));
    }

    #[test]
    fn false_test() {
        let mut lexer = LogosToken::lexer("false false\nfalse");

        assert_eq!(lexer.next(), Some(LogosToken::False));
        assert_eq!(lexer.next(), Some(LogosToken::False));
        assert_eq!(lexer.next(), Some(LogosToken::False));
    }

    #[test]
    fn plus_test() {
        let mut lexer = LogosToken::lexer("1 + 3 + 42");

        assert_eq!(lexer.next(), Some(LogosToken::Number(1)));
        assert_eq!(lexer.next(), Some(LogosToken::Plus));
        assert_eq!(lexer.next(), Some(LogosToken::Number(3)));
        assert_eq!(lexer.next(), Some(LogosToken::Plus));
        assert_eq!(lexer.next(), Some(LogosToken::Number(42)));
    }

    #[test]
    fn minus_test() {
        let mut lexer = LogosToken::lexer("1 - 3-42");

        assert_eq!(lexer.next(), Some(LogosToken::Number(1)));
        assert_eq!(lexer.next(), Some(LogosToken::Minus));
        assert_eq!(lexer.next(), Some(LogosToken::Number(3)));
        assert_eq!(lexer.next(), Some(LogosToken::Minus));
        assert_eq!(lexer.next(), Some(LogosToken::Number(42)));
    }

    #[test]
    fn multiply_test() {
        let mut lexer = LogosToken::lexer("1 * 3*42");

        assert_eq!(lexer.next(), Some(LogosToken::Number(1)));
        assert_eq!(lexer.next(), Some(LogosToken::Multiply));
        assert_eq!(lexer.next(), Some(LogosToken::Number(3)));
        assert_eq!(lexer.next(), Some(LogosToken::Multiply));
        assert_eq!(lexer.next(), Some(LogosToken::Number(42)));
    }

    #[test]
    fn divide_test() {
        let mut lexer = LogosToken::lexer("1 / 3/42");

        assert_eq!(lexer.next(), Some(LogosToken::Number(1)));
        assert_eq!(lexer.next(), Some(LogosToken::Divide));
        assert_eq!(lexer.next(), Some(LogosToken::Number(3)));
        assert_eq!(lexer.next(), Some(LogosToken::Divide));
        assert_eq!(lexer.next(), Some(LogosToken::Number(42)));
    }

    #[test]
    fn modulo_test() {
        let mut lexer = LogosToken::lexer("1 % 3%42");

        assert_eq!(lexer.next(), Some(LogosToken::Number(1)));
        assert_eq!(lexer.next(), Some(LogosToken::Modulo));
        assert_eq!(lexer.next(), Some(LogosToken::Number(3)));
        assert_eq!(lexer.next(), Some(LogosToken::Modulo));
        assert_eq!(lexer.next(), Some(LogosToken::Number(42)));
    }

    #[test]
    fn and_test() {
        let mut lexer = LogosToken::lexer("true and false");

        assert_eq!(lexer.next(), Some(LogosToken::True));
        assert_eq!(lexer.next(), Some(LogosToken::And));
        assert_eq!(lexer.next(), Some(LogosToken::False));
    }

    #[test]
    fn or_test() {
        let mut lexer = LogosToken::lexer("true or false");

        assert_eq!(lexer.next(), Some(LogosToken::True));
        assert_eq!(lexer.next(), Some(LogosToken::Or));
        assert_eq!(lexer.next(), Some(LogosToken::False));
    }

    #[test]
    fn not_test() {
        let mut lexer = LogosToken::lexer("not (true or not false)");

        assert_eq!(lexer.next(), Some(LogosToken::Not));
        assert_eq!(lexer.next(), Some(LogosToken::LeftParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::True));
        assert_eq!(lexer.next(), Some(LogosToken::Or));
        assert_eq!(lexer.next(), Some(LogosToken::Not));
        assert_eq!(lexer.next(), Some(LogosToken::False));
        assert_eq!(lexer.next(), Some(LogosToken::RightParenthesis));
    }

    #[test]
    fn identifier_test() {
        let mut lexer = LogosToken::lexer("let my_super_variable");

        assert_eq!(lexer.next(), Some(LogosToken::Let));
        assert_eq!(
            lexer.next(),
            Some(LogosToken::Identifier("my_super_variable".to_string()))
        );
    }

    #[test]
    fn number_test() {
        let mut lexer = LogosToken::lexer("1442");

        assert_eq!(lexer.next(), Some(LogosToken::Number(1442)));
    }

    #[test]
    fn real_test() {
        let mut lexer = LogosToken::lexer("3.14");

        assert_eq!(lexer.next(), Some(LogosToken::Real(3.14)));
    }

    #[test]
    fn colon_test() {
        let mut lexer = LogosToken::lexer("let my_var: number = 4;");
        assert_eq!(lexer.next(), Some(LogosToken::Let));
        assert_eq!(
            lexer.next(),
            Some(LogosToken::Identifier("my_var".to_string()))
        );
        assert_eq!(lexer.next(), Some(LogosToken::Colon));
        assert_eq!(lexer.next(), Some(LogosToken::Type(ValueType::Number)));
        assert_eq!(lexer.next(), Some(LogosToken::Equal));
        assert_eq!(lexer.next(), Some(LogosToken::Number(4)));
        assert_eq!(lexer.next(), Some(LogosToken::Semicolon));
    }

    #[test]
    fn float_test() {
        let mut lexer = LogosToken::lexer("let pi: real = 3.14;");
        assert_eq!(lexer.next(), Some(LogosToken::Let));
        assert_eq!(lexer.next(), Some(LogosToken::Identifier("pi".to_string())));
        assert_eq!(lexer.next(), Some(LogosToken::Colon));
        assert_eq!(lexer.next(), Some(LogosToken::Type(ValueType::Real)));
        assert_eq!(lexer.next(), Some(LogosToken::Equal));
        assert_eq!(lexer.next(), Some(LogosToken::Real(3.14)));
        assert_eq!(lexer.next(), Some(LogosToken::Semicolon));
    }

    #[test]
    fn bool_test() {
        let mut lexer = LogosToken::lexer("let truth: bool = true;");
        assert_eq!(lexer.next(), Some(LogosToken::Let));
        assert_eq!(
            lexer.next(),
            Some(LogosToken::Identifier("truth".to_string()))
        );
        assert_eq!(lexer.next(), Some(LogosToken::Colon));
        assert_eq!(lexer.next(), Some(LogosToken::Type(ValueType::Bool)));
        assert_eq!(lexer.next(), Some(LogosToken::Equal));
        assert_eq!(lexer.next(), Some(LogosToken::True));
        assert_eq!(lexer.next(), Some(LogosToken::Semicolon));
    }

    #[test]
    fn fn_test() {
        let mut lexer = LogosToken::lexer("fn hey(mom: string): void {}");
        assert_eq!(lexer.next(), Some(LogosToken::Function));
        assert_eq!(
            lexer.next(),
            Some(LogosToken::Identifier("hey".to_string()))
        );
        assert_eq!(lexer.next(), Some(LogosToken::LeftParenthesis));
        assert_eq!(
            lexer.next(),
            Some(LogosToken::Identifier("mom".to_string()))
        );
        assert_eq!(lexer.next(), Some(LogosToken::Colon));
        assert_eq!(lexer.next(), Some(LogosToken::Type(ValueType::String)));
        assert_eq!(lexer.next(), Some(LogosToken::RightParenthesis));
        assert_eq!(lexer.next(), Some(LogosToken::Colon));
        assert_eq!(lexer.next(), Some(LogosToken::Type(ValueType::Void)));
        assert_eq!(lexer.next(), Some(LogosToken::LeftBrace));
        assert_eq!(lexer.next(), Some(LogosToken::RightBrace));
    }

    #[test]
    fn quote_test() {
        let mut lexer = LogosToken::lexer("\"\"");
        assert_eq!(lexer.next(), Some(LogosToken::Quote(String::new())));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn quote_test_statement() {
        let mut lexer = LogosToken::lexer("\"word!\";");
        let quote_tk = lexer.next();
        assert_eq!(quote_tk, Some(LogosToken::Quote(String::new())));
        assert_eq!(
            if let LogosToken::Quote(s) = quote_tk.unwrap() {
                s
            } else {
                unreachable!()
            },
            "word!"
        );

        assert_eq!(lexer.next(), Some(LogosToken::Semicolon));
    }

    #[test]
    fn quote_with_content_test() {
        let mut lexer = LogosToken::lexer("\"hey mom *&*(^)\"");
        let tk = lexer.next();

        assert!(tk.is_some());
        assert_eq!(
            if let LogosToken::Quote(s) = tk.unwrap() {
                s
            } else {
                unreachable!()
            },
            "hey mom *&*(^)"
        );
    }

    #[test]
    fn unclosed_quote_with_content_test() {
        let mut lexer = LogosToken::lexer("\"hey mom *&*(^)");
        assert_eq!(lexer.next().unwrap(), LogosToken::Error);
    }

    #[test]
    fn ptr_keyword_test() {
        let mut lexer = LogosToken::lexer("let my_ptr: ptr");
        assert_eq!(lexer.next().unwrap(), LogosToken::Let);
        assert_eq!(
            lexer.next().unwrap(),
            LogosToken::Identifier(String::from("my_ptr"))
        );
        assert_eq!(lexer.next().unwrap(), LogosToken::Colon);
        assert_eq!(lexer.next().unwrap(), LogosToken::Pointer);
    }

    #[test]
    fn deref_keyword_test() {
        let mut lexer = LogosToken::lexer("deref my_ptr");
        assert_eq!(lexer.next().unwrap(), LogosToken::Dereference);
        assert_eq!(
            lexer.next().unwrap(),
            LogosToken::Identifier(String::from("my_ptr"))
        );
    }

    #[test]
    fn addrof_keyword_test() {
        let mut lexer = LogosToken::lexer("addrof my_ptr");
        assert_eq!(lexer.next().unwrap(), LogosToken::AddressOf);
        assert_eq!(
            lexer.next().unwrap(),
            LogosToken::Identifier(String::from("my_ptr"))
        );
    }

    #[test]
    fn struct_keyword_test() {
        let mut lexer = LogosToken::lexer("struct MyStruct");
        assert_eq!(lexer.next().unwrap(), LogosToken::Struct);
        assert_eq!(
            lexer.next().unwrap(),
            LogosToken::Identifier(String::from("MyStruct"))
        );
    }

    #[test]
    fn dot_keyword_test() {
        let mut lexer = LogosToken::lexer("obj.member");
        assert_eq!(
            lexer.next().unwrap(),
            LogosToken::Identifier(String::from("obj"))
        );
        assert_eq!(lexer.next().unwrap(), LogosToken::Dot);
        assert_eq!(
            lexer.next().unwrap(),
            LogosToken::Identifier(String::from("member"))
        );
    }

    #[test]
    fn import_module_test() {
        let mut lexer = LogosToken::lexer("import \"module\";");
        assert_eq!(lexer.next().unwrap(), LogosToken::Import);
        let str_literal = lexer.next().unwrap();
        assert_eq!(str_literal, LogosToken::Quote(String::new()));

        assert!(if let LogosToken::Quote(content) = str_literal {
            content == "module"
        } else {
            false
        });

        assert_eq!(lexer.next().unwrap(), LogosToken::Semicolon);
    }

    #[test]
    fn double_colon_test() {
        let mut lexer = LogosToken::lexer("module::function()");

        assert!(if let LogosToken::Identifier(id) = lexer.next().unwrap() {
            id == "module"
        } else {
            false
        });
        assert_eq!(lexer.next().unwrap(), LogosToken::DoubleColon);
        assert!(if let LogosToken::Identifier(id) = lexer.next().unwrap() {
            id == "function"
        } else {
            false
        });
        assert_eq!(lexer.next().unwrap(), LogosToken::LeftParenthesis);
        assert_eq!(lexer.next().unwrap(), LogosToken::RightParenthesis);
    }

    #[test]
    fn char_type_test() {
        let mut lexer = LogosToken::lexer("var: char");

        assert!(if let LogosToken::Identifier(id) = lexer.next().unwrap() {
            id == "var"
        } else {
            false
        });
        assert_eq!(lexer.next().unwrap(), LogosToken::Colon);
        assert!(
            if let LogosToken::Type(ValueType::Char) = lexer.next().unwrap() {
                true
            } else {
                false
            }
        );
    }

    #[test]
    fn char_literal_test() {
        let mut lexer = LogosToken::lexer("'a'");

        let tk = lexer.next().unwrap();
        assert!(
            if let LogosToken::CharLiteral(id) = tk {
                id == 'a'
            } else {
                false
            },
            "Got {:?}",
            tk
        );
    }

    #[test]
    fn unclosed_char_literal_test() {
        let mut lexer = LogosToken::lexer("'a");

        let tk = lexer.next().unwrap();
        assert_eq!(tk, LogosToken::Error);
    }

    #[test]
    fn too_much_char_literal_test() {
        let mut lexer = LogosToken::lexer("'abc'");

        let tk = lexer.next().unwrap();
        assert_eq!(tk, LogosToken::Error);
    }
}
