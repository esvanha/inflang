mod lexer;
mod parser;
mod ast;
use crate::lexer::{Lexer, TokenType};
use crate::parser::Parser;

use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Rc::new(RefCell::new(ast::EvaluationScope::new()));

    let mut parser = Parser::new("
        if false {
            \"true\";
        } else {
            \"false\";
        }
        "
        .to_string()
    );
    println!(
        "{}",
        parser
            .parse_expression()?
            .evaluate(ctx.clone())?
            .string_value()?
        );

    Ok(())
}
