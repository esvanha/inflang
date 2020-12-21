use crate::lexer;
use crate::ast;

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

    pub fn peek_token(&mut self) -> Result<lexer::Token, Box<dyn std::error::Error>> {
        if self.lexed_tokens.len() == 0 {
            self.lexed_tokens.push(self.lexer.next_token()?);
        }

        Ok(self.lexed_tokens.last().cloned().unwrap())
    }

    pub fn consume_token(&mut self) {
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

    pub fn parse_program(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        let mut expressions = Vec::new();

        while self.accept(lexer::TokenType::EOF)?.is_none() {
            expressions.push(self.parse_expression()?);
            self.expect(lexer::TokenType::Semicolon)?;
        }

        Ok(ast::Expression::Program(expressions))
    }
 
    pub fn parse_expression(&mut self) -> Result<ast::Expression, Box<dyn std::error::Error>> {
        match self.peek_token()? {
            lexer::Token {
                token_type: lexer::TokenType::Let,
                value: _,
            } => self.parse_let_expression(),

            lexer::Token {
                token_type: lexer::TokenType::If,
                value: _,
            } => todo!(),

            lexer::Token {
                token_type: lexer::TokenType::Fn,
                value: _,
            } => todo!(),

            lexer::Token {
                token_type: lexer::TokenType::Identifier,
                value: identifier,
            } => todo!("parse as identifier or function call"),

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
            } => todo!(),

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
        }
    }
}