use std::{cell::RefCell, env, error::Error, rc::Rc};

use crate::{
    environment::{self, Environment},
    expr::{self, Expr, LiteralValue},
    stmt::{self, Stmt},
};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<LiteralValue, String> {
        expr.eval(self.environment.clone())
    }
    #[allow(warnings)]
    pub fn interpret_stmt(&mut self, st: Vec<Stmt>) -> Result<(), String> {
        for i in st {
            match i {
                Stmt::IfElse {
                    condition,
                    then,
                    els,
                } => {
                    let a = condition
                        .eval(self.environment.clone())?;
                    if a == LiteralValue::True {
                        match *then {
                            Stmt::Block { stmts } => {
                                let mut new_env = Environment::new();
                                new_env.enclosing = Some(self.environment.clone());

                                let old_env = self.environment.clone();
                                self.environment = Rc::new(new_env.into());
                                self.interpret_stmt(stmts)?;
                                self.environment = old_env;
                            }
                            _ => {
                                return Err("invalid Expr".to_string());
                            }
                        }
                    } else {
                        if let Some(a) = els {
                            if let Stmt::Block { stmts } = *a {
                                let mut new_env = Environment::new();
                                new_env.enclosing = Some(self.environment.clone());

                                let old_env = self.environment.clone();
                                self.environment = Rc::new(new_env.into());
                                self.interpret_stmt(stmts)?;
                                self.environment = old_env;
                            }
                        }
                    }
                }
                Stmt::Block { stmts } => {
                    let mut new_env = Environment::new();
                    new_env.enclosing = Some(self.environment.clone());

                    let old_env = self.environment.clone();
                    self.environment = Rc::new(new_env.into());
                    self.interpret_stmt(stmts)?;
                    self.environment = old_env;
                }
                Stmt::Expression { expression } => {
                    expression
                        .eval(self.environment.clone())?;
                }
                Stmt::Print { expression } => {
                    let value = expression
                        .eval(self.environment.clone())?;
                    println!("{}", value.to_string());
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer
                        .eval(self.environment.clone())?;

                    self.environment.borrow_mut()
                        .define(name.lexeme, value);
                }
            };
        }
        Ok(())
    }
}
