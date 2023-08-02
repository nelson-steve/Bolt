use crate::expr::Expr;
use crate::scanner::Token;

pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
    Block { statements: Vec<Stmt> },
    IfStmt {
        predicate: Expr,
        then: Box<Stmt>,
        els: Option<Box<Stmt>>,
    },
    WhileStmt {
        condition: Expr,
        body: Box<Stmt>,
    }
}

impl Stmt {
    pub fn to_string(&self) -> String {
        // use Stmt::*;
        match self {
            Stmt::Expression { expression } => expression.to_string(),
            Stmt::Print { expression } => format!("(print {})", expression.to_string()),
            Stmt::Var { name, initializer: _ } => format!("(var {})", name.to_string()),
            Stmt::Block { statements } => {
                format!(
                    "(block {})",
                    statements
                        .into_iter()
                        .map(|stmt| stmt.to_string())
                        .collect::<String>()
                )
            }
            Stmt::IfStmt { predicate: _, then: _, els: _ } => todo!(),
            Stmt::WhileStmt { condition: _, body: _ } => todo!(),
        }
    }
}
