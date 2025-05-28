use crate::{expr::Expr, token::Token};
#[derive(Clone,Debug)]
pub enum Stmt {
    Expression {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        initializer: Expr,
    },
    Block {
        stmts: Vec<Stmt>,
    },
    IfElse {
        condition: Expr,
        then: Box<Stmt>,
        els: Option<Box<Stmt>>,
    },
    WHILE {
        condition: Expr,
        block: Box<Stmt>,
    },
    Break,
    Continue,
}
