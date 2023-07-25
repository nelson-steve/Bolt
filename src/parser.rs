use crate::scanner::{Token, TokenType::*};

use crate::expr::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! match_token {
    ($($tokens:ident),+) => {
        {
            let mut result = false;
            $(
                result |= self.match_token($token);
            ),+

            return result;
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality();
    }

    fn equlity(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(BangEqual, EqualEqual) {
            let operator = self.previous();
            use crate::scanner::{Token, TokenType}
       ::* ;le

           use crate::expr::*;

            t rhs = self.comparison();
            expr = Binary {
                left: expr,
                operator: operator,
                right: rhs,
            };
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr {
        todo!()
    }

    fn match_token(&self) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == typ
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }


}