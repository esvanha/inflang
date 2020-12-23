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

    pub fn evaluate(self, ctx: Rc<RefCell<EvaluationScope>>) -> Result<Expression, Box<dyn std::error::Error>> {
        Ok(match &self {
            Self::BooleanValue(_) => self,
            Self::IntegerValue(_) => self,
            Self::StringValue(_) => self,
            Self::Null => self,

            Self::Identifier(identifier) => {
                match ctx.borrow().variables.get(identifier) {
                    Some(value) => value.clone(),
                    None => return Err(format!("unknown identifier `{}`", identifier).into()),
                }
            },

            Self::Block(expressions) => {
                let mut return_value = Self::Null;

                for expression in expressions.iter() {
                    return_value = expression.clone().evaluate(ctx.clone())?;
                }

                return_value
            },

            Self::IfExpression(condition, if_block, else_block) => {
                if condition.clone().evaluate(ctx.clone())?.boolean_value()? {
                    if_block.clone().evaluate(ctx.clone())?
                } else {
                    else_block.clone().evaluate(ctx.clone())?
                }
            },

            Self::List(expressions) => {
                let mut result_list = Vec::new();

                for expression in expressions {
                    result_list.push(expression.clone().evaluate(ctx.clone())?);
                }

                Expression::List(result_list)
            }
            
            _ => todo!()
        })
    }
}