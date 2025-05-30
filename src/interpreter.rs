use std::{cell::RefCell, env, error::Error, ops::ControlFlow, rc::Rc};

use crate::{
    environment::{self, Environment},
    expr::{self, Expr, LiteralValue},
    stmt::{self, Stmt},
};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}
#[derive(PartialEq)]
pub enum ControllFlow {
    None,
    Break,
    Continue,
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
    pub fn interpret_stmt(&mut self, st: &[Stmt]) -> Result<ControllFlow, String> {
        for i in st {
            match i {
                Stmt::Break => return Ok(ControllFlow::Break),
                Stmt::Continue => return Ok(ControllFlow::Continue),
                Stmt::WHILE { condition, block } => match **block {
                    Stmt::Block { ref stmts } => {
                        let mut new_env = Environment::new();
                        new_env.enclosing = Some(self.environment.clone());
                        let old_env = self.environment.clone();
                        self.environment = Rc::new(new_env.into());
                        'nox_loop: while condition.eval(self.environment.clone())?.is_truthy() {
                            match self.interpret_stmt(&stmts)?{
                               ControllFlow::Break=>break 'nox_loop,
                               ControllFlow::Continue=> {
                                    let a = stmts.last().unwrap();
                                    self.interpret_stmt(&[a.clone()]);
                                    continue 'nox_loop
                                },
                               ControllFlow::None=>(),
                            }
                        }
                        self.environment = old_env;
                    }
                    _ => return Err("Invalid expr".to_string()),
                },
                Stmt::IfElse {
                    condition,
                    then,
                    els,
                } => {
                    let a = condition.eval(self.environment.clone())?;
                    if a == LiteralValue::True {
                        match **then {
                            Stmt::Block { ref stmts } => {
                                let mut new_env = Environment::new();
                                new_env.enclosing = Some(self.environment.clone());
                                let old_env = self.environment.clone();
                                self.environment = Rc::new(new_env.into());
                                let cf =self.interpret_stmt(&stmts)?;
                                self.environment = old_env;
                                if cf != ControllFlow::None {
                                   return Ok(cf);
                                }
                            }
                            _ => {
                                return Err("invalid Expr".to_string());
                            }
                        }
                    } else {
                        if let Some(a) = els {
                            if let Stmt::Block { ref stmts } = **a {
                                let mut new_env = Environment::new();
                                new_env.enclosing = Some(self.environment.clone());

                                let old_env = self.environment.clone();
                                self.environment = Rc::new(new_env.into());
                                let cf = self.interpret_stmt(&stmts)?;
                                self.environment = old_env;
                                if cf != ControllFlow::None{
                                    return Ok(cf);
                                }
                            }
                        }
                    }
                }
                Stmt::Block { stmts } => {
                    let mut new_env = Environment::new();
                    new_env.enclosing = Some(self.environment.clone());

                    let old_env = self.environment.clone();
                    self.environment = Rc::new(new_env.into());
                    let cf =self.interpret_stmt(stmts)?;
                    self.environment = old_env;
                    if cf != ControllFlow::None {
                         return Ok(cf);
                    }
                }
                Stmt::Expression { expression } => {
                    expression.eval(self.environment.clone())?;
                }
                Stmt::Print { expression } => {
                    let value = expression.eval(self.environment.clone())?;
                    println!("{}", value.to_string());
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer.eval(self.environment.clone())?;

                    self.environment.borrow_mut().define(&name.lexeme, value);
                }
            };
        }
        Ok(ControllFlow::None)
    }
}
