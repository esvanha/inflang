mod lexer;
mod parser;
mod ast;
mod builtin_functions;

use crate::parser::Parser;

use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Rc::new(RefCell::new(ast::EvaluationScope::new()));

    let mut parser = Parser::new("
        print_line(\"What is your name?\");

        let name = get_input_line();

        print_line(join_str(\"Hello, \", name));
        "
        .to_string()
    );

    parser
        .parse_program()?
        .evaluate(ctx.clone())?;

    Ok(())
}
