use crate::ast::Expression;

use std::rc::Rc;
use std::collections::HashMap;
use std::io;
use std::io::*;



pub fn builtin_functions() -> HashMap<String, Expression> {
    let mut function_map = HashMap::new();

    function_map.insert(
        "print".to_string(),
        Expression::BuiltInFn(1, Rc::new(|ctx, items| {
            match items[0].clone() {
                Expression::StringValue(string) => print!("{}", string),
                other => print!("{}", other),
            };

            Ok(Expression::Null)
        }
    )));

    function_map.insert(
        "print_line".to_string(),
        Expression::BuiltInFn(1, Rc::new(|ctx, items| {
            match items[0].clone() {
                Expression::StringValue(string) => println!("{}", string),
                other => println!("{}", other),
            };

            Ok(Expression::Null)
        }
    )));

    function_map.insert(
        "+".to_string(),
        Expression::BuiltInFn(2, Rc::new(|ctx, items| {
            let a = items[0].clone().integer_value()?;
            let b = items[1].clone().integer_value()?;

            Ok(Expression::IntegerValue(a + b))
        }
    )));

    function_map.insert(
        "/".to_string(),
        Expression::BuiltInFn(2, Rc::new(|ctx, items| {
            let a = items[0].clone().integer_value()?;
            let b = items[1].clone().integer_value()?;

            Ok(Expression::IntegerValue(a / b))
        }
    )));

    function_map.insert(
        "-".to_string(),
        Expression::BuiltInFn(2, Rc::new(|ctx, items| {
            let a = items[0].clone().integer_value()?;
            let b = items[1].clone().integer_value()?;

            Ok(Expression::IntegerValue(a - b))
        }
    )));

    function_map.insert(
        "*".to_string(),
        Expression::BuiltInFn(2, Rc::new(|ctx, items| {
            let a = items[0].clone().integer_value()?;
            let b = items[1].clone().integer_value()?;

            Ok(Expression::IntegerValue(a * b))
        }
    )));

    function_map.insert(
        "get_input_line".to_string(),
        Expression::BuiltInFn(0, Rc::new(|ctx, items| {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            if input.ends_with('\n') {
                input.pop();
                if input.ends_with('\r') {
                    input.pop();
                }
            }

            Ok(Expression::StringValue(input))
        }
    )));

    function_map.insert(
        "join_str".to_string(),
        Expression::BuiltInFn(2, Rc::new(|ctx, items| {
            let a = items[0].clone().string_value()?;
            let b = items[1].clone().string_value()?;

            Ok(Expression::StringValue(a + &b))
        }
    )));

    function_map
}
