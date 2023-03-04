enum TokenType {
    ShiftLeft,
    ShiftRight,
    Increment,
    Decrement,
    Input,
    Output,
    JumpOver,
    JumpBack,
}

pub struct Tokenizer {
    content: String,
    content_length: i32,
    cursor: usize,
}

impl Tokenizer {
    pub fn new(content: &String, content_length: i32) -> Self {
        Self { 
            content: content.to_string(), 
            content_length,
            cursor: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<char> {
        if self.cursor < self.content_length.try_into().unwrap() {
            let scontent = self.content.to_string();
            let bcontent = scontent.as_bytes();
            let symbol: char = bcontent[self.cursor] as char; 
            self.cursor += 1;
            return Some(symbol)
        }
        else {
            return None
        }
    }
}

