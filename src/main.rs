
mod token;

use token::token::{Token, TokenType, lookup_ident};

fn main() {
    let token = Token {
        r#type: TokenType::IDENT,
        literal: String::from("foobar"),
        line: 1,
        column: 1,
    };

    println!("{:?}", token);

    let keyword_type = lookup_ident("let");
    println!("{:?}", keyword_type);
}

