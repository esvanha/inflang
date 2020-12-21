mod lexer;
mod parser;
mod ast;
use crate::lexer::{Lexer, TokenType};
use crate::parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = Parser::new("
        let two = 2;
        let str = \"Hello, world!\";

        let id = fn (x) {
            x
        };
        
        if id(true) {
            print(\"true == true\");
        } else {
            print(\"?\");
        }"
        .to_string()
    );

    loop {
        let expression = parser.parse_expression()?;
        println!("{:#?}", expression);
    }
    print!("\n");

    Ok(())
}
