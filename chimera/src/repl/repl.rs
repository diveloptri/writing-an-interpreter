use std::io::{self, Write};
use crate::lexer::lexer::Lexer;
use crate::token::token::*;

pub const PROMPT: &'static str = ">> ";

pub fn start_repl() {
    loop {
        let mut lexer = Lexer::new(prompt());

        loop {
            let token = lexer.next_token();
            if token.token_type == EOF {
                break;
            }
            println!("{:?}", token);
            
        }
    }
}

fn prompt() -> String {
    let mut user_input = String::new();
    print!("{PROMPT}");

    std::io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    return user_input.trim().to_string()
}
