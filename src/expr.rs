use std::{cell::RefCell, fmt::{format, write, Debug}, rc::Rc, result, usize};

use crate::{
    environment::{self, Environment},
    token::{Literal, Token},
    tokentype::TokenType,
};
#[derive(Clone)]
pub enum LiteralValue {
    Number(f64),
    StringValue(String),
    True,
    False,
    Nil,
    Callable{name:String,arity:usize,fun:Rc<dyn Fn(Rc<RefCell<Environment>>,&Vec<LiteralValue>)->LiteralValue>}
}
impl Debug for LiteralValue{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.to_string())
    }
}





impl PartialEq for LiteralValue{
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (Self::Number(a),Self::Number(b))=>a == b,
            (Self::StringValue(a),Self::StringValue(b))=> a==b,
            (Self::True,Self::True)=>true,
            (Self::False,Self::False)=>true,
            (Self::Callable { name, arity, ..},Self::Callable {
                name:name2, arity:arity2, .. })=>{
             name ==  name2 && arity == arity2
            }
            _=> false
        }
    }
}
impl LiteralValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            LiteralValue::False | LiteralValue::Nil => false,
            _ => true,
        }
    }

    pub fn is_falsy(&self) -> LiteralValue {
        match self {
            Self::Number(x) => {
                if *x == 0.0 {
                    Self::True
                } else {
                    Self::False
                }
            }
            Self::StringValue(s) => {
                if s.len() == 0 {
                    Self::True
                } else {
                    Self::False
                }
            }
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Nil => Self::True,
            Self::Callable { name, arity, fun }=>panic!("cant use Callable as truthly value")

        }
    }
    pub fn from_bool(b: bool) -> Self {
        if b { Self::True } else { Self::False }
    }

    pub fn to_type(&self) -> String {
        match self {
            LiteralValue::Number(_) => "Number".to_string(),
            LiteralValue::StringValue(_) => "String".to_string(),
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Callable { name, arity, fun }=>"Callable".to_string()
        }
    }
}

#[derive(Debug,Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Call{
        callie:Box<Expr>,
        paren: Token,
        args:Vec<Expr>
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Logical {
        expression: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}

#[allow(warnings)]
impl ToString for LiteralValue {
    fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(x) => x.clone(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Callable { name, arity, fun }=>format!("{name}/{arity}")
        }
    }
}
fn unwrap_as_string(literal: Option<Literal>) -> String {
    match literal {
        Some(Literal::StringLiteral(x)) => x.clone(),
        Some(Literal::IdentifierLiteral(s)) => s.clone(),
        _ => panic!("could not unwrap"),
    }
}
fn unwrap_as_f64(literal: Option<Literal>) -> f64 {
    match literal {
        Some(Literal::FLiteral(x)) => x as f64,
        _ => panic!("could not unwrap"),
    }
}

impl LiteralValue {
    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            crate::tokentype::TokenType::NUMBER => Self::Number(unwrap_as_f64(token.literal)),
            crate::tokentype::TokenType::STRING => {
                Self::StringValue(unwrap_as_string(token.literal))
            }
            crate::tokentype::TokenType::FALSE => Self::False,
            crate::tokentype::TokenType::TRUE => Self::True,
            crate::tokentype::TokenType::NIL => Self::Nil,
            _ => panic!("could not create literal value from {:?}", token),
        }
    }
}
#[allow(warnings)]
impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Call { callie, paren, args }=>{
                format!("{:?}",callie)
            }
            Expr::Logical {
                expression,
                operator,
                right,
            } => "".to_string(),
            Expr::Assign { name, value } => {
                format!("{name:?} = {}", value.to_string())
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    left.to_string(),
                    right.to_string()
                )
            }
            Expr::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})", operator_str, right_str)
            }
            Expr::Literal { value } => {
                format!("{}", value.to_string())
            }
            Expr::Grouping { expression } => {
                format!("(group {})", (*expression).to_string())
            }
            Expr::Variable { name } => format!("(var {})", name.lexeme),
        }
    }
}

