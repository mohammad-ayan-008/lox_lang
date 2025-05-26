use crate::{expr::Expr, token::Token};

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
}
