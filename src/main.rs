// https://dgtools.readthedocs.io/en/latest/brainfuck.html
use lexer::Tokenizer;
use lexer::Intrinsic;
use stack::LoopStack;

use std::env;
use std::fs;
use std::io;

pub mod lexer;
pub mod stack;

const MAX_SIZE: usize = 1000;

fn lex_file(file_path: &String) -> Tokenizer{
    let contents = fs::read_to_string(file_path)
        .expect("Read File");
    let length = contents.trim().len() as i32;
    let lexer = Tokenizer::new(&contents, length);
    
    return lexer;
}


fn interpret(program: &Vec<lexer::Token>) {
    let mut tape: [i32; MAX_SIZE] = [0; MAX_SIZE];
    let mut pointer: usize = 0;
    let mut program_counter: i32 = 0;
    let mut stdout: Vec<String> = vec![];
    let mut loop_stack: LoopStack<i32> = LoopStack::new();  

    while program_counter < program.len().try_into().unwrap() {
        let ptr = program_counter as usize;
        let instruction: &lexer::Token = &program[ptr];
        match instruction.intrinsic {
            Intrinsic::Increment  => {
                if tape[pointer] > 255 {
                    tape[pointer] = 0;
                } else{
                    tape[pointer] += 1;   
                }
            },

            Intrinsic::Decrement  => {
                if tape[pointer] < 1 {
                    tape[pointer] = 255;
                } else {
                    tape[pointer] -= 1;
                }
            },

            Intrinsic::ShiftLeft  => {
                if pointer < 1 {
                    pointer = 0;
                } else {
                    pointer -= 1;
                }
            },

            Intrinsic::ShiftRight => {
                if pointer > MAX_SIZE.try_into().unwrap() {
                    pointer = MAX_SIZE - 1;
                } else {
                    pointer += 1;
                }
            },

            Intrinsic::Input      => {
                let mut user_input = String::new();
                
                io::stdin()
                    .read_line(&mut user_input)
                    .unwrap();
                let parsed_input = user_input.trim();

                match parsed_input.parse::<char>() {
                    Ok(i) => {
                        tape[pointer] = i as i32;
                    },
                    Err(..) => println!("Invalid Input please enter a character."),
                }
            },

            Intrinsic::Output     => {
                let ascii = char::from_u32(tape[pointer].try_into().unwrap()); 
                match ascii {
                    Some(x) => stdout.push(x.to_string()),
                    None => print!(""),
                }
            },

            Intrinsic::JumpOver   => {
                loop_stack.push(program_counter);
            },

            Intrinsic::JumpBack   => {
                if tape[pointer] > 0 {
                    let block_start_pointer:Option<&i32> = loop_stack.peek();
                    match block_start_pointer {
                        Some(pointer_start) => {
                            if tape[pointer] > 0 {
                                program_counter = *pointer_start;
                            }
                            else if tape[pointer] == 0 {
                                loop_stack.pop();
                            }
                        },
                        None => panic!("Closing Interation Block Token missing Opening ["),
                    }

                }
            },
        }
        program_counter += 1;
    }
    println!("{}", stdout.join(""));
} 

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Useage:");
        println!("cargo run ./bf/hello.bf");
        panic!("File Path not Provided...");
    }
    let mut lexer: Tokenizer = lex_file(&args[1]);
    let token_vector: Vec<lexer::Token> = lexer.tokenize();
        
    interpret(&token_vector);
}
