use crate::{
    lexer::LogosToken,
    type_system::value_type::{StaticArray, ValueType},
};

use super::parser::Parser;

impl Parser {
    pub fn parse_type(&mut self) -> Result<ValueType, ()> {
        if self.match_expr(&[LogosToken::LeftBracket]) {
            let array_type = self.parse_type()?;
            if let None = self.consume(&LogosToken::Semicolon, "Expected ';' after array type.") {
                return Err(());
            }

            let size: usize = if let Some(LogosToken::Number(n)) =
                self.consume(&LogosToken::Number(0), "Expected constant number size")
            {
                *n as usize
            } else {
                return Err(());
            };

            if let None = self.consume(&LogosToken::RightBracket, "Unclosed ']' after array size.")
            {
                return Err(());
            }

            Ok(ValueType::Array(StaticArray {
                size,
                array_type: Box::new(array_type),
            }))
        } else {
            match self.advance() {
                Some(LogosToken::Type(t)) => Ok(t.clone()),
                Some(LogosToken::Pointer) => {
                    let inner_type = self.parse_type()?;
                    Ok(ValueType::Pointer(Box::new(inner_type)))
                }
                Some(LogosToken::Identifier(name)) => Ok(ValueType::Struct(name.to_string())),
                _ => {
                    println!("Expected type hints.");
                    Err(())
                }
            }
        }
    }
}
