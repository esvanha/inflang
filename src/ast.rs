
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
    Body(Vec<Expression>),
    EndOfProgram
}