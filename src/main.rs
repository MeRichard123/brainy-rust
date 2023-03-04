// https://dgtools.readthedocs.io/en/latest/brainfuck.html
use Lexer::Tokenizer;
use std::env;
use std::fs;

pub mod Lexer;

fn lex_file(file_path: &String) -> Tokenizer{
    let contents = fs::read_to_string(file_path)
        .expect("Read File");
    let length = contents.trim().len() as i32;
    let lexer = Tokenizer::new(&contents, length);
    
    return lexer;
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Useage:");
        println!("./brainfuck-interpreter ./bf/hello.bf");
        panic!("File Path not Provided...");
    }
    let mut lexer: Tokenizer = lex_file(&args[1]);
    let c = lexer.next_token();
    let cn = lexer.next_token();
    println!("{:#?}", c);
    println!("{:#?}", cn);
    println!("Hello, world!");
}
