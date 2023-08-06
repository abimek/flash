use std::io::Read;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,

    // Keywords
    Let,
    Func,
    If,
    Else,
    For,
    True,
    False,
    Return,
    Run,

    // Idents & Literals
    Ident(String),
    Int(i64),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    GreaterThan,
    LessThan,
    And,
    Equal,
    NotEqual,

    // Delimeters
    Comma,
    Semicolon,
    Colon,

    LParen,
    RParen,
    LBrace,
    RBrace
}

pub struct Lexer {
    input: String,
    bytes: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8
}

impl Lexer {
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        }else{
            self.ch = self.bytes[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> Option<u8> {
        if self.read_position >= self.input.len() {
            return None
        }
        return Some(self.bytes[self.read_position])
    }

    fn next_char_is(&mut self, chr: u8) -> bool {
        if let Some(ch) = self.peek_char() {
            if ch == chr {
                return true;
            }
        }
        return false;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            b'=' => {
                if self.next_char_is(b'=') {
                    self.read_char();
                    Token::Equal
                }else{
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b':' => Token::Colon,
            b'!' => {
                if self.next_char_is(b'='){
                    self.read_char();
                    Token::NotEqual
                }else{
                    Token::Bang
                }
            },
            b'>' => Token::GreaterThan,
            b'<' => Token::LessThan,
            b',' => Token::Comma,
            b'&' => {
                if self.next_char_is(b'&') {
                    self.read_char();
                    Token::And
                }else{
                    Token::Illegal
                }
            }
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'a'..=b'z' | b'A'..=b'Z' => {
                let iden = self.read_identifier();
                return match iden.as_str() {
                    // Keywords
                    "func" => Token::Func,
                    "let" => Token::Let,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "for" => Token::For,
                    "true" => Token::True,
                    "false" => Token::False,
                    "return" => Token::Return,
                    "run" => Token::Run,
                    _ => Token::Ident(iden)
                };
            },
            b'0'..=b'9' => {
                let int = self.read_number();
                return Token::Int(int)
            }
            _ => Token::Illegal
        };
        self.read_char();
        return tok
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' | b'\n' | b'\r' => self.read_char(),
                _ => return
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while is_letter(self.ch) || is_digit(self.ch) {
            self.read_char();
        }
        return (&self.input[pos..self.position]).to_string()
    }

    fn read_number(&mut self) -> i64 {
        let pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
            println!("{}", self.ch as char);
        }
        return (&self.input[pos..self.position]).to_string().parse::<i64>().unwrap();
    }
}

fn is_letter(chr: u8) -> bool {
    return match chr {
        b'a'..=b'z' | b'A'..=b'Z' => true,
        _ => false
    }
}

fn is_digit(chr: u8) -> bool {
    return match chr {
        b'0'..=b'9' => true,
        _ => false
    }
}

// NewLexer returns a lexer that will read
pub fn new_lexer(str: impl Into<String>) -> Lexer {
    let inp = str.into();
    let mut lex = Lexer{
        bytes: inp.as_bytes().to_vec(),
        input: inp,
        position: 0,
        read_position: 0,
        ch: 0
    };
    lex.read_char();
    lex
}
#[cfg(test)]
pub mod tests {
    use super::{new_lexer, Token};

    #[test]
    fn test_simple() {
        let input = "=+(){},;";
        let tests: Vec<Token> = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon
        ];

        let mut lexer = new_lexer(input);
        let mut index = 0;
        for tok in tests {
            let next_tok = lexer.next_token();
            if next_tok != tok {
                panic!("Lexer failed: Expected Token {:?} got token {:?} at index {}", tok, next_tok, index)
            }
            index += 1;
        }
        assert!(true);
    }

    #[test]
    fn test_func() {
        let input = "
        func add(x, y) {
            if(x == y && x == true){
                return !x;
            }
        }

        func main() {
            let val = run add(20, 30);
            let val2 = run add(20, 30);
        }";
        let tests: Vec<Token> = vec![
            Token::Func,
            Token::Ident("add".to_owned()),
            Token::LParen,
            Token::Ident("x".to_owned()),
            Token::Comma,
            Token::Ident("y".to_owned()),
            Token::RParen,
            Token::LBrace,
            Token::If,
            Token::LParen,
            Token::Ident("x".to_owned()),
            Token::Equal,
            Token::Ident("y".to_owned()),
            Token::And,
            Token::Ident("x".to_owned()),
            Token::Equal,
            Token::True,
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Bang,
            Token::Ident("x".to_owned()),
            Token::Semicolon,
            Token::RBrace,
            Token::RBrace,
            Token::Func,
            Token::Ident("main".to_owned()),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::Let,
            Token::Ident("val".to_owned()),
            Token::Assign,
            Token::Run,
            Token::Ident("add".to_owned()),
            Token::LParen,
            Token::Int(20),
            Token::Comma,
            Token::Int(30),
            Token::RParen,
            Token::Semicolon,
            Token::Let,
            Token::Ident("val2".to_owned()),
            Token::Assign,
            Token::Run,
            Token::Ident("add".to_owned()),
            Token::LParen,
            Token::Int(20),
            Token::Comma,
            Token::Int(30),
            Token::RParen,
            Token::Semicolon,
            Token::RBrace
        ];

        let mut lexer = new_lexer(input);
        let mut index = 0;
        for tok in tests {
            let next_tok = lexer.next_token();
            if next_tok != tok {
                panic!("Lexer failed: Expected Token {:?} got token {:?} at index {}", tok, next_tok, index)
            }
            index += 1;
        }
        assert!(true);
    }
}
