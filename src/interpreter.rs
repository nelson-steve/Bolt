use crate::environment::{self, Environment};
use crate::expr::{LiteralValue, Expr};
use crate::scanner::{Token, TokenType};
use crate::stmt::Stmt;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::{self, Rc};
use std::time::SystemTime;
use std::vec;

pub struct Interpreter {
    pub specials: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
}

fn clock_impl(_args: &Vec<LiteralValue>) -> LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time")
        .as_millis();

    LiteralValue::Number(now as f64 / 1000.0)
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.define(
            "clock".to_string(),
            LiteralValue::Callable {
                name: "clock".to_string(),
                arity: 0,
                fun: Rc::new(clock_impl),
            },
        );
        Self {
            specials: Rc::new(RefCell::new(Environment::new())),
            environment: Rc::new(RefCell::new(env)),
        }
    }

    fn for_closure(parent: Rc<RefCell<Environment>>) -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));
        environment.borrow_mut().enclosing = Some(parent);

        Self {
            specials: Rc::new(RefCell::new(Environment::new())),
            environment,
        }
    }

    pub fn for_anon(parent: Rc<RefCell<Environment>>) -> Self {
        let mut env = Environment::new();
        env.enclosing = Some(parent);
        Self {
            specials: Rc::new(RefCell::new(Environment::new())),
            environment: Rc::new(RefCell::new(env)),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<&Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(self.environment.clone())?;
                }
                Stmt::Print { expression } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    println!("{}", value.to_string());
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(self.environment.clone())?;
                    self.environment
                        .borrow_mut()
                        .define(name.lexeme.to_string(), value);
                }
                Stmt::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(RefCell::new(new_environment));
                    let block_result =
                        self.interpret((*statements).iter().map(|b| b.as_ref()).collect());
                    self.environment = old_environment;

                    block_result?;
                }
                Stmt::IfStmt {
                    predicate,
                    then,
                    els,
                } => {
                    let truth_value = predicate.evaluate(self.environment.clone())?;
                    if truth_value.is_truthy() == LiteralValue::True {
                        let statements = vec![then.as_ref()];
                        self.interpret(statements)?;
                    } else if let Some(els_stmt) = els {
                        let statements = vec![els_stmt.as_ref()];
                        self.interpret(statements)?;
                    }
                }
                Stmt::WhileStmt { condition, body } => {
                    let mut flag = condition.evaluate(self.environment.clone())?;

                    while flag.is_truthy() == LiteralValue::True {
                        let statements = vec![body.as_ref()];
                        self.interpret(statements)?;
                        flag = condition.evaluate(self.environment.clone())?;
                    }
                }
                Stmt::Function { name, params, body } => {
                    let arity = params.len();

                    let params: Vec<Token> = params.iter().map(|t| (*t).clone()).collect();
                    let body: Vec<Box<Stmt>> = body.iter().map(|b| (*b).clone()).collect();
                    let name_clone = name.lexeme.clone();
                    let parent_env = self.environment.clone();
                    let fun_impl = move |args: &Vec<LiteralValue>| {
                        let mut clos_int = Interpreter::for_closure(parent_env.clone());

                        // let n = params.clone();
                        for (i, arg) in args.iter().enumerate() {
                            // let name_clone = n[i].lexeme.clone();
                            clos_int
                                .environment
                                .borrow_mut()
                                .define(params[i].lexeme.clone(), (*arg).clone());
                        }
                        for i in 0..(body.len()) {
                            clos_int
                                .interpret(vec![body[i].as_ref()])
                                .expect(&format!("Evaluating a failed inside {}", name_clone));

                            if let Some(value) = clos_int.specials.borrow_mut().get("return") {
                                return value;
                            }
                        }
                        LiteralValue::Nil

                        //     if let Stmt::ReturnStmt {
                        //         keyword: _,
                        //         value: _,
                        //     } = *body[i]
                        //     {
                        //         let value = clos_int
                        //             .environment
                        //             .borrow_mut()
                        //             .get("return")
                        //             .expect("return value was not defined in the environment even though return statememnt was intereted");
                        //         return value;
                        //     }
                        // }

                        // let value;
                        // match body[body.len() - 1].as_ref() {
                        //     Stmt::Expression { expression } => {
                        //         value = expression.evaluate(clos_int.environment.clone()).unwrap()
                        //     }
                        //     // _ => todo!("Didn't get and expression"),
                        //     _ => todo!("Didn't get and expression"),
                        // }

                        // value
                    };

                    let callable = LiteralValue::Callable {
                        name: name.lexeme.clone(),
                        arity,
                        fun: Rc::new(fun_impl),
                    };

                    self.environment
                        .borrow_mut()
                        .define(name.lexeme.clone(), callable);
                }
                Stmt::ReturnStmt { keyword, value } => {
                    let evaL_val;
                    if let Some(value) = value {
                        evaL_val = value.evaluate(self.environment.clone())?;
                    } else {
                        evaL_val = LiteralValue::Nil;
                    }
                    self.specials
                        .borrow_mut()
                        .define_top_level("return".to_string(), evaL_val);
                }
            };
        }

        Ok(())
    }

    pub fn resolve(&mut self, expr: &Expr, _steps: usize) -> Result<(), String> {
        todo!()
    }
}
