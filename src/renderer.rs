use crate::scanner::Token;
use colorust::Color;

pub fn render_to_console(input: &Vec<Token>) -> String {
    let mut rendered = String::new();
    for token in input {
        rendered.push_str(&render_token_to_console(token));
    }
    rendered
}

pub fn render_to_console_with_line_num(input: &Vec<Token>) -> String {
    let mut rendered = String::new();
    let mut num: usize = 1;

    rendered.push_str(&format!("{num:-3} "));
    for token in input {
        rendered.push_str(&render_token_to_console(token));
        if *token == Token::NewLine() {
            num += 1;
            rendered.push_str(&format!("{num:-3} "));
        }
    }
    rendered
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
