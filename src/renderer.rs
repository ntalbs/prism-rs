use crate::scanner::{Line, Token};
use colorust::Color;

pub fn render_to_console(input: Vec<Line>) {
    for (linum, line) in input.iter().enumerate() {
        print!("{:03} ", linum + 1);
        for token in &line.tokens {
            print!("{}", render_token_to_console(token));
        }
        println!();
    }
}

fn render_token_to_console(token: &Token) -> String {
    match token {
        Token::Whitespace(n) => " ".repeat(*n),
        Token::Punctuation(s) => s.red(),
        Token::Number(s) => s.yellow(),
        Token::String(s) => s.green(),
        Token::Name(s) => s.white(),
        Token::Keyword(s) => s.bright_blue(),
        _ => "".to_string(),
    }
}
