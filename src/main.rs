// src/lib.rs or src/main.rs

mod lexer;
mod token;

use lexer::lexer::Lexer;

fn main() {
    let input = "your input here".to_string();
    let mut lexer = Lexer::new(input);

    // Example usage
    let token = lexer.next_token();
    println!("{:?}", token);
}

