use std::{cell::RefCell, env, error::Error, ops::ControlFlow, rc::Rc, result, time::{Instant, SystemTime}};

use crate::{
    environment::{self, Environment}, expr::{self, Expr, LiteralValue}, parser, stmt::{self, Stmt}
};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}
#[derive(PartialEq)]
pub enum ControllFlow {
    None,
    Break,
    ReturnVal(LiteralValue),
    Continue,
}
pub fn time_fn(env:Rc<RefCell<Environment>>,args:&Vec<LiteralValue>)->LiteralValue{

    let a = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    LiteralValue::Number(a.as_secs_f64())
}
pub fn floor(env:Rc<RefCell<Environment>>,args:&Vec<LiteralValue>)->LiteralValue{
    let value = &args[0];
    if let LiteralValue::Number(x)= value{

        LiteralValue::Number( x.floor() )
    }else {
        panic!("Expected number");
    }
}
impl Interpreter {
    fn forClosure(parent:Rc<RefCell<Environment>>) -> Self{
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut().enclosing = Some(parent);
        Self { environment: env }
    }
    pub fn new() -> Self {
        let mut global = Environment::new();
        global.define(&"time".to_string(), LiteralValue::Callable { name: "time".to_string(), arity: 0, fun:Rc::new(time_fn) });
 global.define(&"floor".to_string(), LiteralValue::Callable { name: "floor".to_string(), arity: 1, fun:Rc::new(floor) });
        Self {
            environment: Rc::new(RefCell::new(global)),
        }
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<LiteralValue, String> {
        expr.eval(self.environment.clone())
    }
    #[allow(warnings)]
    pub fn interpret_stmt(&mut self, st: &[Stmt]) -> Result<ControllFlow, String> {
        for i in st {
            match i {
                Stmt::Return { token, expr }=>{
                   if let Some(value) = expr{
                        return Ok(ControllFlow::ReturnVal(value.eval(self.environment.clone())?));
                   }else {
                        return Ok(ControllFlow::ReturnVal(LiteralValue::Nil));
                   }
                }
                Stmt::Function { name, params, body }=>{

                    let arity = params.len();
                    let params = params.clone();
                    let body = body.clone();
                    let call = move |parent,args:&Vec<LiteralValue>| {
                        let mut closure_interpreter = Interpreter::forClosure(parent);
                        for (i,arg) in args.iter().enumerate(){
                            closure_interpreter.environment.borrow_mut().define(&params.clone()[i].lexeme, arg.clone());
                        }
;
                        let mut cf =closure_interpreter.interpret_stmt(&body).unwrap();
                        let return_value;
                        if let ControllFlow::ReturnVal(val) = cf{
                            return_value = val; 
                        }else {
                            return_value = LiteralValue::Nil;
                        }
                        return_value
                    };

                    let callable = LiteralValue::Callable { name:name.to_string(), arity, fun: Rc::new(call)};
                    self.environment.borrow_mut().define(&name.lexeme, callable);

                },
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
                               _=>(),
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
