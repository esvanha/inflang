mod lexer;
use crate::lexer::{Lexer, TokenType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new("let x = y; print(x); 234234; 42".to_string());

    loop {
        let token = lexer.next_token()?;
        if token.token_type == TokenType::EOF {
            break;
        }

        println!("{:#?}", token);
    }

    Ok(())
}
