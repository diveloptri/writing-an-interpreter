use std::io::{self, Write};
use crate::evaluator::evaluator;
use crate::lexer::lexer::Lexer;
use crate::object::object::{Environment, Object};
use crate::parser::parser;

pub const PROMPT: &'static str = ">> ";
pub const CHIMERA: &'static str = r#"
(╯°□°）╯︵ ┻━┻ 
"#;

pub fn start_repl() {
    let mut environment = Environment::new();
    loop {
        let lexer = Lexer::new(prompt());
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();

        if parser.errors().len() != 0 {
            print_parser_error(parser.errors());
            continue;
        };
        
        let evaluated = evaluator::eval(&program, &mut environment);
        match evaluated {
            Object::Null => (),
            _ => println!("{}", evaluated.inspect()),
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

fn print_parser_error(errors: Vec<String>) {
    println!("{}", CHIMERA);
    println!("Woops! We ran into some errors here!");
    println!("Parser errors:");
    errors.iter().for_each(
        |e| println!("\t{}", e)
    );
}