impl Expr {
    pub fn print(&self) {
        println!("=> {}", self.to_string());
    }
    pub fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LiteralValue, String> {
        match self {
            Expr::Call {
                callie, 
                paren, 
                args }=>{
                let evals = callie.eval(env.clone())?;
                match evals {
                    LiteralValue::Callable { name, arity, fun }=>{
                        if arity != args.len() {
                            return Err("Wrong args".to_string());
                        }
                        let args:Vec<LiteralValue> = args.iter().map(|x| x.eval(env.clone()).unwrap()).collect();
                        Ok(fun(env.clone(),&args))
                    }
                    other=>Err(format!("{:?} type is not callable",other))
                }

            },
            Expr::Logical {
                expression,
                operator,
                right,
            } => {
                let left = { expression.eval(env.clone())? };
                if operator.token_type == TokenType::OR {
                    if left.is_truthy() {
                        return Ok(left);
                    }
                } else {
                    if !left.is_truthy() {
                        return Ok(left);
                    }
                }
                right.eval(env)
            }
            Expr::Assign { name, value } => {
                let new_value = (*value).eval(env.clone())?;
                let assign_success = env.borrow_mut().assign(&name.lexeme, new_value.clone());
                if assign_success {
                    Ok(new_value)
                } else {
                    Err(format!("variable {} not declared", &name.lexeme))
                }
            }
            Expr::Variable { name } => match env.borrow_mut().get(&name.lexeme) {
                Some(v) => Ok(v.clone()),
                None => Err(format!("Variable {} is not declared ", name.lexeme)),
            },
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Grouping { expression } => expression.eval(env),
            Expr::Unary { operator, right } => {
                let right = right.eval(env)?;
                match (&right, operator.token_type) {
                    (LiteralValue::Number(x), TokenType::MINUS) => Ok(LiteralValue::Number(-x)),
                    (_, TokenType::MINUS) => {
                        Err(format!("Minus is not implemented for {}", right.to_type()))
                    }
                    (any, TokenType::BANG) => Ok(any.is_falsy()),
                    (_, ttype) => Err(format!("{:?} is not a valid unary operator", ttype)),
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.eval(env.clone())?;
                let right = right.eval(env)?;

                match (&left, operator.token_type, &right) {
                    (LiteralValue::Number(x), TokenType::Modulus, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x % y))
                    }
                    (LiteralValue::Number(x), TokenType::PLUS, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x + y))
                    }
                    (LiteralValue::Number(x), TokenType::MINUS, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x - y))
                    }
                    (LiteralValue::Number(x), TokenType::SLASH, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x / y))
                    }

                    (LiteralValue::Number(x), TokenType::STAR, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x * y))
                    }

                    (LiteralValue::Number(x), TokenType::GREATER, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::from_bool(x > y))
                    }

                    (
                        LiteralValue::Number(x),
                        TokenType::GREATER_EQUAL,
                        LiteralValue::Number(y),
                    ) => Ok(LiteralValue::from_bool(x >= y)),

                    (LiteralValue::Number(x), TokenType::LESS, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::from_bool(x < y))
                    }

                    (LiteralValue::Number(x), TokenType::LESS_EQUAL, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::from_bool(x <= y))
                    }
                    (LiteralValue::StringValue(x), TokenType::PLUS, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::StringValue(format!("{}{}", x, y)))
                    }

                    (LiteralValue::StringValue(_), _, LiteralValue::Number(_)) => {
                        Err("Cannot operate on String and number".to_string())
                    }

                    (LiteralValue::Number(_), _, LiteralValue::StringValue(_)) => {
                        Err("Cannot operate on String and number".to_string())
                    }

                    (
                        LiteralValue::StringValue(x),
                        TokenType::PLUS,
                        LiteralValue::StringValue(y),
                    ) => Ok(LiteralValue::StringValue(format!("{}{}", x, y))),

                    (x, TokenType::BANG_EQUAL, y) => Ok(LiteralValue::from_bool(x != y)),

                    (x, TokenType::EQUAL_EQUAL, y) => Ok(LiteralValue::from_bool(x == y)),
                    (
                        LiteralValue::StringValue(x),
                        TokenType::GREATER,
                        LiteralValue::StringValue(y),
                    ) => Ok(LiteralValue::from_bool(x > y)),

                    (
                        LiteralValue::StringValue(x),
                        TokenType::GREATER_EQUAL,
                        LiteralValue::StringValue(y),
                    ) => Ok(LiteralValue::from_bool(x >= y)),

                    (
                        LiteralValue::StringValue(x),
                        TokenType::LESS,
                        LiteralValue::StringValue(y),
                    ) => Ok(LiteralValue::from_bool(x < y)),

                    (
                        LiteralValue::StringValue(x),
                        TokenType::LESS_EQUAL,
                        LiteralValue::StringValue(y),
                    ) => Ok(LiteralValue::from_bool(x <= y)),

                    (x, ttype, y) => Err(format!("{:?}  not impl for {:?} and {:?}", ttype, x, y)),
                }
            }

            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::num;
    use std::fmt::Binary;

    use crate::token::Literal;

    use super::*;

    #[test]
    pub fn test() {
        let minus_token = Token {
            token_type: crate::tokentype::TokenType::MINUS,
            lexeme: "-".to_string(),
            literal: None,
            line: 0,
        };
        let nums = Expr::Literal {
            value: LiteralValue::Number(123.0),
        };
        let group = Expr::Grouping {
            expression: Box::from(Expr::Literal {
                value: LiteralValue::Number(45.67),
            }),
        };

        let multi = Token {
            token_type: crate::tokentype::TokenType::STAR,
            lexeme: "*".to_string(),
            literal: None,
            line: 0,
        };
        let ast = Expr::Binary {
            left: Box::from(Expr::Unary {
                operator: minus_token,
                right: Box::from(nums),
            }),
            operator: multi,
            right: Box::from(group),
        };

        let result = ast.to_string();
        assert_eq!(result, "(* (- 123) (group 45.67))")
    }
}
