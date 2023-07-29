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

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        // self.advance();

        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let op = self.previous();
            let rhs = self.comparison()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
            // matches_eq = self.match_tokens(&[BangEqual, EqualEqual]);
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op = self.previous();
            let rhs = self.term()?;
            expr = Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }
        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let rhs = self.factor()?;
            expr = Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_tokens(&[Slash, Star]) {
            let op = self.previous();
            let rhs = self.unary()?;
            expr = Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            }
        }

        return Ok(expr);
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

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[Bang, Minus]) {
            let op = self.previous();
            let rhs = self.unary()?;
            Ok(Unary {
                operator: op,
                right: Box::from(rhs),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek();

        let result;
        match token.token_type {
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')'")?;
                result = Grouping { expression: Box::from(expr), };
            }
            TokenType::False | TokenType::True | TokenType::Nil | TokenType::Number | TokenType::StringLit => {
                self.advance();
                result = Literal { value: LiteralValue::from_token(&token), }
            }
            _ => return Err("Expected expression".to_string()),
        }
        
        return Ok(result);
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), String>{
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            Ok(())
        } else {
            Err(msg.to_string())
            // panic!("{}", msg);
        }
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn synchronization(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            
            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | 
                TokenType::While | TokenType::Print | TokenType::Return => return,
                _ => (),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::string;

    use super::*;
    use crate::scanner::{LiteralValue::*, Scanner};
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

        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr.to_string();

        assert_eq!(string_expr, "(+ 1 2)");
    }

    #[test]
    fn test_comparison() {
        let source = "1 + 2 == 5 + 7";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr.to_string();

        assert_eq!(string_expr, "(== (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn test_eq_with_paren() {
        let source = "1 == (2 + 2)";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr.to_string();

        assert_eq!(string_expr, "(== 1 (group (+ 2 2)))");
    }
}