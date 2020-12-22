mod lexer;
mod parser;
mod ast;
use crate::lexer::{Lexer, TokenType};
use crate::parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser1 = Parser::new("true".to_string());
    println!("{}", parser1.parse_expression()?.boolean_value()?);

    let mut parser2 = Parser::new("3".to_string());
    println!("{}", parser2.parse_expression()?.integer_value()?);

    let mut parser3 = Parser::new("\"Hello, world\"".to_string());
    println!("{}", parser3.parse_expression()?.string_value()?);

    let mut parser4 = Parser::new("x".to_string());
    println!("{}", parser4.parse_expression()?.identifier_name()?);

    Ok(())
}
