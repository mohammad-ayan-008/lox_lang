use std::{env, result, usize, vec};

use crate::{
    expr::{self, Expr, LiteralValue},
    stmt::Stmt,
    token::{self, Literal, Token},
    tokentype::TokenType,
};
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    is_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            is_error: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmt = vec![];

        while !self.is_at_end() && !self.is_error {
            match self.declaration() {
                Ok(s) => stmt.push(s),
        Err(e) => {
                    eprintln!("{}", e);
                    self.is_error = true;
                }
            }
        }
        Ok(stmt)
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_tokens(&[TokenType::VAR]) {
            match self.var_declaration() {
                Ok(s) => Ok(s),
                Err(e) => {
                    self.syncronize();
                    Err(e)
                }
            }
        } else {
            self.statement()
        }
    }
    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let token = self.consume(TokenType::IDENTIFIER, "Expected variable name")?;

        let init;
        if self.match_tokens(&[TokenType::EQUAL]) {
            init = self.expression();
        } else {
            init = Ok(Expr::Literal {
                value: LiteralValue::Nil,
            });
        }
        self.consume(
            TokenType::SEMICOLON,
            "Expected ';' after variable declaration",
        )?;
        Ok(Stmt::Var {
            name: token,
            initializer: init?,
        })
    }
    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_tokens(&[TokenType::PRINT]) {
            self.print_stmt()
        }
        //else if self.match_tokens(&[TokenType::CONTINUE]) {
         //   self.continue_statement()
        //} 
        else if self.match_tokens(&[TokenType::BREAK]){
            self.break_stmt()
        }else if self.match_tokens(&[TokenType::IF]) {
            self.if_statement()
        }else if self.match_tokens(&[TokenType::FOR]) {
           self.for_statement()
        }else if self.match_tokens(&[TokenType::LEFT_BRACE]) {
            self.block()
        } else if self.match_tokens(&[TokenType::WHILE]) {
            self.while_stmt()
        } else {
            self.expression_stmt()
        }
    }
    //fn continue_statement(&mut self)->Result<Stmt,String>{
      //  self.consume(TokenType::SEMICOLON, "Expected ; after break")?;
      //  Ok(Stmt::Continue)
    //}



    fn break_stmt(&mut self)->Result<Stmt,String>{
        self.consume(TokenType::SEMICOLON, "Expected ; after break")?;
        Ok(Stmt::Break)
    }

    fn for_statement(&mut self)-> Result<Stmt,String>{
        // expr statemet | var var_declaration
        self.consume(TokenType::LEFT_PAREN,"Expected '('  after for")?;
        let statement_declaration =
        if  self.match_tokens(&[TokenType::SEMICOLON]){
            None
        }else if self.match_tokens(&[TokenType::VAR]){
            Some(self.var_declaration()?)
        }else{
            Some(self.expression_stmt()?)
        };

        let mut condition = if self.check(&TokenType::SEMICOLON){
            None
        }else{
            Some(self.expression()?)
        };

        self.consume(TokenType::SEMICOLON, "Expected ';' after loop condition")?;
        let increment= if self.check(&TokenType::RIGHT_PAREN){
                 None
        }else{

                Some(self.expression()?)
        };

        let mut body;
        self.consume(TokenType::RIGHT_PAREN, "expected ')' after clause")?;
        self.consume(TokenType::LEFT_BRACE, "expected '{' after for clause")?;
        let  Stmt::Block {mut stmts }= self.block()? else{
            return Err("Expeccted a block".to_string());
        };
        if !increment.is_none() {
            stmts.push(Stmt::Block { 
                         stmts: vec![
                            Stmt::Expression { expression:increment.unwrap()}
                         ]
                    }
            );
        }
        if condition.is_none(){
            condition = Some(Expr::Literal { value: LiteralValue::True })
        }
        body = Stmt::WHILE { condition: condition.unwrap(), block: Box::new(Stmt::Block {  stmts }) }; 
        if let Some(init)=statement_declaration{

            Ok(Stmt::Block { stmts: vec![init,body] })
        }else {
          Ok(body)
        }
    }

    fn while_stmt(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LEFT_PAREN, "( Expected after while")?;
        let expr = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, ") Expected after statement")?;
        self.consume(TokenType::LEFT_BRACE, "{ Expected before start of block")?;
        let block = self.block()?;
        Ok(Stmt::WHILE {
            condition: expr,
            block: Box::new(block),
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LEFT_PAREN, "Expected '(' after if")?;
        let expression = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expected ')' after expression")?;
        self.consume(TokenType::LEFT_BRACE, "Expected { before start of block ")?;
        let block = Box::new(self.block()?);

        let els_stmt = if self.match_tokens(&[TokenType::ELSE]) {
            self.consume(TokenType::LEFT_BRACE, "Expected { before start of block ")?;
            Some(Box::new(self.block()?))
        } else {
            None
        };
        Ok(Stmt::IfElse {
            condition: expression,
            then: block,
            els: els_stmt,
        })
    }

    fn block(&mut self) -> Result<Stmt, String> {
        let mut statements = vec![];
        while !self.check(&TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RIGHT_BRACE, "Expected '}' after the block")?;
        Ok(Stmt::Block { stmts: statements })
    }

    fn print_stmt(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LEFT_PAREN, "Expected '(' before value")?;
        let expr = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expected ')' after value")?;
        self.consume(TokenType::SEMICOLON, "Expected after value';' ")?;

        Ok(Stmt::Print { expression: expr })
    }

    fn expression_stmt(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expected  ';' after expression")?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.or()?;
        if self.match_tokens(&[TokenType::EQUAL]) {
            let value = self.assignment()?;
            match expr {
                Expr::Variable { name } => Ok(Expr::Assign {
                    name,
                    value: Box::from(value),
                }),
                _ => Err("Invalid assignment target".to_string()),
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;
        while self.match_tokens(&[TokenType::OR]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical {
                expression: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;
        while self.match_tokens(&[TokenType::AND]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical {
                expression: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparision()?;
        while self.match_tokens(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparision()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right),
            }
        }
        Ok(expr)
    }
    fn syncronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }
            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => (),
            }
            self.advance();
        }
    }
    fn comparision(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_tokens(&[
            TokenType::GREATER_EQUAL,
            TokenType::GREATER,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right),
            }
        }
        Ok(expr)
    }
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right),
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::STAR, TokenType::SLASH]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(right),
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if (self.match_tokens(&[TokenType::BANG, TokenType::MINUS])) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::from(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek();
        let result;
        match token.token_type {
            TokenType::LEFT_PAREN => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RIGHT_PAREN, "Expected ')'")?;
                result = Expr::Grouping {
                    expression: Box::from(expr),
                }
            }
            TokenType::FALSE
            | TokenType::TRUE
            | TokenType::NIL
            | TokenType::NUMBER
            | TokenType::STRING => {
                self.advance();
                result = Expr::Literal {
                    value: LiteralValue::from_token(token),
                }
            }
            TokenType::IDENTIFIER => {
                self.advance();
                result = Expr::Variable {
                    name: self.previous(),
                }
            }
            _ => {
                let token = self.peek();
                return Err(format!(
                    "[line {}]  Expected expression found {}",
                    token.line, token.lexeme
                ));
            }
        }

        Ok(result)
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            let token = self.previous();
            Ok(token)
        } else {
            return Err(format!("{} at line {}", msg.to_string(), token.line));
        }
    }

    fn match_tokens(&mut self, token_type: &[TokenType]) -> bool {
        for i in token_type {
            if self.check(i) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, tk_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            //TokenType impl Clone so no need of deref
            self.peek().token_type == *tk_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }
    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }
    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1 as usize).unwrap().clone()
    }
}
/*
#[cfg(test)]
mod tests {
    use crate::scanner::{self, Scanner};

    use super::*;

    #[test]
    fn test_addition() {
        let one = Token {
            token_type: TokenType::NUMBER,
            lexeme: "1".to_string(),
            literal: Some(Literal::FLiteral(1.0)),
            line: 0,
        };

        let plus = Token {
            token_type: TokenType::PLUS,
            lexeme: "+".to_string(),
            literal: None,
            line: 0,
        };
        let two = Token {
            token_type: TokenType::NUMBER,
            lexeme: "2".to_string(),
            literal: Some(Literal::FLiteral(2.0)),
            line: 0,
        };
        let semicoln = Token {
            token_type: TokenType::SEMICOLON,
            lexeme: ";".to_string(),
            literal: None,
            line: 0,
        };
        let tokens = vec![one, plus, two, semicoln];

        let mut parser = Parser::new(tokens);
        let parsed = parser.parse();
        let pstr = parsed.unwrap().to_string();
        println!(": {}", pstr);
        assert_eq!(pstr, "(+ 1 2)");
    }

    #[test]
    fn comparision() {
        let source = "1 + 2 == 5 + 7".to_string();
        let scan = Scanner::new(source);
        let token = scan.scanTokens();
        let mut parser = Parser::new(token);
        let parsed_eq = parser.parse().unwrap().to_string();
        assert_eq!(parsed_eq, "(== (+ 1 2) (+ 5 7))");
    }
    #[test]
    fn comparision_paren() {
        let source = "1 == (2 + 2)".to_string();
        let scan = Scanner::new(source);
        let token = scan.scanTokens();
        let mut parser = Parser::new(token);
        let parsed_eq = parser.parse().unwrap().to_string();
        println!("{}", parsed_eq);
        assert_eq!(parsed_eq, "(== 1 (group (+ 2 2)))");
    }
}*/
