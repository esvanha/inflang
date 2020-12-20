mod lexer;
use crate::lexer::{Lexer, TokenType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new("let id = fn (x) { x }; if id(true) { print(\"true == true\"); } else { print(\"?\"); }".to_string());

    loop {
        let token = lexer.next_token()?;
        if token.token_type == TokenType::EOF {
            break;
        }

        print!("{} ", token.token_type);
    }
    print!("\n");

    Ok(())
}
