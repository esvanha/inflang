use std::cmp::{max, min};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    AssignmentOperator, //.. =
    Comma,              //.. ,
    Else,               //.. else
    EOF,
    False,              //.. false
    Fn,                 //.. fn
    Identifier,         //.. x
    If,                 //.. if
    Integer,            //.. 0-9
    LCurlyBrace,        //.. {
    Let,                //.. let
    LParen,             //.. (
    LSquareBracket,     //.. [
    RCurlyBrace,        //.. }
    RParen,             //.. )
    RSquareBracket,     //.. ]
    Semicolon,          //.. ;
    StringLiteral,      //.. "*"
    True,               //.. true
    While,              //.. while
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let token_type_as_str = match *self {
            TokenType::AssignmentOperator   => "=",
            TokenType::Comma                => ",",
            TokenType::Else                 => "else",
            TokenType::EOF                  => "end of file",
            TokenType::False                => "false",
            TokenType::Fn                   => "fn",
            TokenType::Identifier           => "identifier",
            TokenType::If                   => "if",
            TokenType::Integer              => "integer",
            TokenType::LCurlyBrace          => "{",
            TokenType::Let                  => "let",
            TokenType::LParen               => "(",
            TokenType::LSquareBracket       => "[",
            TokenType::RCurlyBrace          => "}",
            TokenType::RParen               => ")",
            TokenType::RSquareBracket       => "]",
            TokenType::Semicolon            => ";",
            TokenType::StringLiteral        => "string literal",
            TokenType::True                 => "true",
            TokenType::While                => "while"
        };

        write!(f, "{}", token_type_as_str)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let token_as_str = match self {
            Token {
                token_type: TokenType::StringLiteral,
                value: string
            } => format!("\"{}\"", string),
            Token {
                token_type: TokenType::Identifier,
                value: identifier
            } => identifier.clone(),
            Token {
                token_type: TokenType::Integer,
                value: integer,
            } => integer.clone(),
            Token {
                token_type: t_type,
                value: _,
            } => format!("{}", t_type),
        };

        write!(f, "{}", token_as_str)
    }
}

