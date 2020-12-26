mod lexer;
mod parser;
mod ast;
use crate::parser::Parser;

use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Rc::new(RefCell::new(ast::EvaluationScope::new()));

    let mut parser = Parser::new("
        let bool_to_str = fn (x) {
            if x {
                \"true\";
            } else {
                \"false\";
            };
        };
        
        bool_to_str(true);
        bool_to_str(false);
        "
        .to_string()
    );
    println!(
        "{:#?}",
        parser
            .parse_program()?
            .evaluate(ctx.clone())?
        );

    Ok(())
}
