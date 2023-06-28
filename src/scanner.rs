use std::fmt;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token(){
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            lineNumber: self.line,
        });

        if errors.len() > 0 {
            let mut joined  = "".to_string();
            errors.iter().for_each(|msg| {
                joined.push_str(msg);
                joined.push_str("\n");
            });
            return Err(joined);
        }   

        Ok(self.tokens.clone())
    }

    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(self: &mut Self) -> Result<(), String>  {
        let c = self.advance();

        match c {
            '(' => Ok(self.add_token(TokenType::LeftParen)),
            ')' => Ok(self.add_token(TokenType::RightParen)),
            '{' => Ok(self.add_token(TokenType::LeftBrace)),
            '}' => Ok(self.add_token(TokenType::RightBrace)),
            ',' => Ok(self.add_token(TokenType::Comma)),
            '.' => Ok(self.add_token(TokenType::Dot)),
            '-' => Ok(self.add_token(TokenType::Minus)),
            '+' => Ok(self.add_token(TokenType::Plus)),
            ';' => Ok(self.add_token(TokenType::Semicolon)),
            '*' => Ok(self.add_token(TokenType::Star)),
            _ =>return Err(format!("Unrecognized char: {}", c)),
        }
    }

    fn advance(self: &mut Self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;

        c as char
    }

    fn add_token(self: &mut Self, token_type: TokenType){
        self.add_token_lit(token_type, None);
    }

    fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>){
        let mut text = "".to_string();
        let bytes = self.source.as_bytes();
        for i in self.start..self.current{
            text.push(bytes[i] as char);
        }

        //let text:String = self.source.as_bytes()[self.start..self.current].iter().collect();

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            lineNumber: self.line,
        });

    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
    
    None,
}

//use TokenType::*;

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    lineNumber: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        lineNumber: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            lineNumber,
        }
    }

    pub fn to_string(self: &Self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
