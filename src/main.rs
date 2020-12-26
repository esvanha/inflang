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
        let inc = fn (n) {
            +(n, 1);
        };

        let is_prime = fn (x) {
            if <(x, 3) {
                if eq(x, 2) {
                    true;
                } else {
                    false;
                };
            } else {
                let i = 2;
                let sqrt_x = sqrt(x);
                let stop = false;
                let result = true;
                
                while not(stop) {
                    if eq(mod(x, i), 0) {
                        let stop = true;
                        let result = false;
                    } else {
                        let i = inc(i);
                        let stop = >(i, sqrt_x);
                    };
                };

                result;
            };
        };

        print_line(\"Prime Number Test\");
        print_line(\"What number do you want to test?\");

        let number = str_to_int(get_input_line());

        if is_prime(number) {
            print_line(\"This is a prime number!\");
        } else {
            print_line(\"This is not a prime number!\");
        };
        "
        .to_string()
    );

    parser
        .parse_program()?
        .evaluate(ctx.clone())?;

    Ok(())
}
