use std::{
    cell::RefCell,
    env,
    fmt::{format, write},
    rc::Rc,
    vec,
};

use crate::{
    environment::{self, Environment},
    interpreter::Interpreter,
    scanner::{self, Token, TokenType},
    stmt::Stmt,
};

#[derive(Clone)]
pub enum LiteralValue {
    Number(f64),
    StringValue(String),
    True,
    False,
    Nil,
    Callable {
        name: String,
        arity: usize,
        fun: Rc<dyn Fn(&Vec<LiteralValue>) -> LiteralValue>,
    },
}

impl std::fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LiteralValue::Number(x), LiteralValue::Number(y)) => x == y,
            (
                LiteralValue::Callable {
                    name,
                    arity,
                    fun: _,
                },
                LiteralValue::Callable {
                    name: name2,
                    arity: arity2,
                    fun: _,
                },
            ) => name == name2 && arity == arity2,
            (LiteralValue::StringValue(x), LiteralValue::StringValue(y)) => x == y,
            (LiteralValue::True, LiteralValue::True) => true,
            (LiteralValue::False, LiteralValue::False) => true,
            (LiteralValue::Nil, LiteralValue::Nil) => true,
            _ => false,
        }
    }
}

fn unwrap_as_f64(literal: Option<scanner::LiteralValue>) -> f64 {
    match literal {
        Some(scanner::LiteralValue::FValue(x)) => x as f64,
        _ => panic!("Could not unwrap as f32"),
    }
}

fn unwrap_as_string(literal: Option<scanner::LiteralValue>) -> String {
    match literal {
        Some(scanner::LiteralValue::StringValue(s)) => s.clone(),
        _ => panic!("Could not unwrap as string"),
    }
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(x) => format!("\"{}\"", x),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Callable {
                name,
                arity,
                fun: _,
            } => format!("{name}|{arity}"),
        }
    }

    pub fn to_type(&self) -> &str {
        match self {
            LiteralValue::Number(_) => "Number",
            LiteralValue::StringValue(_) => "String",
            LiteralValue::True => "Boolean",
            LiteralValue::False => "Boolean",
            LiteralValue::Nil => "Nil",
            LiteralValue::Callable {
                name: _,
                arity: _,
                fun,
            } => "Callable",
        }
    }

    pub fn from_token(token: &Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f64(token.literal.clone())),
            TokenType::StringLit => Self::StringValue(unwrap_as_string(token.literal.clone())),

            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            _ => panic!("Could not create LiteralValue from {:?}", token),
        }
    }

    pub fn from_bool(b: bool) -> Self {
        if b {
            LiteralValue::True
        } else {
            LiteralValue::False
        }
    }

    pub fn is_falsy(&self) -> LiteralValue {
        match self {
            Self::Number(x) => {
                if *x == 0.0 {
                    Self::True
                } else {
                    Self::False
                }
            }
            Self::StringValue(s) => {
                if s.len() == 0 {
                    Self::True
                } else {
                    Self::False
                }
            }
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Nil => Self::True,
            Self::Callable {
                name: _,
                arity: _,
                fun,
            } => panic!("Can not use callable as a falsy value"),
        }
    }

    pub fn is_truthy(&self) -> LiteralValue {
        match self {
            Self::Number(x) => {
                if *x == 0.0 {
                    Self::False
                } else {
                    Self::True
                }
            }
            Self::StringValue(s) => {
                if s.len() == 0 {
                    Self::False
                } else {
                    Self::True
                }
            }
            Self::True => Self::True,
            Self::False => Self::False,
            Self::Nil => Self::False,
            Self::Callable {
                name: _,
                arity: _,
                fun,
            } => panic!("Can not use callable as a truthy"),
        }
    }
}

