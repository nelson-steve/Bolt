#![allow(dead_code)]
#![allow(non_snake_case)]

use std::io::Write;

use substring::Substring;

fn take_input(){
    loop{
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut line =  String::new();
        let _input = std::io::stdin().read_line(&mut line).unwrap();


        let mut l1 = Lexer::new(line);
        loop {
            let token = l1.NextToken();
            if token.kind == SyntaxKind::EndOfFileToken {
                break;
            }
            else if token.kind == SyntaxKind::MinusToken {
                println!("Minus token found");
                std::io::stdout().flush().unwrap();
                break;
            }
            else if token.kind == SyntaxKind::NumberToken {
                println!("Number token found");
                std::io::stdout().flush().unwrap();
                break;
            }
            else {
                println!("Invalid input");
                break;
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
    EndOfFileToken
}
#[derive(PartialEq)]
struct SyntaxToken{
    kind:SyntaxKind,
    position:u64,
    text:String
}
impl SyntaxToken {
    pub fn new(kind:SyntaxKind, position:u64, text:String)->Self{
        SyntaxToken { kind: (kind), position: (position), text: (text) }
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
            return char::from(0);
        }

        let my_vec: Vec<char> = self.text.chars().collect();
        return my_vec[self.position as usize];
    }

    fn Position(self)->u64{
        return self.position;
    }

    fn Next(&mut self)->u64{
        let mut position  = self.position;
        position +=1;
        return position;
    }

    pub fn NextToken(&mut self)->SyntaxToken {
        if self.Current().is_numeric(){
            let start = self.position;
            while self.Current().is_numeric() {
                self.position += 1;
            }
            let length = self.position - start;
            let sub = self.text.substring(start as usize, length as usize);
            return SyntaxToken { kind: (SyntaxKind::NumberToken), position: (start), text: (String::from(sub)) }
        }
        else if char::is_whitespace(self.Current()){
            let start = self.position;

            while char::is_whitespace(self.Current()) {
                self.position += 1;
            }

            // let slice1= self.text.char_indices().nth(2).unwrap().1;
            let length = self.position - start;
            let sub = self.text.substring(start as usize, length as usize);
            return SyntaxToken { kind: (SyntaxKind::NumberToken), position: (start), text: (String::from(sub)) }
        }
        else if self.Current() == '+' {
            // let pos:u64 = self.position;
            return SyntaxToken { kind: (SyntaxKind::PlusToken), position: (self.Next()), text: ("+".to_string()) }
        }
        else if self.Current() == '-' {
            return SyntaxToken { kind: (SyntaxKind::MinusToken), position: (self.Next()), text: ("-".to_string()) }
        }
        else if self.Current() == '*' {
            return SyntaxToken { kind: (SyntaxKind::StarToken), position: (self.Next()), text: ("*".to_string()) }
        }
        else if self.Current() == '/' {
            return SyntaxToken { kind: (SyntaxKind::SlashToken), position: (self.Next()), text: ("/".to_string()) }
        }
        else if self.Current() == '(' {
            return SyntaxToken { kind: (SyntaxKind::OpenParanthesisToken), position: (self.Next()), text: ("(".to_string()) }
        }
        else if self.Current() == ')' {
            return SyntaxToken { kind: (SyntaxKind::CloseParanthesisToken), position: (self.Next()), text: (")".to_string()) }
        }

        let start = self.position;
        let sub = self.text.substring(start as usize, 1);
        return SyntaxToken { kind: (SyntaxKind::CloseParanthesisToken), position: (start), text: (sub).to_string() }

    }
}

fn main() {
    take_input()
    // let l1 = Lexer::new("hello".to_string());
}
