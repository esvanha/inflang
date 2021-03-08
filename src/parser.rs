use crate::lexer;
use crate::ast;
use std::fs::File;
use std::io::prelude::*;

pub struct Parser {
    lexer: lexer::Lexer,
    lexed_tokens: Vec<lexer::Token>,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self {
            lexer: lexer::Lexer::new(input),
            lexed_tokens: Vec::new(),
        }
    }

    pub fn from_file(path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut program_source = String::new();
        file.read_to_string(&mut program_source)?;
        
        Ok(Self::new(program_source))
    }

    fn peek_token(&mut self) -> Result<lexer::Token, Box<dyn std::error::Error>> {
        if self.lexed_tokens.len() == 0 {
            self.lexed_tokens.push(self.lexer.next_token()?);
        }

        Ok(self.lexed_tokens.last().cloned().unwrap())
    }

    fn consume_token(&mut self) {
        self.lexed_tokens.pop();
    }

    fn expect(&mut self, expected_type: lexer::TokenType) -> Result<lexer::Token, Box<dyn std::error::Error>> {
        let tok = self.peek_token()?;

        if tok.token_type == expected_type {
            self.consume_token();
            Ok(tok)
        } else {
            Err(format!(
                "expected token of type {}, got `{}` instead",
                expected_type, tok
            )
            .into())
        }
    }

    fn accept(&mut self, token_type: lexer::TokenType) -> Result<Option<lexer::Token>, Box<dyn std::error::Error>> {
        let tok = self.peek_token()?;

        Ok(if tok.token_type == token_type {
            self.consume_token();
            Some(tok)
        } else {
            None
        })
    }

    fn parse_let_expression(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        //.. let <identifier> = <expression>

        self.expect(lexer::TokenType::Let)?;

        let variable_name = self.expect(lexer::TokenType::Identifier)?.value;

        self.expect(lexer::TokenType::AssignmentOperator)?;

        let expression = self.parse_expression()?;

        Ok(ast::Expression::LetBinding(variable_name, Box::new(expression)))
    }

    fn parse_block(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        //.. { <one or more expressions, separated by `;`> }

        self.expect(lexer::TokenType::LCurlyBrace)?;

        let mut expressions = Vec::new();

        while self.accept(lexer::TokenType::RCurlyBrace)?.is_none() {
            expressions.push(self.parse_expression()?);
            self.expect(lexer::TokenType::Semicolon)?;
        }

        if expressions.is_empty() {
            return Err("expected at least one expression in block, got none".into());
        }

        Ok(ast::Expression::Block(expressions))
    }

    fn parse_if_expression(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        //.. if <condition> <block> else <block>

        self.expect(lexer::TokenType::If)?;

        let condition = self.parse_expression()?;

        let when_true_block = self.parse_block()?;

        self.expect(lexer::TokenType::Else)?;

        let when_false_block = self.parse_block()?;

        Ok(ast::Expression::IfExpression(
            Box::new(condition),
            Box::new(when_true_block),
            Box::new(when_false_block),
        ))
    }

    fn parse_while(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        //.. while <condition> <block>

        self.expect(lexer::TokenType::While)?;

        let condition = self.parse_expression()?;

        let body = self.parse_block()?;

        Ok(ast::Expression::While(Box::new(condition), Box::new(body)))
    }

    fn parse_fn_declaration(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        //.. fn (<argument names, separated by `,`>) <block>
        
        self.expect(lexer::TokenType::Fn)?;

        self.expect(lexer::TokenType::LParen)?;

        let mut argument_names = Vec::new();
        let mut was_separated = true;

        while self.accept(lexer::TokenType::RParen)?.is_none() {
            if !was_separated {
                return Err("unseparated argument name in fn declaration".into());
            }

            let argument_name = self.expect(lexer::TokenType::Identifier)?;
            argument_names.push(argument_name.value);
            
            was_separated = self.accept(lexer::TokenType::Comma)?.is_some();
        }

        let fn_body = self.parse_block()?;

        if argument_names.len() == 0 {
            return Ok(ast::Expression::Fn(None, Box::new(fn_body)));
        }

        //.. Functions are composed of nested unary functions. While looping
        //   through the argument names, `last_fn` is the outer function every
        //   time, which will then become the inner function.
        let mut last_fn = ast::Expression::Fn(
            Some(argument_names.pop().unwrap()),
            Box::new(fn_body),
        );
        argument_names.reverse();

        for argument_name in argument_names {
            last_fn = ast::Expression::Fn(
                Some(argument_name),
                Box::new(last_fn),
            );
        }

        Ok(last_fn)
    }

    fn parse_fn_call(&mut self, expr: ast::Expression) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        //.. <expr>(<arguments, separated by `,`>)
        //.. This gets turned into a nested FnCall, e.g.: 
        //   FnCall(FnCall(<expr>, <argument>), <argument>)

        self.expect(lexer::TokenType::LParen)?;

        let mut was_separated = true;
        let mut fn_call = ast::Expression::Null;

        while self.accept(lexer::TokenType::RParen)?.is_none() {
            if !was_separated {
                return Err("unseparated argument name in fn call".into());
            }

            if fn_call.is_null() {
                fn_call = ast::Expression::FnCall(
                    Box::new(expr.clone()), Box::new(Some(self.parse_expression()?))
                );
            } else {
                fn_call = ast::Expression::FnCall(
                    Box::new(fn_call), Box::new(Some(self.parse_expression()?))
                );
            }
            
            was_separated = self.accept(lexer::TokenType::Comma)?.is_some();
        }
        
        //.. If there were no arguments between the parentheses, fn_call will still
        //   be Null.
        if fn_call.is_null() {
            fn_call = ast::Expression::FnCall(
                Box::new(expr.clone()), Box::new(None),
            );
        }

        Ok(fn_call)
    }

    fn parse_list(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        //.. [ <items, separated by `,`> ]
        
        self.expect(lexer::TokenType::LSquareBracket)?;

        let mut items = Vec::new();
        let mut was_separated = true;

        while self.accept(lexer::TokenType::RSquareBracket)?.is_none() {
            if !was_separated {
                return Err("unseparated item in list".into());
            }

            items.push(self.parse_expression()?);
            
            was_separated = self.accept(lexer::TokenType::Comma)?.is_some();
        }

        Ok(ast::Expression::List(items))
    }

    pub fn parse_program(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        let mut expressions = Vec::new();

        while self.accept(lexer::TokenType::EOF)?.is_none() {
            expressions.push(self.parse_expression()?);
            self.expect(lexer::TokenType::Semicolon)?;
        }

        Ok(ast::Expression::Program(expressions))
    }
 
    pub fn parse_expression(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        let mut expr = match self.peek_token()? {
            lexer::Token {
                token_type: lexer::TokenType::Let,
                value: _,
            } => self.parse_let_expression(),

            lexer::Token {
                token_type: lexer::TokenType::If,
                value: _,
            } => self.parse_if_expression(),

            lexer::Token {
                token_type: lexer::TokenType::Fn,
                value: _,
            } => self.parse_fn_declaration(),

            lexer::Token {
                token_type: lexer::TokenType::Identifier,
                value: identifier,
            } => {
                self.consume_token();
                Ok(ast::Expression::Identifier(identifier))
            },

            lexer::Token {
                token_type: lexer::TokenType::While,
                value: _,
            } => self.parse_while(),

            lexer::Token {
                token_type: lexer::TokenType::Integer,
                value: integer,
            } => {
                self.consume_token();
                Ok(ast::Expression::IntegerValue(integer.parse()?))
            },

            lexer::Token {
                token_type: lexer::TokenType::StringLiteral,
                value: string,
            } => {
                self.consume_token();
                Ok(ast::Expression::StringValue(string))
            }

            lexer::Token {
                token_type: lexer::TokenType::LSquareBracket,
                value: _,
            } => self.parse_list(),

            lexer::Token {
                token_type: lexer::TokenType::True,
                value: _,
            } => {
                self.consume_token();
                Ok(ast::Expression::BooleanValue(true))
            },

            lexer::Token {
                token_type: lexer::TokenType::False,
                value: _,
            } => {
                self.consume_token();
                Ok(ast::Expression::BooleanValue(false))
            }

            lexer::Token {
                token_type: lexer::TokenType::EOF,
                value: _,
            } => Ok(ast::Expression::EndOfProgram),

            misc_token => {
                return Err(format!(
                    "unexpected `{}`; no valid expression starts with this",
                    misc_token
                ).into());
            }
        }?;

        //.. A function call can return a function so an expression may contain
        //   multiple function calls after each other, e.g.:
        //   "fn (x, y) { +(x, y); }(2)(3)"
        while self.peek_token()?.token_type == lexer::TokenType::LParen {
            expr = self.parse_fn_call(expr)?;
        }

        Ok(expr)
    }
}