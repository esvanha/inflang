mod lexer;
mod parser;
mod ast;
mod builtin_functions;

use crate::parser::Parser;

use std::rc::Rc;
use std::cell::RefCell;
use std::env;
use std::io::stdout;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let ctx = Rc::new(RefCell::new(ast::EvaluationContext::new()));
    
    //.. Run file
    if args.len() > 1 {
        let mut parser = Parser::from_file(args[1].clone())?;

        parser
            .parse_program()?
            .evaluate(ctx.clone())?;

        return Ok(());
    }

    //.. REPL
    loop {
        print!("inflang:repl> ");
        stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let mut parser = Parser::new(input);

        let evaluated_value = parser
            .parse_expression()?
            .evaluate(ctx.clone())?;

        if evaluated_value != ast::Expression::Null {
            println!("{}", evaluated_value);
        }
    }
}
