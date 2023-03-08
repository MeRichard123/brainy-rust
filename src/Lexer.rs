#[derive(Debug)]
pub enum Intrinsic {
    ShiftLeft,
    ShiftRight,
    Increment,
    Decrement,
    Input,
    Output,
    JumpOver,
    JumpBack,
}

#[derive(Debug)]
pub struct Token {
    pub intrinsic: Intrinsic,
    pub position: usize,
}

pub struct Tokenizer {
    content: String,
    content_length: i32,
    cursor: usize,
    line: i32,
    column: i32,
}

fn report_error(token: char, line: i32, col: i32) {
    println!("Unexpected token: {token} found at {line}:{col}");
    panic!("Unexpected Token");
}


impl Tokenizer {
    pub fn new(content: &String, content_length: i32) -> Self {
        Self { 
            content: content.to_string(), 
            content_length,
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    fn next_lexeme(&mut self) -> Option<char> {
        if self.cursor < self.content_length.try_into().unwrap() {
            let scontent = self.content.to_string();
            let bcontent = scontent.as_bytes();
            let symbol: char = bcontent[self.cursor] as char; 
            self.cursor += 1;
            
            // skipping escape characters
            if symbol == '\n' || symbol == '\r' || symbol == '\t' {
                self.column = 1;
                self.line += 1;
                return None;
            }
           
            // skipping lines with a comment.
            if self.column == 1 && symbol.is_alphabetic() {
                let mut s: char = bcontent[self.cursor] as char;
                while s != '\n'{
                    self.cursor += 1;
                    s = bcontent[self.cursor] as char;
                }
                return None;
            }

            self.column += 1;

            return Some(symbol)
        }
        else {
            return None
        }
    }

   fn tokenize_single_character(&mut self, lexeme: Option<char>) -> Option<Token> {
        match lexeme {
            Some('+') => {
                let token = Token { intrinsic: Intrinsic::Increment, position: self.cursor };
                return Some(token);
            },
            Some('-') => {
                let token = Token { intrinsic: Intrinsic::Decrement, position: self.cursor };
                return Some(token);
            },
            Some('>') => {
                let token = Token { intrinsic: Intrinsic::ShiftRight, position: self.cursor };
                return Some(token);
            },
            Some('<') => {
                let token = Token { intrinsic: Intrinsic::ShiftLeft, position: self.cursor };
                return Some(token);
            },
            Some('[') => {
                let token = Token { intrinsic: Intrinsic::JumpOver, position: self.cursor };
                return Some(token);
            },
            Some(']') => {
                let token = Token { intrinsic: Intrinsic::JumpBack, position: self.cursor };
                return Some(token);
            },
            Some(',') => {
                let token = Token { intrinsic: Intrinsic::Input, position: self.cursor };
                return Some(token);
            },
            Some('.') => {
                let token = Token { intrinsic: Intrinsic::Output, position: self.cursor };
                return Some(token);
            },
            Some(x) => {
                report_error(x, self.line, self.column);
                return None;
            },
            None => {
                return None;
            },
        }   
   }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        
        while self.cursor < self.content_length.try_into().unwrap() {
            let lexeme: Option<char> = self.next_lexeme();
            let token: Option<Token> = self.tokenize_single_character(lexeme);
            match token {
                Some(t) => {
                    tokens.push(t);
                },
                None => (),
            }
        } 
        return tokens;
    } 
}
