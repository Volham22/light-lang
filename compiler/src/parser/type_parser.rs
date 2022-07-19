use crate::{
    lexer::Token,
    type_system::value_type::{StaticArray, ValueType},
};

use super::parser::Parser;

impl Parser {
    pub fn parse_type(&mut self) -> Result<ValueType, ()> {
        if self.match_expr(&[Token::LeftBracket]) {
            let array_type = self.parse_type()?;
            if let None = self.consume(&Token::Semicolon, "Expected ';' after array type.") {
                return Err(());
            }

            let size: usize = if let Some(Token::Number(n)) =
                self.consume(&Token::Number(0), "Expected constant number size")
            {
                *n as usize
            } else {
                return Err(());
            };

            if let None = self.consume(&Token::RightBracket, "Unclosed ']' after array size.") {
                return Err(());
            }

            Ok(ValueType::Array(StaticArray {
                size,
                array_type: Box::new(array_type),
            }))
        } else {
            match self.advance() {
                Some(Token::Type(t)) => Ok(t.clone()),
                Some(Token::Pointer) => {
                    let inner_type = self.parse_type()?;
                    Ok(ValueType::Pointer(Box::new(inner_type)))
                }
                _ => {
                    println!("Expected type hints.");
                    Err(())
                }
            }
        }
    }
}
