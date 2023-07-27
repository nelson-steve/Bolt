use crate::{
    expr::LiteralValue,
    scanner::{
        Token,
        TokenType::{self, *},
    },
};

use crate::expr::{Expr, Expr::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// macro_rules! match_tokens {
//     ($parser:ident, $($token:ident),+) => {
//         {
//             let mut result = false;
//             {
//                 $(result |= $parser.match_token($token);)*
//             }

//             return result;
//         }
//     }
// }

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        // self.advance();

        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let op = self.previous();
            let rhs = self.comparison();
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
            // matches_eq = self.match_tokens(&[BangEqual, EqualEqual]);
        }

        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let expr = self.term();

        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op = self.previous();
            let rhs = self.term();
            let expr = Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
            return expr;
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let rhs = self.factor();
            expr = Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }

        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_tokens(&[Slash, Star]) {
            let op = self.previous();
            let rhs = self.unary();
            expr = Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            }
        }

        return expr;
    }

    fn match_token(&mut self, typ: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().token_type == *typ {
                self.advance();
                return true;
            } else {
                return false;
            }
        }
    }

    fn match_tokens(&mut self, typs: &[TokenType]) -> bool {
        for typ in typs {
            if self.match_token(typ) {
                return true;
            }
        }
        return false;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[Bang, Minus]) {
            let op = self.previous();
            let rhs = self.unary();
            Unary {
                operator: op,
                right: Box::from(rhs),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(&TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected ')'");
            Grouping {
                expression: Box::from(expr),
            }
        } else {
            let token = self.peek().clone();
            self.advance();
            Literal {
                value: LiteralValue::from_token(&token),
            }
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
        } else {
            panic!("{}", msg);
        }
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::LiteralValue::*;
    // use LiteralValue::*;

    #[test]
    fn test_addition() {
        let one = Token{
            token_type: TokenType::Number,
            lexeme: "1".to_string(),
            literal: Some(IntValue(1)),
            lineNumber: 0,
        };
        let plus = Token{
            token_type: Plus,
            lexeme: "+".to_string(),
            literal: Option::None,
            lineNumber: 0,
        };
        let two = Token{
            token_type: Number,
            lexeme: "2".to_string(),
            literal: Some(IntValue(2)),
            lineNumber: 0,
        };
        let semicolon = Token{
            token_type: Semicolon,
            lexeme: ";".to_string(),
            literal: Option::None,
            lineNumber: 0,
        };

        let tokens = vec![one, plus, two, semicolon];
        let mut parser = Parser::new(tokens);

        let parsed_expr = parser.expression();
        let string_expr = parsed_expr.to_string();

        assert_eq!(string_expr, "(+ 1 2)");
    }
}