mod lexer;
mod parser;
mod ast;
use crate::lexer::{Lexer, TokenType};
use crate::parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = Parser::new("
        let two = 2;
        let str = \"Hello, world!\";
        "
        .to_string()
    );

    println!("{:#?}", parser.parse_program()?);

    Ok(())
}
