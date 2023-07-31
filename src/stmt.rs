use crate::expr::Expr;
use crate::scanner::Token;

pub enum Stmt{
    Expression {expression: Expr},
    Print {expression: Expr},
    Var {name: Token, initializer: Expr},
}

impl Stmt {
    pub fn to_string(&self) -> String {
        // use Stmt::*;
        match self {
            Self::Expression { expression } => expression.to_string(),
            Self::Print { expression } => format!("(print {})", expression.to_string()),
            Self::Var { name, initializer } => format!("(var {})", name.to_string()),
        }
    }
}