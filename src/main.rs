mod lexer;
mod parser;
mod ast;
mod builtin_functions;

use crate::parser::Parser;

use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Rc::new(RefCell::new(ast::EvaluationContext::new()));

    let mut parser = Parser::from_file("src/example.inf".to_string())?;

    parser
        .parse_program()?
        .evaluate(ctx.clone())?;

    Ok(())
}