#[derive(Debug)]
pub struct Lexer {
    source: String,
    current_index: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source,
            current_index: 0,
        }
    }

    fn consume(&mut self, n: usize) {
        self.current_index = min(self.current_index + n, self.source.len());
    }

    fn peek(&mut self, n: usize) -> Vec<char> {
        self.source
            .chars()
            .skip(self.current_index)
            .take(n)
            .collect()
    }

    fn peek_one(&mut self) -> Option<char> {
        self.peek(1).first().cloned()
    }

    //.. todo: return iterator(?)
    fn take(&mut self, n: usize) -> Vec<char> {
        let return_value = self.peek(n);
        self.consume(n);
        return_value
    }

    fn take_one(&mut self) -> Option<char> {
        self.take(1).first().cloned()
    }

    fn rewind(&mut self, n: usize) {
        self.current_index = max(0, self.current_index - n);
    }

    fn skip_whitespace(&mut self) {
        loop {
            let ch = match self.take_one() {
                Some(ch) => ch,
                None => return,
            };

            if !ch.is_whitespace() {
                break;
            }
        }

        self.rewind(1);
    }

    fn expect_ch(&mut self, expected_ch: char) -> Result<(), Box<dyn std::error::Error>> {
        match self.take_one() {
            Some(ch) => {
                if ch == expected_ch {
                    Ok(())
                } else {
                    Err(format!("expected `{}`, got `{}`", expected_ch, ch).into())
                }
            }
            None => Err("no remaining characters".into()),
        }
    }

    fn expect(&mut self, expected_str: String) -> Result<(), Box<dyn std::error::Error>> {
        let actual = self
            .take(expected_str.len())
            .into_iter()
            .collect::<String>();

        if actual == expected_str {
            Ok(())
        } else {
            Err(format!("expected `{}`, got `{}` instead", expected_str, actual).into())
        }
    }

    //.. this function takes strings which could either be keywords or identifiers
    fn keyword_or_identifier(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::new();

        loop {
            let ch = match self.take_one() {
                Some(ch) => ch,
                None => break,
            };

            if ch.is_whitespace()
                || ch == '('
                || ch == ')'
                || ch == ';'
                || ch == ','
                || ch == '['
                || ch == ']'
                || ch == '}'
                || ch == '{'
            {
                self.rewind(1);
                break;
            }

            if !(ch == '+'
                || ch == '='
                || ch == '*'
                || ch == '/'
                || ch == '-'
                || ch == '>'
                || ch == '<'
                || ch == '_'
                || ch.is_ascii_alphanumeric())
            {
                return Err(format!(
                    "`{}`: unexpected `{}` while scanning identifier/keyword",
                    result, ch
                )
                .into());
            }

            result.push(ch);
        }

        Ok(result)
    }

    fn take_keyword_or_identifier(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        let keyword_or_identifier = self.keyword_or_identifier()?;

        Ok(match &keyword_or_identifier[..] {
            "let"   => Token { token_type: TokenType::Let, value: String::new() },
            "if"    => Token { token_type: TokenType::If, value: String::new() },
            "else"  => Token { token_type: TokenType::Else, value: String::new() },
            "true"  => Token { token_type: TokenType::True, value: String::new() },
            "false" => Token { token_type: TokenType::False, value: String::new() },
            "fn"    => Token { token_type: TokenType::Fn, value: String::new() },
            "while" => Token { token_type: TokenType::While, value: String::new() },
            _       => Token { token_type: TokenType::Identifier, value: keyword_or_identifier }
        })
    }

    fn take_integer(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        let mut result = String::new();

        loop {
            let ch = match self.take_one() {
                Some(ch) => ch,
                None => break,
            };

            if ch.is_whitespace()
                || ch == '('
                || ch == ')'
                || ch == ';'
                || ch == ','
                || ch == '['
                || ch == ']'
                || ch == '}'
                || ch == '{'
            {
                self.rewind(1);
                break;
            }

            if !ch.is_digit(10) {
                return Err(format!(
                    "`{}`: unexpected `{}` while scanning integer",
                    result, ch
                )
                .into());
            }

            result.push(ch);
        }

        Ok(Token{ token_type: TokenType::Integer, value: result })
    }

    fn take_string_literal(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        self.expect_ch('"')?;

        let mut string_literal = String::new();

        loop {
            let ch = match self.take_one() {
                Some(ch) => ch,
                None => {
                    return Err(format!(
                        "`\"{}`: unterminated string literal, got EOF",
                        string_literal
                    ).into());
                }
            };

            if ch == '"' {
                break;
            }

            string_literal.push(ch);
        }

        Ok(Token{ token_type: TokenType::StringLiteral, value: string_literal })
    }


    pub fn next_token(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        let consume_and_return = |this: &mut Lexer, t_type| -> Result<Token, Box<dyn std::error::Error>> {
            this.consume(1);
            Ok(Token {
                token_type: t_type,
                value: String::new(),
            })
        };

        self.skip_whitespace();

        match self.peek_one() {
            Some('%')       => {
                loop {
                    let next_char = self.take_one();
                    if next_char == Some('\n') || next_char == None {
                        break
                    }
                }
                self.next_token()
            },
            Some('(')       => consume_and_return(self, TokenType::LParen),
            Some(')')       => consume_and_return(self, TokenType::RParen),
            Some(';')       => consume_and_return(self, TokenType::Semicolon),
            Some('=')       => consume_and_return(self, TokenType::AssignmentOperator),
            Some(',')       => consume_and_return(self, TokenType::Comma),
            Some('{')       => consume_and_return(self, TokenType::LCurlyBrace),
            Some('}')       => consume_and_return(self, TokenType::RCurlyBrace),
            Some('[')       => consume_and_return(self, TokenType::LSquareBracket),
            Some(']')       => consume_and_return(self, TokenType::RSquareBracket),
            Some('0'..='9') => self.take_integer(),
            Some('"')       => self.take_string_literal(),
            Some(_)         => self.take_keyword_or_identifier(),
            None            => Ok(
                Token {
                    token_type: TokenType::EOF,
                    value: String::new(),
                }
            ),
        }
    }
}
