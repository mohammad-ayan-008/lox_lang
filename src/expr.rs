use crate::token::{Literal, Token};

pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil
}
pub enum Expr{
    Binary{
        left:Box<Expr>,
        operator:Token,
        right:Box<Expr>
    },
    Grouping{
        expression:Box<Expr>
    },
    Literal{
        value:LiteralValue
    },
    Unary{
        operator:Token,
        right:Box<Expr>
    }

}
impl ToString for LiteralValue{
    fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x)=>x.to_string(),
            LiteralValue::StringValue(x)=>x.clone(),
            LiteralValue::True=>"true".to_string(),
            LiteralValue::False=>"false".to_string(),
            LiteralValue::Nil=>"nil".to_string()
        }
    }
}
fn unwrap_as_string(literal:Option<Literal>)->String{
    match literal {
        Some(Literal::StringLiteral(x))=> x.clone(),
        Some(Literal::IdentifierLiteral(s))=>s.clone(),
        _=> panic!("could not unwrap"),
    }
}
fn unwrap_as_f32(literal:Option<Literal>)->f32{
    match literal {
        Some(Literal::FLiteral(x))=> x as f32,
        _=> panic!("could not unwrap"),
    }
}

impl LiteralValue{
     pub fn from_token(token:Token)-> Self{
        match token.token_type {
            crate::tokentype::TokenType::NUMBER=> Self::Number(unwrap_as_f32(token.literal)),
            crate::tokentype::TokenType::STRING=> Self::StringValue(unwrap_as_string(token.literal)),
            crate::tokentype::TokenType::FALSE=>Self::False,
            crate::tokentype::TokenType::TRUE=>Self::True,
            crate::tokentype::TokenType::NIL=>Self::Nil,
            _=>panic!("could not create literal value from {:?}",token)

        }
    }
}

impl ToString for Expr {
    fn to_string(&self)->String{
        match self {
           Expr::Binary { left, operator, right }=>{
                format!("({} {} {})",operator.lexeme,left.to_string(),right.to_string())
            },
           Expr::Unary { operator, right }=>{
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})",operator_str,right_str)
            },
           Expr::Literal { value }=>{
                format!("{}",value.to_string())
            },
           Expr::Grouping { expression }=>{
                format!("(group {})",(*expression).to_string())
            }  
        }
    }
}
impl Expr{
     pub fn print(&self){
        println!("=> {}",self.to_string());
    }
   
}

#[cfg(test)]
mod tests{
    use core::num;
    use std::fmt::Binary;

    use crate::token::Literal;

    use super::*;

    #[test]
    pub fn test(){
        let minus_token = Token{
            token_type:crate::tokentype::TokenType::MINUS,
            lexeme:"-".to_string(),
            literal:None,
            line:0
        };
        let nums = Expr::Literal { 
            value: LiteralValue::Number(123.0) 
        };
        let group = Expr::Grouping { expression: 
            Box::from(
               Expr::Literal{
                    value:LiteralValue::Number(45.67),
               }
            ) 
        };

        let multi = Token{
            token_type:crate::tokentype::TokenType::STAR,
            lexeme:"*".to_string(),
            literal:None,
            line:0
        };
        let ast = Expr::Binary{
            left: Box::from(
                Expr::Unary{
                  operator: minus_token,
                   right: Box::from(nums)
                }
            ),
            operator:multi,
            right: Box::from(group)
        };

        let result = ast.to_string();
        assert_eq!(result,"(* (- 123) (group 45.67))")
    }
}
