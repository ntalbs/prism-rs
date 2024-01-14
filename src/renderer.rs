use crate::scanner::Token;
use colorust::Color;

pub fn render_to_console(input: &Vec<Token>) {
    for token in input {
        print!("{}", render_token_to_console(&token));
    }
    println!();
}

pub fn render_to_console_with_line_num(input: &Vec<Token>) {
    let mut num: usize = 1;
    print!("{num:03} ");
    for token in input {
        print!("{}", render_token_to_console(&token));
        if *token == Token::NewLine() {
            num += 1;
            print!("{num:03} ");
        }
    }
    println!();
}

fn render_token_to_console(token: &Token) -> String {
    match token {
        Token::Whitespace(s) => s.to_string(),
        Token::NewLine() => "\n".to_string(),
        Token::Punctuation(s) => s.red(),
        Token::Number(s) => s.yellow(),
        Token::String(s) => s.green(),
        Token::LineComment(s) => s.bright_yellow(),
        Token::BlockComment(s) => s.bright_green(),
        Token::Name(s) => s.white(),
        Token::Keyword(s) => s.bright_blue(),
        _ => "".to_string(),
    }
}
