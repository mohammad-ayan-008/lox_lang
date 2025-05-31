use std::fmt::format;

use crate::tokentype::TokenType;

#[derive(Debug, Clone)]
pub enum Literal {
    StringLiteral(String),
    FLiteral(f64),
    ILiteral(i64),
    IdentifierLiteral(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!(
            " {:?}  {}  {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            token_type: t_type,
            lexeme,
            literal,
            line,
        }
    }
}
