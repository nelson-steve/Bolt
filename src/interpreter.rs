use crate::expr::{Expr, LiteralValue};
use crate::stmt::Stmt;
use crate::environment::{Environment, self};

pub struct Interpreter {
    environement: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { environement: Environment::new(), }
    }

    pub fn interpret_expr(&mut self, expr: Expr) -> Result<LiteralValue, String> {
        expr.evaluate(&self.environement)
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts{
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(&self.environement)?;
                }
                Stmt::Print { expression } => {
                    let value = expression.evaluate(&self.environement)?;
                    println!("{value:?}");
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(&self.environement)?;
                    self.environement.define(name.lexeme, value);
                }
            };
        }
            
        Ok(())
    }
}
