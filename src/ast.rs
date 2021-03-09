use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::builtin_functions;

type BuiltInFnClosure = Rc<
    dyn Fn(
        SharedContext, Vec<Expression>
    ) -> Result<Expression, Box<dyn std::error::Error>>
>;

#[derive(Clone)]
pub enum Expression {
    List(Vec<Expression>),
    IntegerValue(u64),
    StringValue(String),
    BooleanValue(bool),
    Identifier(String),
    //.. IfExpression: condition, expression when true, expression when false
    IfExpression(Box<Expression>, Box<Expression>, Box<Expression>),
    //.. While: condition, body
    While(Box<Expression>, Box<Expression>),
    //.. Fn: argument name (if given), function body
    Fn(Option<String>, Box<Expression>),
    //.. LetBinding: variable name, value
    LetBinding(String, Box<Expression>),
    //.. FnCall: function, argument (if given)
    FnCall(Box<Expression>, Box<Option<Expression>>),
    Block(Vec<Expression>),
    Program(Vec<Expression>),
    //.. BuiltInFn: argument length, function
    BuiltInFn(u8, BuiltInFnClosure),
    Null,
    EndOfProgram
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a == b,
            (Self::StringValue(a), Self::StringValue(b)) => a == b,
            (Self::IntegerValue(a), Self::IntegerValue(b)) => a == b,
            (Self::BooleanValue(a), Self::BooleanValue(b)) => a == b,
            (Self::Identifier(a), Self::Identifier(b)) => a == b,
            (Self::Null, Self::Null) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let expr_as_str = match self {
            Self::List(expressions) => {
                format!(
                    "[{}]",
                    expressions
                        .iter()
                        .map(|expression| expression.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            },
            Self::IntegerValue(n) => n.to_string(),
            Self::Program(expressions) => {
                format!(
                    "{}",
                    expressions
                        .iter()
                        .map(|expression| expression.to_string() + ";")
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            },
            Self::IfExpression(condition, if_true, if_false) => {
                format!(
                    "if {} {} else {}",
                    condition, if_true, if_false,
                )
            },
            Self::Block(expressions) => {
                format!(
                    "{{ {} }}",
                    expressions
                        .iter()
                        .map(|expression| expression.to_string() + ";")
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            Self::StringValue(value) => format!("\"{}\"", value),
            Self::BooleanValue(value) => value.to_string(),
            Self::Identifier(name) => name.clone(),
            Self::Fn(argument_name, body) => {
                format!(
                    "fn ({}) {}",
                    argument_name.clone().unwrap_or_default(),
                    body,
                )
            },
            Self::LetBinding(variable, value) => {
                format!("let {} = {}", variable, value)
            },
            Self::While(condition, body) => {
                format!("while {} {}", condition, body)
            },
            Self::FnCall(name, argument) => {
                format!(
                    "({}({}))",
                    name,
                    argument.clone().map(|name| name.to_string()).unwrap_or_default()
                )
            },
            Self::Null => "<null>".to_string(),
            Self::EndOfProgram => "<end of program>".to_string(),
            Self::BuiltInFn(_, _) => "<built-in function>".to_string()
        };

        write!(f, "{}", expr_as_str)
    }
}

struct EvaluationScope {
    variables: HashMap<String, Expression>,
}

impl EvaluationScope {
    pub fn new_default() -> Self {
        Self {
            variables: builtin_functions::builtin_functions(),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

pub struct EvaluationContext {
    scopes: Vec<EvaluationScope>,
}

impl EvaluationContext {
    pub fn new() -> Self {
        Self {
            scopes: vec![EvaluationScope::new_default()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(EvaluationScope::new_empty());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn resolve_var(&self, var_name: String) -> Option<Expression> {
        for scope in self.scopes.iter().rev() {
            match scope.variables.get(&var_name) {
                Some(var) => return Some(var.clone()),
                None => (),
            };
        }

        None
    }

    fn outer_scope(&mut self) -> &mut EvaluationScope {
        self.scopes.iter_mut().rev().nth(1).unwrap()
    }

    pub fn add_local_var(&mut self, var_name: String, value: Expression) {
        self.outer_scope().variables.insert(var_name, value);
    }
}

type SharedContext = Rc<RefCell<EvaluationContext>>;

impl Expression {
    pub fn integer_value(&self) -> Result<u64, Box<dyn std::error::Error>> {
        match &self {
            Self::IntegerValue(n) => Ok(*n),
            _ => Err(format!("expected integer, got `{}`", self).into()),
        }
    }

    pub fn string_value(&self) -> Result<String, Box<dyn std::error::Error>> {
        match &self {
            Self::StringValue(n) => Ok(n.clone()),
            _ => Err(format!("expected string, got `{}`", self).into()),
        }
    }

    pub fn boolean_value(&self) -> Result<bool, Box<dyn std::error::Error>> {
        match &self {
            Self::BooleanValue(n) => Ok(*n),
            _ => Err(format!("expected boolean, got `{}`", self).into()),
        }
    }

    pub fn identifier_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        match &self {
            Self::Identifier(name) => Ok(name.clone()),
            _ => Err(format!("expected identifier, got `{}`", self).into()),
        }
    }

    pub fn is_null(&self) -> bool {
        match &self {
            Self::Null => true,
            _ => false
        }
    }

    fn evaluate_identifier(&self, ctx: SharedContext, identifier: String) -> Result<Expression, Box<dyn std::error::Error>> {
        match ctx.borrow().resolve_var(identifier.clone()) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("unknown identifier `{}`", identifier.clone()).into()),
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
                
        ctx.borrow_mut().add_local_var(
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

    fn evaluate_fn_call(&self, ctx: SharedContext, function: Box<Expression>, argument_value_opt: &Option<Expression>) -> Result<Expression, Box<dyn std::error::Error>> {
        let function_name = function.identifier_name().unwrap_or("<anonymous>".to_string());

        match function.clone().evaluate(ctx.clone())? {
            Self::Fn(argument_name, body) => {
                if argument_value_opt.is_none() != argument_name.is_none() {
                    return Err(format!(
                        "function `{}` expected {} arguments, got {} instead",
                        function_name,
                        if argument_name.is_some() { 1 } else { 0 },
                        if argument_value_opt.is_some() { 1 } else { 0 },
                    ).into())
                }

                if let Some(argument_value) = argument_value_opt {
                    let value = argument_value.clone().evaluate(ctx.clone())?.clone();

                    ctx.borrow_mut().add_local_var(
                        argument_name.unwrap().clone(), value
                    );
                }

                body.clone().evaluate(ctx.clone())
            },

            Self::BuiltInFn(argument_length, closure_fn) => {
                if argument_length != 0 && argument_value_opt.is_none() {
                    return Err(format!(
                        "function `{}` expected {} arguments, got {} instead",
                        function_name,
                        argument_length,
                        if argument_value_opt.is_some() { 1 } else { 0 },
                    ).into());
                }

                //.. In the case of a built-in function accepting multiple
                //   arguments, each time an argument is applied, a
                //   BuiltInFunction is returned with a new closure, containing
                //   the applied argument. This is the necessary because all
                //   functions are unary functions, which allows for the
                //   support of partial-application.
                if argument_length > 1 {
                    let argument_value = argument_value_opt.clone().unwrap();
                    return Ok(Self::BuiltInFn(argument_length - 1, Rc::new(move |ctx, mut items| {
                        items.insert(0, argument_value.clone().evaluate(ctx.clone())?);

                        closure_fn(ctx.clone(), items)
                    })));
                }

                //.. If there are one or zero arguments applied, call the
                //   closure function.
                if let Some(argument_value) = argument_value_opt {
                    closure_fn(
                        ctx.clone(),
                        vec![argument_value.clone().evaluate(ctx.clone())?],
                    )
                } else {
                    closure_fn(ctx.clone(), Vec::new())
                }
            }

            other => {
                return Err(format!("trying to call `{}`, which is not a function", other).into());
            },
        }
    }

    pub fn evaluate_while(&self, ctx: SharedContext, condition: &Expression, body: &Expression) -> Result<Expression, Box<dyn std::error::Error>> {
        let mut result = Expression::Null;

        while condition.clone().evaluate(ctx.clone())?.boolean_value()? {
            result = body.clone().evaluate(ctx.clone())?;
        }

        Ok(result)
    }

    pub fn evaluate(self, ctx: SharedContext) -> Result<Expression, Box<dyn std::error::Error>> {
        ctx.borrow_mut().enter_scope();
        
        
        let result = match &self {
            Self::BooleanValue(_) => self,
            Self::IntegerValue(_) => self,
            Self::StringValue(_) => self,
            Self::Null => self,
            Self::EndOfProgram => self,
            Self::Fn(_, _) => self,
            Self::BuiltInFn(_, _) => self,

            Self::Identifier(identifier) => {
                self.evaluate_identifier(ctx.clone(), identifier.clone())?
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
            Self::FnCall(function_name, argument_value) => {
                self.evaluate_fn_call(ctx.clone(), function_name.clone(), argument_value)?
            },
            Self::While(condition, body) => {
                self.evaluate_while(ctx.clone(), condition, body)?
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
        };

        ctx.borrow_mut().exit_scope();

        Ok(result)
    }
}