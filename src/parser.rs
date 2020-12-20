use crate::lexer;
use crate::ast::Expression;

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
            Ok(tok)
        } else {
            Err(format!(
                "expected token of type {}, got `{}` of type {} instead",
                expected_type, tok.value, tok.token_type
            )
            .into())
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, Box<dyn std::error::Error>> {
        match self.peek_token()? {
            lexer::Token {
                token_type: lexer::TokenType::Let,
                value: _,
            } => todo!(),
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
            } => todo!(),
            lexer::Token {
                token_type: lexer::TokenType::StringLiteral,
                value: string,
            } => todo!(),
            lexer::Token {
                token_type: lexer::TokenType::LSquareBracket,
                value: _,
            } => todo!(),
            lexer::Token {
                token_type: lexer::TokenType::EOF,
                value: _,
            } => todo!(),
            misc_token => {
                return Err(format!(
                    "unexpected `{}`; no valid expression starts with this",
                    misc_token
                ).into());
            }
        };
    }
}