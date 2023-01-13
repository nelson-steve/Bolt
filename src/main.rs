#![allow(dead_code)]

fn take_input(){
    loop{
        print!(">");
        let mut line =  String::new();
        let _input = std::io::stdin().read_line(&mut line).unwrap();

        if line.contains("exit"){
            break;
        }

        if line.is_empty() {
            return;
        }
        else if line.contains("1")||line.contains("2"){
            print!("{}",line);
        }
        else{
            println!("Invalied expression");
        }
    }
}

enum SyntaxKind {
    NumberToken,
    WhitespaceToken,
    PlusToken
}

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

struct Lexer{
    position:u64,
    text:String
}
impl Lexer {
    fn new(text:String)->Self{
        Lexer { position: (0), text: (text) }
    }

    fn Current(self)->char {
        if self.position >= self.text.len().try_into().unwrap(){
            return char::from(0);
        }

        let my_vec: Vec<char> = self.text.chars().collect();
        return my_vec[self.position as usize];
    }

    pub fn NextToken(self)->SyntaxToken {
        if char::is_digit(self.Current(), 1) {
            let mut start = self.position;

            while char::is_digit(self.Current(), 1) {
                self.position += 1;
            }

            let length = self.position - start;
            let slice = self.text.chars().skip(start as usize).take(length as usize);
            return SyntaxToken { kind: (SyntaxKind::NumberToken), position: (start), text: (self.text) }
        }
        if char::is_whitespace(self.Current()){
            let mut start = self.position;

            while char::is_whitespace(self.Current()) {
                self.position += 1;
            }

            let length = self.position - start;
            let slice = self.text.chars().skip(start as usize).take(length as usize);
            return SyntaxToken { kind: (SyntaxKind::NumberToken), position: (start), text: (self.text) }
        }
        if (self.Current() == '+'){
            let pos:u64 = self.position;
            return SyntaxToken { kind: (SyntaxKind::PlusToken), position: (), text: ("+".to_string()) }
        }Z

        return SyntaxToken { kind: (), position: (), text: () }

    }
}

fn main() {
    // take_input()
    // let l1 = Lexer::new(, 2, "hello".to_string());
}
