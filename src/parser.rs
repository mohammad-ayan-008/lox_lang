use std::{result, usize};

use crate::{expr::{self, Expr, LiteralValue}, token::{self, Literal, Token}, tokentype::TokenType};
pub struct Parser{
    tokens:Vec<Token>,
    current:usize
}

impl Parser{

    pub fn new(tokens:Vec<Token>)->Self{
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self)->Result<Expr,String>{
        self.expression()
    }
    fn expression(&mut self)->Result<Expr,String>{
        self.equality()
    }

     fn equality(&mut self)->Result<Expr,String>{
        let mut expr = self.comparision()?;
        while (self.match_tokens(&[TokenType::BANG_EQUAL,TokenType::EQUAL_EQUAL])){
            let operator =self.previous();
            let right = self.comparision()?;
            expr = Expr::Binary { 
                left: Box::from(expr)
                ,operator
                , right: Box::from(right) 
            }
        }
        Ok(expr)
    }
    fn syncronize(&mut self){
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON{
                return;
            }
            match self.peek().token_type {
                TokenType::CLASS | TokenType::FUN | TokenType::VAR | TokenType::FOR | TokenType::IF | TokenType::WHILE | TokenType::PRINT | TokenType::RETURN => return,
                _=> (),
            }
           self.advance();
        }
    }
    fn comparision(&mut self)-> Result<Expr,String>{
        let mut expr = self.term()?;

        while self.match_tokens(&[ TokenType::GREATER_EQUAL,TokenType::GREATER,TokenType::LESS,TokenType::LESS_EQUAL]){
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary { left: Box::from(expr), operator, right: Box::from(right) }
        }
        Ok(expr)
    }
   fn term(&mut self)->Result<Expr,String>{
        let mut expr = self.factor()?;

        while self.match_tokens(&[ TokenType::MINUS,TokenType::PLUS]){
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary { left: Box::from(expr), operator, right: Box::from(right) }
        }
        Ok(expr)
    }

       fn factor(&mut self)->Result<Expr,String>{
        let mut expr = self.unary()?;

        while self.match_tokens(&[ TokenType::STAR,TokenType::SLASH]){
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary { left: Box::from(expr),  operator, right: Box::from(right) }
        }
        Ok(expr)
    }


   fn unary(&mut self)->Result<Expr,String>{
        if (self.match_tokens(&[TokenType::BANG,TokenType::MINUS])){
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary {  operator, right: Box::from(right) })
        }else {
            self.primary()
        }
   }

    fn primary(&mut self)->Result<Expr,String>{
        let token = self.peek();
        let result;
        match token.token_type {
            TokenType::LEFT_PAREN=>{
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RIGHT_PAREN, "Expected ')'")?;
                result = Expr::Grouping { expression: Box::from(expr) }
            }
            TokenType::FALSE | TokenType::TRUE | TokenType::NIL | TokenType::NUMBER | TokenType::STRING=>{
                self.advance();
                result = Expr::Literal{
                    value: LiteralValue::from_token(token),
                }
            }
            _=> return  Err("Expected Expression ".to_string()),
        }

        Ok(result)
    }

    fn consume(&mut self,token_type:TokenType,msg:&str)->Result<(),String>{
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
        }else {
            return Err(msg.to_string());
        }
        Ok(())
    }




    fn match_tokens(&mut self,token_type:&[TokenType])->bool{
        for i in token_type{
            if self.check(i) {
                self.advance();
                return true;
            }
        } 
        false
    }

    fn check(&self,tk_type:&TokenType)->bool{
        if self.is_at_end(){
            false
        }else {
            //TokenType impl Clone so no need of deref
            self.peek().token_type == *tk_type
        }
    }

    fn advance(&mut self)->Token{
        if !self.is_at_end(){
            self.current +=1;
        }
        self.previous()
    }

    fn is_at_end(&self)->bool{
        self.peek().token_type== TokenType::EOF
    }
    fn peek(&self)->Token{
        self.tokens.get(self.current).unwrap().clone()
    }
    fn previous(&self)->Token{
        self.tokens.get(self.current -1 as usize).unwrap().clone()
    }
   
}



#[cfg(test)]
mod tests{
    use crate::scanner::{self, Scanner};

    use super::*;

    #[test]
    fn test_addition(){
        let one = Token{
            token_type:TokenType::NUMBER,
            lexeme:"1".to_string(),
            literal:Some(Literal::FLiteral(1.0)),
            line:0
        };

        let plus= Token{
            token_type:TokenType::PLUS,
            lexeme:"+".to_string(),
            literal:None,
            line:0
        };
        let two = Token{
            token_type:TokenType::NUMBER,
            lexeme:"2".to_string(),
            literal:Some(Literal::FLiteral(2.0)),
            line:0
        };
        let semicoln= Token{
            token_type:TokenType::SEMICOLON,
            lexeme:";".to_string(),
            literal:None,
            line:0
        };
        let tokens = vec![one,plus,two,semicoln];

        let mut  parser = Parser::new(tokens);
        let parsed = parser.parse();
        let pstr = parsed.unwrap().to_string();
        println!(": {}",pstr);
        assert_eq!(pstr,"(+ 1 2)");
    }

    #[test]
    fn comparision(){
        let source = "1 + 2 == 5 + 7".to_string();
        let scan = Scanner::new(source);
        let token = scan.scanTokens();
        let mut parser = Parser::new(token);
        let parsed_eq = parser.parse().unwrap().to_string();
        assert_eq!(parsed_eq,"(== (+ 1 2) (+ 5 7))");
    }
     #[test]
    fn comparision_paren(){
        let source = "1 == (2 + 2)".to_string();
        let scan = Scanner::new(source);
        let token = scan.scanTokens();
        let mut parser = Parser::new(token);
        let parsed_eq = parser.parse().unwrap().to_string();
        println!("{}",parsed_eq);
        assert_eq!(parsed_eq,"(== 1 (group (+ 2 2)))");
    }
}
