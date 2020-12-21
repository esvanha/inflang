mod lexer;
mod parser;
mod ast;
use crate::lexer::{Lexer, TokenType};
use crate::parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = Parser::new("
        let return_two = fn () {
            2;
        };
    
        let result = if eq(return_two(), add(1, 1)) {
            \"true\";
        } else {
            \"false\";
        };
        
        print(result);
        "
        .to_string()
    );

    println!("{:#?}", parser.parse_program()?);

    Ok(())
}
