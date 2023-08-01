use crate::environment::{self, Environment};
use crate::expr::{Expr, LiteralValue};
use crate::scanner::TokenType;
use crate::stmt::Stmt;
use std::rc::Rc;
use std::vec;

pub struct Interpreter {
    environement: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environement: Rc::new(Environment::new()),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(
                        Rc::get_mut(&mut self.environement)
                            .expect("Could not get mutable reference to environemnt"),
                    )?;
                }
                Stmt::Print { expression } => {
                    let value = expression.evaluate(
                        Rc::get_mut(&mut self.environement)
                            .expect("Could not get mutable reference to environemnt"),
                    )?;
                    println!("{}", value.to_string());
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(
                        Rc::get_mut(&mut self.environement)
                            .expect("Could not get mutable reference to environemnt"),
                    )?;
                    Rc::get_mut(&mut self.environement)
                        .expect("Could not get mutable reference to environemnt")
                        .define(name.lexeme, value);
                }
                Stmt::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environement.clone());

                    let old_environment = self.environement.clone();
                    self.environement = Rc::new(new_environment);
                    let block_result = self.interpret(statements);
                    self.environement = old_environment;

                    block_result?;
                }
                Stmt::IfStmt {
                    predicate,
                    then,
                    els,
                } => {
                    let truth_value = predicate.evaluate(
                        Rc::get_mut(&mut self.environement)
                            .expect("Could not load mutable ref to env"),
                    )?;
                    if truth_value.is_truthy() == LiteralValue::True {
                        self.interpret(vec![*then])?;
                    } else if let Some(els_stmt) = els{
                        self.interpret(vec![*els_stmt])?;
                    }
                }
            };
        }

        Ok(())
    }
}