#[derive(Clone)]
pub enum Expr {
    AnonFunction {
        paren: Token,
        arguments: Vec<Token>,
        body: Vec<Box<Stmt>>,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Expr {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Expr::AnonFunction {
                paren: _,
                arguments,
                body: _,
            } => format!("anon{}", arguments.len()),
            Expr::Assign { name, value } => format!("{name:?} = {}", value.to_string()),
            Expr::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => format!("({} {:?})", (*callee).to_string(), arguments),
            Expr::Grouping { expression } => format!("(group {})", (*expression).to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Unary { operator, right } => {
                let operator_str = &operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})", operator_str, right_str)
            }
            Expr::Variable { name } => format!("(var {})", name.lexeme),
            Expr::Logical {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.to_string(),
                left.to_string(),
                right.to_string()
            ),
        }
    }

    pub fn evaluate(&self, environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, String> {
        let env = environment;
        match self {
            Expr::AnonFunction {
                paren,
                arguments,
                body,
            } => {
                let arity = arguments.clone();
                let env = env.clone();
                let arguments: Vec<Token> = arguments.iter().map(|t| (*t).clone()).collect();
                let body: Vec<Box<Stmt>> = body.iter().map(|b| (*b).clone()).collect();
                let paren = paren.clone();
                let len = arguments.len().clone();
                
                let fun_impl = move |args: &Vec<LiteralValue>| {
                    let mut anon_int = Interpreter::for_anon(env.clone());
                    for (i, arg) in args.iter().enumerate() {
                        anon_int
                            .environment
                            .borrow_mut()
                            .define(arguments[i].lexeme.clone(), (*arg).clone());
                    }

                    for i in 0..(body.len()) {
                        anon_int.interpret(vec![&body[i]]).expect(&format!(
                            "Evaluating failed inside anon function at line {}",
                            paren.lineNumber
                        ));

                        if let Some(value) = anon_int.specials.borrow_mut().get("return") {
                            return value;
                        }
                    }

                    LiteralValue::Nil
                };

                // let anon_env = Interpreter::for_anon(environment.clone());
                Ok(LiteralValue::Callable {
                    name: "anon_funtion".to_string(),
                    arity: len,
                    fun: Rc::new(fun_impl),
                })
            }
            Expr::Assign { name, value } => {
                let new_value = (*value).evaluate(env.clone())?;
                let assign_success = env.borrow_mut().assign(&name.lexeme, new_value.clone());
                if assign_success {
                    Ok(new_value)
                } else {
                    Err(format!(
                        "Variable {} has not been declared - assign",
                        name.lexeme
                    ))
                }
            }
            Expr::Variable { name } => match env.borrow().get(&name.lexeme) {
                Some(value) => Ok(value.clone()),
                None => Err(format!(
                    "Variable '{}' has not been declared - declare",
                    name.lexeme
                )),
            },
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                let callable = (*callee).evaluate(env.clone())?;
                match callable {
                    LiteralValue::Callable { name, arity, fun } => {
                        if arguments.len() != arity {
                            return Err(format!(
                                "Callable {} expected {} arguments but got {}",
                                name,
                                arity,
                                arguments.len()
                            ));
                        }
                        let mut arg_vals = vec![];
                        for arg in arguments {
                            let val = arg.evaluate(env.clone()).unwrap();
                            arg_vals.push(val);
                        }

                        Ok(fun(&arg_vals))
                    }
                    other => Err(format!("{} is not callable", other.to_type())),
                }
            }
            Expr::Literal { value } => Ok((*value).clone()),
            Expr::Logical {
                left,
                operator,
                right,
            } => match operator.token_type {
                TokenType::Or => {
                    let lhs_value = left.evaluate(env.clone())?;
                    let lhs_true = lhs_value.is_truthy();
                    if lhs_true == LiteralValue::True {
                        Ok(lhs_value)
                    } else {
                        right.evaluate(env.clone())
                    }
                }
                TokenType::And => {
                    let lhs_value = left.evaluate(env.clone())?;
                    let lhs_true = lhs_value.is_truthy();
                    if lhs_true == LiteralValue::False {
                        Ok(lhs_true)
                    } else {
                        right.evaluate(env)
                    }
                }
                ttype => Err(format!("Invalid token in logical expression: {}", ttype)),
            },
            Expr::Grouping { expression } => expression.evaluate(env),
            Expr::Unary { operator, right } => {
                let right = right.evaluate(env)?;

                match (&right, operator.token_type) {
                    (LiteralValue::Number(x), TokenType::Minus) => Ok(LiteralValue::Number(-x)),
                    (_, TokenType::Minus) => {
                        Err(format!("Minus not implemented for {}", right.to_type()))
                    }
                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    (_, ttype) => Err(format!("{} is not a valid unary operator", ttype)),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(env.clone())?;
                let right = right.evaluate(env.clone())?;

                match (&left, operator.token_type, &right) {
                    (LiteralValue::Number(x), TokenType::Plus, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x + y))
                    }
                    (LiteralValue::Number(x), TokenType::Minus, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x - y))
                    }
                    (LiteralValue::Number(x), TokenType::Star, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x * y))
                    }
                    (LiteralValue::Number(x), TokenType::Slash, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::Number(x / y))
                    }
                    (LiteralValue::Number(x), TokenType::Greater, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::from_bool(x > y))
                    }
                    (LiteralValue::Number(x), TokenType::GreaterEqual, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::from_bool(x >= y))
                    }
                    (LiteralValue::Number(x), TokenType::Less, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::from_bool(x < y))
                    }
                    (LiteralValue::Number(x), TokenType::LessEqual, LiteralValue::Number(y)) => {
                        Ok(LiteralValue::from_bool(x <= y))
                    }
                    (LiteralValue::StringValue(_), op, LiteralValue::Number(_)) => {
                        Err(format!("{} is not defined string and number", op))
                    }
                    (LiteralValue::Number(_), op, LiteralValue::StringValue(_)) => {
                        Err(format!("{} is not defined string and number", op))
                    }
                    (
                        LiteralValue::StringValue(s1),
                        TokenType::Plus,
                        LiteralValue::StringValue(s2),
                    ) => Ok(LiteralValue::StringValue(format!("{}{}", s1, s2))),
                    (x, TokenType::BangEqual, y) => Ok(LiteralValue::from_bool(x != y)),
                    (x, TokenType::EqualEqual, y) => Ok(LiteralValue::from_bool(x == y)),
                    (
                        LiteralValue::StringValue(s1),
                        TokenType::Greater,
                        LiteralValue::StringValue(s2),
                    ) => Ok(LiteralValue::from_bool(s1 > s2)),
                    (
                        LiteralValue::StringValue(s1),
                        TokenType::GreaterEqual,
                        LiteralValue::StringValue(s2),
                    ) => Ok(LiteralValue::from_bool(s1 >= s2)),
                    (
                        LiteralValue::StringValue(s1),
                        TokenType::Less,
                        LiteralValue::StringValue(s2),
                    ) => Ok(LiteralValue::from_bool(s1 < s2)),
                    (
                        LiteralValue::StringValue(s1),
                        TokenType::LessEqual,
                        LiteralValue::StringValue(s2),
                    ) => Ok(LiteralValue::from_bool(s1 <= s2)),
                    (x, ttype, y) => Err(format!(
                        "{} is not implemented for operands {:?} and {:?}",
                        ttype, x, y
                    )),
                }
            }
        }
    }
    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Binary;

    use super::Expr::*;
    use super::*;

    #[test]
    fn pretty_print_ast() {
        let minus_token = Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            lineNumber: 0,
        };
        let onetwothree = Literal {
            value: LiteralValue::Number(123.0),
        };
        let group = Grouping {
            expression: Box::from(Literal {
                value: LiteralValue::Number(45.67),
            }),
        };

        let multi = Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            lineNumber: 0,
        };
        let ast = Expr::Binary {
            left: Box::from(Unary {
                operator: minus_token,
                right: Box::from(onetwothree),
            }),
            operator: multi,
            right: Box::from(group),
        };

        let result = ast.to_string();
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
