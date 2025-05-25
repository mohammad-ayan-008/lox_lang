use core::panic;
use std::{any::type_name, collections::HashMap, string, usize};

use crate::{
    token::{Literal, Token},
    tokentype::TokenType,
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: Self::init_keywords(),
        }
    }

    pub fn scanTokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scanToken()
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "".to_owned(), None, self.line));
        self.tokens
    }

    fn scanToken(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.token_add(TokenType::LEFT_PAREN),
            ')' => self.token_add(TokenType::RIGHT_PAREN),
            '{' => self.token_add(TokenType::LEFT_BRACE),
            '}' => self.token_add(TokenType::RIGHT_BRACE),
            ',' => self.token_add(TokenType::COMMA),
            '.' => self.token_add(TokenType::DOT),
            '-' => self.token_add(TokenType::MINUS),
            '+' => self.token_add(TokenType::PLUS),
            ';' => self.token_add(TokenType::SEMICOLON),
            '*' => self.token_add(TokenType::STAR),
            '!' => {
                let token = match self.match_token('=') {
                    true => TokenType::BANG_EQUAL,
                    false => TokenType::BANG,
                };
                self.token_add(token);
            }
            '=' => {
                let token = match self.match_token('=') {
                    true => TokenType::EQUAL_EQUAL,
                    false => TokenType::EQUAL,
                };
                self.token_add(token);
            }
            '>' => {
                let token = match self.match_token('=') {
                    true => TokenType::GREATER_EQUAL,
                    false => TokenType::GREATER,
                };
                self.token_add(token);
            }
            '<' => {
                let token = match self.match_token('<') {
                    true => TokenType::LESS_EQUAL,
                    false => TokenType::LESS,
                };
                self.token_add(token);
            }
            '/' => {
                match self.match_token('/') {
                    true => {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }
                    false => self.token_add(TokenType::SLASH),
                };
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            c if Self::is_digit(c) => self.number(),
            c if Self::is_alpha(c) => self.identifier(),
            e => {
                panic!("Uknown Symbol {}", e);
            }
        }
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let key = &self.source[self.start..self.current];

        let mut t = self.keywords.get(key).cloned();
        if let None = t {
            t = Some(TokenType::IDENTIFIER);
        }
        self.token_add(t.unwrap());
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();
            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.add_token(TokenType::NUMBER, Some(Literal::FLiteral(value)));
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            println!("{} Unterminated String", self.line);
            return;
        }
        self.advance();
        let string = self.source[self.start + 1..self.current - 1].to_owned();
        self.add_token(TokenType::STRING, Some(Literal::StringLiteral(string)));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }
    fn advance(&mut self) -> char {
        let char = self.source.as_bytes()[self.current as usize];
        self.current += 1;
        char as char
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source
                .chars()
                .nth((self.current + 1) as usize)
                .unwrap()
        }
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.as_bytes()[self.current] as char != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn token_add(&mut self, type_token: TokenType) {
        self.add_token(type_token, None);
    }

    fn add_token(&mut self, type_token: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start as usize..self.current as usize].to_string();
        self.tokens
            .push(Token::new(type_token, text, literal, self.line));
    }
    fn init_keywords() -> HashMap<&'static str, TokenType> {
        let mut keywords = HashMap::new();

        keywords.insert("and", TokenType::AND);
        keywords.insert("class", TokenType::CLASS);
        keywords.insert("else", TokenType::ELSE);
        keywords.insert("false", TokenType::FALSE);
        keywords.insert("for", TokenType::FOR);
        keywords.insert("fun", TokenType::FUN);
        keywords.insert("if", TokenType::IF);
        keywords.insert("nil", TokenType::NIL);
        keywords.insert("or", TokenType::OR);
        keywords.insert("print", TokenType::PRINT);
        keywords.insert("return", TokenType::RETURN);
        keywords.insert("super", TokenType::SUPER);
        keywords.insert("this", TokenType::THIS);
        keywords.insert("true", TokenType::TRUE);
        keywords.insert("var", TokenType::VAR);
        keywords.insert("while", TokenType::WHILE);

        keywords
    }

    fn is_alpha(c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }
}
