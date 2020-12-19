mod lexer;
use crate::lexer::{Lexer, TokenType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new("let x = y; test(\"Hello, world!\", abc, 23);".to_string());

    loop {
        let token = lexer.next_token()?;
        if token.token_type == TokenType::EOF {
            break;
        }

        println!("{:#?}", token);
    }

    Ok(())
}
