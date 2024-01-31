use crate::scanner::Token;
use colorust::Color;

pub fn render_to_console(input: &Vec<Token>) {
    for token in input {
        print!("{}", render_token_to_console(token));
    }
    println!();
}

pub fn render_to_console_with_line_num(input: &Vec<Token>) {
    let mut num: usize = 1;
    print!("{num:-3} ");
    for token in input {
        print!("{}", render_token_to_console(token));
        if *token == Token::NewLine() {
            num += 1;
            print!("{num:-3} ");
        }
    }
    println!();
}

fn render_token_to_console(token: &Token) -> String {
    match token {
        Token::Whitespace(s) => s.into(),
        Token::NewLine() => "\n".into(),
        Token::Punctuation(s) => s.red(),
        Token::Number(s) => s.yellow(),
        Token::String(s) => s.bright_magenta(),
        Token::LineComment(s) => s.green(),
        Token::BlockComment(s) => s.bright_green(),
        Token::Name(s) => s.white(),
        Token::Keyword(s) => s.blue(),
        _ => "".into(),
    }
}
