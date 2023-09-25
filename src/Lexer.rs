#[derive(Debug)]
pub enum Intrinsic {
    ShiftLeft,
    ShiftRight,
    Increment,
    Decrement,
    Input,
    OutputChar,
    OutputInt,
    JumpOver,
    JumpBack,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
}

#[derive(Debug)]
pub struct Loc {
    pub line: i32,
    pub column: i32,
}


#[derive(Debug)]
pub struct Token {
    pub intrinsic: Intrinsic,
    pub position: Loc,
}

pub struct Tokenizer {
    content: String,
    content_length: i32,
    cursor: usize,
    line: i32,
    column: i32,
}

const TOKEN_KINDS: [char; 12]  = ['+','-', '>', '<', '.','#', ',', ']','[', '&', '|', '!'];

fn report_compiletime_error(token: char, line: i32, col: i32, content_slice: &str) {
    println!("Unexpected token: {token} found at {line}:{col}");
    println!("\n{}", content_slice.trim());
    
    // determine if we have sliced because if not we can just place a ^ on the col
    let spaces_repeat;
    if col > 10 {
        spaces_repeat = col - (col - 7) + 1;
    }else{
        spaces_repeat = col - 2;
    }
    
    let spaces = " ".repeat((spaces_repeat).try_into().unwrap());  
    println!("{spaces}^");
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
        // gets individual lexemes in order
        
        // make sure we aren't at the end.
        if self.cursor < self.content_length.try_into().unwrap() {
            // need to convert to bytes so we can index it
            let bcontent = self.content.as_bytes();
            let symbol: char = bcontent[self.cursor] as char; 
            self.cursor += 1;
            
            // skipping escape characters
            if symbol == '\n' || symbol == '\r' || symbol == '\t' {
                self.column = 1;
                self.line += 1;
                return None;
            }
           
            // skipping lines with a comment.
            if self.column == 1 && !TOKEN_KINDS.contains(&symbol) {
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
       assert!(TOKEN_KINDS.len() == 12, "Exhaustive Keyword Handling");

       let pos: Loc = Loc { line: self.line, column: self.column };

       match lexeme {
            Some('+') => {
                let token = Token { intrinsic: Intrinsic::Increment, position: pos };
                return Some(token);
            },
            Some('-') => {
                let token = Token { intrinsic: Intrinsic::Decrement, position: pos };
                return Some(token);
            },
            Some('>') => {
                let token = Token { intrinsic: Intrinsic::ShiftRight, position: pos };
                return Some(token);
            },
            Some('<') => {
                let token = Token { intrinsic: Intrinsic::ShiftLeft, position: pos };
                return Some(token);
            },
            Some('[') => {
                let token = Token { intrinsic: Intrinsic::JumpOver, position: pos };
                return Some(token);
            },
            Some(']') => {
                let token = Token { intrinsic: Intrinsic::JumpBack, position: pos };
                return Some(token);
            },
            Some(',') => {
                let token = Token { intrinsic: Intrinsic::Input, position: pos };
                return Some(token);
            },
            Some('.') => {
                let token = Token { intrinsic: Intrinsic::OutputChar, position: pos };
                return Some(token);
            },
            Some('#') => {
                let token = Token { intrinsic: Intrinsic::OutputInt, position: pos };
                return Some(token);
            },
            Some('&') => {
                let token = Token { intrinsic: Intrinsic::BitwiseAnd, position: pos };
                return Some(token);
            },
            Some('|') => {
                let token = Token { intrinsic: Intrinsic::BitwiseOr, position: pos };
                return Some(token);
            },
            Some('!') => {
                let token = Token { intrinsic: Intrinsic::BitwiseNot, position: pos };
                return Some(token);
            },
            Some(x) => {
                let lines = &self.content.split("\n");
                let lines_vec = lines.clone()
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|line| line.trim())
                    .collect::<Vec<&str>>();

                let error_line = lines_vec[((self.line - 1) as usize)];
                
                println!("{}", self.column);
                let mut start = 0;
                // handle underflow
                if self.column > 10 {
                    start = (self.column - 10) as usize;
                }
                
                let mut end = (self.column + 7) as usize;
                // handle overflow
                if end > error_line.len() - 1 {
                    end = error_line.len() - 1;
                }
                let code_slice: &str = &error_line[start..end];

                report_compiletime_error(x, self.line, self.column, code_slice);
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
