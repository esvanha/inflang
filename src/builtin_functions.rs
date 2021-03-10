use crate::ast::Expression;

use std::rc::Rc;
use std::collections::HashMap;

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
        "<".to_string(),
        Expression::BuiltInFn(2, Rc::new(|ctx, items| {
            let a = items[0].clone().integer_value()?;
            let b = items[1].clone().integer_value()?;

            Ok(Expression::BooleanValue(a < b))
        }
    )));

    function_map.insert(
        ">".to_string(),
        Expression::BuiltInFn(2, Rc::new(|ctx, items| {
            let a = items[0].clone().integer_value()?;
            let b = items[1].clone().integer_value()?;

            Ok(Expression::BooleanValue(a > b))
        }
    )));

    function_map.insert(
        "eq".to_string(),
        Expression::BuiltInFn(2, Rc::new(|ctx, items| {
            Ok(Expression::BooleanValue(items[0] == items[1]))
        }
    )));

    function_map.insert(
        "not".to_string(),
        Expression::BuiltInFn(1, Rc::new(|ctx, items| {
            let a = items[0].boolean_value()?;
            Ok(Expression::BooleanValue(!a))
        }
    )));

    function_map.insert(
        "mod".to_string(),
        Expression::BuiltInFn(2, Rc::new(|ctx, items| {
            let a = items[0].clone().integer_value()?;
            let b = items[1].clone().integer_value()?;

            Ok(Expression::IntegerValue(a % b))
        }
    )));

    function_map.insert(
        "sqrt".to_string(),
        Expression::BuiltInFn(1, Rc::new(|ctx, items| {
            let a = items[0].clone().integer_value()?;

            Ok(Expression::IntegerValue((a as f64).sqrt() as u64))
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

    function_map.insert(
        "str_to_int".to_string(),
        Expression::BuiltInFn(1, Rc::new(|ctx, items| {
            let a = items[0].clone().string_value()?;

            Ok(Expression::IntegerValue(a.parse()?))
        }
    )));

    function_map.insert(
        "list_len".to_string(),
        Expression::BuiltInFn(1, Rc::new(|ctx, items| {
            let len = items[0].clone().list_value()?.len();

            Ok(Expression::IntegerValue(len as u64))
        }
    )));

    function_map.insert(
        "list_nth".to_string(),
        Expression::BuiltInFn(2, Rc::new(|ctx, items| {
            let nth = items[0].clone().integer_value()?;
            let list = items[1].clone().list_value()?;

            if nth > (list.len() as u64 - 1) {
                return Err(format!(
                    "trying to access element #{} of a list only containing {} elements",
                    nth, list.len()
                ).into());
            }

            Ok(list[nth as usize].clone())
        }
    )));

    function_map.insert(
        "list_push".to_string(),
        Expression::BuiltInFn(2, Rc::new(|ctx, items| {
            let mut list = items[0].clone().list_value()?;
            let element = items[1].clone();
            
            list.push(element);

            Ok(Expression::List(list.clone()))
        }
    )));

    function_map
}
