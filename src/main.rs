#![allow(dead_code)]
#![allow(non_snake_case)]

use std::{io::Write};

use substring::Substring;

fn take_input(){
    loop{
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut line =  String::new();
        let _input = std::io::stdin().read_line(&mut line).unwrap();

        let mut l1 = Lexer::new(line.clone());
        loop {
            let token = l1.NextToken();
            if token.kind == SyntaxKind::EndOfFileToken {
                println!("End of file token found");
                break;
            }
            else if token.kind == SyntaxKind::MinusToken {
                println!("Minus token found");
                std::io::stdout().flush().unwrap();
                // break;
            }
            else if token.kind == SyntaxKind::PlusToken {
                println!("Plus token found");
                std::io::stdout().flush().unwrap();
                // break;
            }
            else if token.kind == SyntaxKind::StarToken {
                println!("Star token found");
                std::io::stdout().flush().unwrap();
                // break;
            }
            else if token.kind == SyntaxKind::WhitespaceToken {
                println!("Whitespace token found");
                std::io::stdout().flush().unwrap();
                // break;
            }
            else if token.kind == SyntaxKind::NumberToken {
                println!("Number token found");
                std::io::stdout().flush().unwrap();
                // break;
            }
            else if token.kind == SyntaxKind::IllegalToken {
                println!("Illegal token found");
                std::io::stdout().flush().unwrap();
                // break;
            }
            else {
                println!("Invalid input");
                // break;
            };
        }
        // if line == "1 + 2 * 3".to_string() {
        //     println!("7");
        // }

        // if line.contains("exit") {
        //     break;
        // }

        // if line.is_empty() {
        //     return;
        // }
        // else{
        //     println!("Invalied expression");
        // }
    }
}
#[derive(PartialEq)]
enum SyntaxKind {
    NumberToken,
    WhitespaceToken,
    PlusToken,
    MinusToken,
    StarToken,
    SlashToken,
    OpenParanthesisToken,
    CloseParanthesisToken,
    EndOfFileToken,
    IllegalToken
}
#[derive(PartialEq)]
struct SyntaxToken{
    kind:SyntaxKind,
    position:u64,
    text:String,
    value:u64
}
impl SyntaxToken {
    pub fn new(kind:SyntaxKind, position:u64, text:String, value:u64)->Self{
        SyntaxToken { kind: (kind), position: (position), text: (text), value: (value) }
    }
}

#[derive(Debug, Clone)]
struct Lexer{
    position:u64,
    text:String
}

impl Lexer {
    fn new(text:String)->Self{
        Lexer { position: (0), text: (text) }
    }

    fn Current(&self)->char {
        if self.position >= self.text.len().try_into().unwrap(){
            return char::from('e');
        }

        let my_vec: Vec<char> = self.text.chars().collect();
        return my_vec[self.position as usize];
    }

    fn Position(self)->u64{
        return self.position;
    }

    fn Next(&mut self)->u64{
        self.position += 1;
        return self.position;
    }

    pub fn NextToken(&mut self)->SyntaxToken {
        if self.Current().is_numeric(){
            let start = self.position;
            while self.Current().is_numeric() {
                self.Next();
            }
            let length = self.position - start;
            let sub = self.text.substring(start as usize, length as usize);
            return SyntaxToken { kind: (SyntaxKind::NumberToken), position: (start), text: (String::from(sub)), value:length }
        }
        else if self.Current() == char::from('e') {
            return SyntaxToken { kind: (SyntaxKind::EndOfFileToken), position: self.position, text: "\0".to_string(), value:1 };
        }
        else if self.Current().is_whitespace() {
            // char::is_whitespace(self.Current()) 
            let start = self.position;

            while char::is_whitespace(self.Current()) {
                self.Next();
            }

            // let slice1= self.text.char_indices().nth(2).unwrap().1;
            let length = self.position - start;
            let sub = self.text.substring(start as usize, length as usize);
            return SyntaxToken { kind: (SyntaxKind::WhitespaceToken), position: start, text: (String::from(sub)), value: length }
        }
        else if self.Current() == '+' {
            let pos = self.position;
            self.Next();
            return SyntaxToken { kind: (SyntaxKind::PlusToken), position: pos, text: ("+".to_string()), value: 1 }
        }
        else if self.Current() == '-' {
            let pos = self.position;
            self.Next();
            return SyntaxToken { kind: (SyntaxKind::MinusToken), position: pos, text: ("-".to_string()), value: 1 }
        }
        else if self.Current() == '*' {
            let pos = self.position;
            self.Next();
            return SyntaxToken { kind: (SyntaxKind::StarToken), position: pos, text: ("*".to_string()), value: 1 }
        }
        else if self.Current() == '/' {
            let pos = self.position;
            self.Next();
            return SyntaxToken { kind: (SyntaxKind::SlashToken), position: pos, text: ("/".to_string()), value: 1 }
        }
        else if self.Current() == '(' {
            let pos = self.position;
            self.Next();
            return SyntaxToken { kind: (SyntaxKind::OpenParanthesisToken), position: pos, text: ("(".to_string()), value: 1 }
        }
        else if self.Current() == ')' {
            let pos = self.position;
            self.Next();
            return SyntaxToken { kind: (SyntaxKind::CloseParanthesisToken), position: pos, text: (")".to_string()), value: 1 }
        }
        else if self.position >= self.text.len().try_into().unwrap() {
        let pos = self.position;
        self.Next();
        return SyntaxToken { kind: (SyntaxKind::EndOfFileToken), position: pos, text: "\0".to_string(), value:1 };
        }
        else {
            let pos = self.position;
            self.Next();
            return SyntaxToken { kind: (SyntaxKind::IllegalToken), position: pos, text: self.Current().to_string(), value: 1 }
        }
    }
}

fn main() {
    take_input()
    // let l1 = Lexer::new("hello".to_string());
}
