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
    Function{
        name:Token,
        params: Vec<Token>,
        body: Vec<Stmt>
    },
    Return{
        token:Token,
        expr:Option<Expr>
    },
    Break,
    Continue,
}
