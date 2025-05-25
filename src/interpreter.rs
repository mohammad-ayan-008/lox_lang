use crate::{
    expr::{self, Expr, LiteralValue},
    stmt::{self, Stmt},
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<LiteralValue, String> {
        expr.eval()
    }

    pub fn interpret_stmt(&mut self, st: Vec<Stmt>) -> Result<(), String> {
        for i in st {
            match i {
                Stmt::Expression { expression } => {
                    expression.eval()?;
                }
                Stmt::Print { expression } => {
                    let value = expression.eval()?;
                    println!("{}",value.to_string());
                }
            };
        }
        Ok(())
    }
}
