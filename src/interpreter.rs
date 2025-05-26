use std::rc::Rc;

use crate::{
    environment::{self, Environment},
    expr::{self, Expr, LiteralValue},
    stmt::{self, Stmt},
};

pub struct Interpreter {
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(Environment::new()),
        }
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<LiteralValue, String> {
        expr.eval(Rc::get_mut(&mut self.environment).expect("cannot get mut ref"))
    }

    pub fn interpret_stmt(&mut self, st: Vec<Stmt>) -> Result<(), String> {
        for i in st {
            match i {
                Stmt::Block { stmts } => {
                    let mut new_env = Environment::new();
                    new_env.enclosing = Some(self.environment.clone());

                    let old_env = self.environment.clone();
                    self.environment = Rc::new(new_env);
                    self.interpret_stmt(stmts)?;
                    self.environment = old_env;
                }
                Stmt::Expression { expression } => {
                    expression
                        .eval(Rc::get_mut(&mut self.environment).expect("cannot get mut reff"))?;
                }
                Stmt::Print { expression } => {
                    let value = expression.eval(
                        Rc::get_mut(&mut self.environment).expect("cannot get mutable reff"),
                    )?;
                    println!("{}", value.to_string());
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer
                        .eval(Rc::get_mut(&mut self.environment).expect("cannot get mut ref"))?;
                    Rc::get_mut(&mut self.environment)
                        .expect("cannot get mutable reff")
                        .define(name.lexeme, value);
                }
            };
        }
        Ok(())
    }
}
