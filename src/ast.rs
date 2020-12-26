use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum Expression {
    List(Vec<Expression>),
    IntegerValue(u64),
    StringValue(String),
    BooleanValue(bool),
    Identifier(String),
    //.. IfExpression: condition, expression when true, expression when false
    IfExpression(Box<Expression>, Box<Expression>, Box<Expression>),
    //.. Fn: argument names, function body
    Fn(Vec<Expression>, Box<Expression>),
    //.. LetBinding: variable name, value
    LetBinding(String, Box<Expression>),
    //.. FnCall: function, arguments
    FnCall(Box<Expression>, Vec<Expression>),
    Block(Vec<Expression>),
    Program(Vec<Expression>),
    Null,
    EndOfProgram
}

pub struct EvaluationScope {
    variables: HashMap<String, Expression>,
}

impl EvaluationScope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

type SharedContext = Rc<RefCell<EvaluationScope>>;

impl Expression {
    pub fn integer_value(&self) -> Result<u64, Box<dyn std::error::Error>> {
        match &self {
            Self::IntegerValue(n) => Ok(*n),
            //.. todo: add Display trait to Expression
            _ => Err(format!("expected integer, got `{:#?}`", self).into()),
        }
    }

    pub fn string_value(&self) -> Result<String, Box<dyn std::error::Error>> {
        match &self {
            Self::StringValue(n) => Ok(n.clone()),
            _ => Err(format!("expected string, got `{:#?}`", self).into()),
        }
    }

    pub fn boolean_value(&self) -> Result<bool, Box<dyn std::error::Error>> {
        match &self {
            Self::BooleanValue(n) => Ok(*n),
            _ => Err(format!("expected boolean, got `{:#?}`", self).into()),
        }
    }

    pub fn identifier_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        match &self {
            Self::Identifier(name) => Ok(name.clone()),
            _ => Err(format!("expected identifier, got `{:#?}`", self).into()),
        }
    }

    fn evaluate_identifier(&self, ctx: SharedContext, identifier: &String) -> Result<Expression, Box<dyn std::error::Error>> {
        match ctx.borrow().variables.get(identifier) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("unknown identifier `{}`", identifier).into()),
        }
    }

    fn evaluate_block(&self, ctx: SharedContext, block_body: &Vec<Expression>) -> Result<Expression, Box<dyn std::error::Error>> {
        let mut return_value = Self::Null;

        for expression in block_body.iter() {
            return_value = expression.clone().evaluate(ctx.clone())?;
        }

        Ok(return_value)
    }

    fn evaluate_let_binding(&self, ctx: SharedContext, variable_name: &String, value: &Expression) -> Result<Expression, Box<dyn std::error::Error>> {
        let evaluated_value = value.clone().evaluate(ctx.clone())?;
                
        ctx.borrow_mut().variables.insert(
            variable_name.clone(),
            evaluated_value.clone()
        );

        Ok(evaluated_value)
    }

    fn evaluate_list(&self, ctx: SharedContext, expressions: &Vec<Expression>) -> Result<Expression, Box<dyn std::error::Error>> {
        let mut result_list = Vec::new();

        for expression in expressions {
            result_list.push(expression.clone().evaluate(ctx.clone())?);
        }

        Ok(Expression::List(result_list))
    }

    fn evaluate_fn_call(&self, ctx: SharedContext, name: &Expression, argument_values: &Vec<Expression>) -> Result<Expression, Box<dyn std::error::Error>> {
        let variables = ctx.borrow_mut().variables.clone();
        let function_name_str = name.clone().identifier_name()?;
        let function = variables.get(&function_name_str);

        match function {
            Some(Self::Fn(argument_names, body)) => {
                if argument_values.len() != argument_names.len() {
                    return Err(format!(
                        "function `{}` expected {} arguments, got {} instead",
                        function_name_str, argument_names.len(), argument_values.len(),
                    ).into())
                }

                for i in 0..argument_names.len() {
                    let name = argument_names[i].identifier_name()?;
                    let value = argument_values[i].clone().evaluate(ctx.clone())?.clone();

                    ctx.borrow_mut().variables.insert(name, value);
                }

                body.clone().evaluate(ctx.clone())
            },
            Some(other) => {
                return Err(format!("trying to call `{:#?}`, which is not a function", other).into());
            },
            None => {
                return Err(format!("unknown identifier `{}`", function_name_str).into());
            }
        }
    }

    pub fn evaluate(self, ctx: SharedContext) -> Result<Expression, Box<dyn std::error::Error>> {
        Ok(match &self {
            Self::BooleanValue(_) => self,
            Self::IntegerValue(_) => self,
            Self::StringValue(_) => self,
            Self::Null => self,
            Self::EndOfProgram => self,
            Self::Fn(_, _) => self,

            Self::Identifier(identifier) => {
                self.evaluate_identifier(ctx.clone(), identifier)?
            },
            Self::Block(expressions) => {
                self.evaluate_block(ctx.clone(), expressions)?
            },
            Self::List(expressions) => {
                self.evaluate_list(ctx.clone(), expressions)?
            },
            Self::LetBinding(variable_name, value) => {
                self.evaluate_let_binding(ctx.clone(), variable_name, value)?
            },
            Self::FnCall(function_name, argument_values) => {
                self.evaluate_fn_call(ctx.clone(), function_name, argument_values)?
            },

            Self::IfExpression(condition, if_block, else_block) => {
                if condition.clone().evaluate(ctx.clone())?.boolean_value()? {
                    if_block.clone().evaluate(ctx.clone())?
                } else {
                    else_block.clone().evaluate(ctx.clone())?
                }
            },
            
            Self::Program(expressions) => {
                for expression in expressions.iter() {
                    expression.clone().evaluate(ctx.clone())?;
                }

                Expression::EndOfProgram
            },
        })
    }
}