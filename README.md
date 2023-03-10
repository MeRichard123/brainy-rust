# brainy-rust
A Rust Based Brain Fuck Interpreter. 

## Features
1. Lexer.rs which [tokenizes](https://en.wikipedia.org/wiki/Lexical_analysis) the file
2. stack.rs an bare bones implemetation of a stack for the looping implementation
3. Main.rs Interpreter and Parser

> Note: This implementation doesn't use an AST hence we don't syntax check we only check for invalid tokens. If you do not open a loop just close it, the interperter will bring you up on it because there was no relevant [. 



## Dealing with Loops \[ and \] 
I had two solutions 
![](./diagrams/Handling-Iteration.png)
