use crate::{
    expr::LiteralValue,
    scanner::{
        Token,
        TokenType::{self, *},
    },
    stmt::{self, Stmt},
};

use crate::expr::{Expr, Expr::*};

#[derive(Debug)]
enum FunctionKind {
    Function,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = vec![];
        let mut errs = vec![];

        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(s) => stmts.push(s),
                Err(msg) => {
                    errs.push(msg);
                    self.synchronization();
                }
            }
        }
        if errs.len() == 0 {
            Ok(stmts)
        } else {
            Err(errs.join("\n"))
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(&TokenType::Var) {
            self.var_declaration()
        } else if self.match_token(&TokenType::Fun){
            self.function(FunctionKind::Function)
        } 
        else {
            self.statement()
        }
    }

    fn function(&mut self, kind: FunctionKind) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, &format!("Expected {kind:?} name"))?;

        self.consume(TokenType::LeftParen, &format!("Expected '(' after {kind:?} name"))?;

        let mut parameters = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    let location = self.peek().lineNumber;
                    return Err(format!(
                        "Line {location}: Cant have more than 255 arguments"
                    ));
                }

                let param = self.consume(TokenType::Identifier, "Expected parameter name")?;
                parameters.push(param);

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expected ')' after paramters.")?;

        self.consume(TokenType::LeftBrace, "Expected '{{' before {kind:?} body.")?;
        let body = match self.block_statement()? {
            Stmt::Block { statements } => statements,
            _ => panic!("Block statement parsed something that was not a block"),
        };

        return Ok(Stmt::Function { name: name, params: parameters, body, });
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let token = self.consume(TokenType::Identifier, "Expected variable name")?;

        let initializer;
        if self.match_token(&TokenType::Equal) {
            initializer = self.expression()?;
        } else {
            initializer = Literal {
                value: LiteralValue::Nil,
            };
        }

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration",
        )?;

        Ok(Stmt::Var {
            name: token,
            initializer: initializer,
        })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(&TokenType::Print) {
            self.print_statement()
        } else if self.match_token(&TokenType::LeftBrace) {
            self.block_statement()
        } else if self.match_token(&TokenType::If) {
            self.if_statement()
        } else if self.match_token(&TokenType::While) {
            self.while_statement()
        } else if self.match_token(&TokenType::For) {
            self.for_statement()
        } else {
            self.expression_statement()
        }
    }

    fn for_statement(&mut self) -> Result<Stmt, String> {
        // for v
        //          ( SMTH; SMTH; SMTH )
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'")?;

        let initializer: Option<Stmt>;
        if self.match_token(&TokenType::Semicolon) {
            initializer = Option::None;
        } else if self.match_token(&TokenType::Var) {
            let var_decl = self.var_declaration()?;
            initializer = Some(var_decl);
        } else {
            let expr = self.expression_statement()?;
            initializer = Some(expr);
        }

        let condition;
        if !self.check(TokenType::Semicolon) {
            let expr = self.comparison()?;
            condition = Some(expr)
        } else {
            condition = Option::None;
        }
        self.consume(TokenType::Semicolon, "Expected ';' after loop condition")?;

        let increment;
        if !self.check(TokenType::RightParen) {
            let expr = self.expression()?;
            increment = Some(expr)
        } else {
            increment = Option::None;
        }
        self.consume(TokenType::RightParen, "Expected ')' after for clause")?;

        let mut body = self.statement()?;

        if let Some(incr) = increment {
            body = Stmt::Block {
                statements: vec![
                    Box::new(body),
                    Box::new(Stmt::Expression { expression: incr }),
                ],
            }
        }

        let cond;
        match condition {
            Option::None => {
                cond = Expr::Literal {
                    value: LiteralValue::True,
                }
            }
            Some(c) => cond = c,
        }
        body = Stmt::WhileStmt {
            condition: cond,
            body: Box::new(body),
        };

        if let Some(init) = initializer {
            body = Stmt::Block {
                statements: vec![Box::new(init), Box::new(body)],
            }
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Exptected ')' after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::WhileStmt {
            condition,
            body: Box::new(body),
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'")?;
        let predicate = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after if-predicate")?;

        let then = Box::new(self.statement()?);
        let els = if self.match_token(&TokenType::Else) {
            let stm = self.statement()?;
            Some(Box::new(stm))
        } else {
            Option::None
        };

        Ok(Stmt::IfStmt {
            predicate,
            then,
            els,
        })
    }

    fn block_statement(&mut self) -> Result<Stmt, String> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let decl = self.declaration()?;
            statements.push(Box::new(decl));
        }

        self.consume(TokenType::RightBrace, "Expected '}' after a block")?;
        Ok(Stmt::Block { statements })
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after  expression.")?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.or()?;

        if self.match_token(&TokenType::Equal) {
            let value = self.assignment()?;

            match expr {
                Variable { name } => Ok(Assign {
                    name,
                    value: Box::from(value),
                }),
                _ => Err("Invalid argument target".to_string()),
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;

        while self.match_token(&TokenType::Or) {
            let operator = self.previous();
            let right = self.and()?;

            expr = Logical {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;

        while self.match_token(&TokenType::And) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Logical {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        // self.advance();

        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let op = self.previous();
            let rhs = self.comparison()?;
            expr = Binary {
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
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                let arg = self.expression()?;
                arguments.push(arg);
                if arguments.len() >= 255 {
                    let location = self.peek().lineNumber;
                    return Err(format!(
                        "Line {location}: Cant have more than 255 arguments"
                    ));
                }

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen, "Expected ')' after arguments.")?;

        Ok(Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek();

        let result;
        match token.token_type {
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')'")?;
                result = Grouping {
                    expression: Box::from(expr),
                };
            }
            TokenType::False
            | TokenType::True
            | TokenType::Nil
            | TokenType::Number
            | TokenType::StringLit => {
                self.advance();
                result = Literal {
                    value: LiteralValue::from_token(&token),
                }
            }
            TokenType::Identifier => {
                self.advance();
                result = Variable {
                    name: self.previous(),
                };
            }
            _ => return Err("Expected expression".to_string()),
        }

        return Ok(result);
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            let token = self.previous();
            Ok(token)
        } else {
            Err(msg.to_string())
        }
    }

    fn check(&mut self, typ: TokenType) -> bool {
        self.peek().token_type == typ
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
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
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
        let one = Token {
            token_type: TokenType::Number,
            lexeme: "1".to_string(),
            literal: Some(FValue(1.0)),
            lineNumber: 0,
        };
        let plus = Token {
            token_type: Plus,
            lexeme: "+".to_string(),
            literal: Option::None,
            lineNumber: 0,
        };
        let two = Token {
            token_type: Number,
            lexeme: "2".to_string(),
            literal: Some(FValue(2.0)),
            lineNumber: 0,
        };
        let semicolon = Token {
            token_type: Semicolon,
            lexeme: ";".to_string(),
            literal: Option::None,
            lineNumber: 0,
        };
        let eof = Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: Option::None,
            lineNumber: 0,
        };

        let tokens = vec![one, plus, two, semicolon, eof];
        println!("test addition tokens length: {}", tokens.len());
        let mut parser = Parser::new(tokens);

        let parsed_expr = parser.parse().unwrap();
        assert_eq!(parsed_expr.len(), 1);
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(+ 1 2)");
    }

    #[test]
    fn test_comparison() {
        let source = "1 + 2 == 5 + 7;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(== (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn test_eq_with_paren() {
        let source = "1 == (2 + 2);";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(== 1 (group (+ 2 2)))");
    }
}
