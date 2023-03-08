pub struct LoopStack<T> {
  stack: Vec<T>,
}

impl<T> LoopStack<T> {
    pub fn new() -> Self {
        LoopStack { stack: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}


