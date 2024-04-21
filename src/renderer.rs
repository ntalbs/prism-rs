use crate::scanner::Token;
use colorust::Color;

pub fn render_to_console(input: &[Token]) -> String {
    let mut rendered = String::new();
    for token in input {
        rendered.push_str(&render_token_to_console(token));
    }
    rendered
}

pub fn render_to_console_with_line_num(input: &[Token]) -> String {
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

pub fn render_to_html(input: &[Token]) -> String {
    let mut rendered = String::new();
    rendered
        .push_str("<div class=\"highlight\"><pre class=\"prism\"><code class=\"language-rust\">");

    for line in input.split(|t| *t == Token::NewLine()) {
        rendered.push_str("<span class=\"line\">");
        for token in line {
            rendered.push_str(&render_token_to_html(token));
        }
        rendered.push_str("</span>\n")
    }

    rendered.push_str("</code></pre></div>");
    rendered
}

pub fn render_to_html_with_line_num(input: &[Token]) -> String {
    let mut rendered = String::new();
    rendered.push_str("<div class=\"highlight\">");

    rendered.push_str("<pre class=\"gutter\">");

    let linum = input.iter().filter(|t| **t == Token::NewLine()).count();
    for i in 1..=linum {
        rendered.push_str(&format!("{i:-3}\n"));
    }

    rendered.push_str("</pre>");

    rendered.push_str("<pre class=\"prism\"><code class=\"language-rust\">");

    for line in input.split(|t| *t == Token::NewLine()) {
        rendered.push_str("<span class=\"line\">");
        for token in line {
            rendered.push_str(&render_token_to_html(token));
        }
        rendered.push_str("</span>\n")
    }

    rendered.push_str("</code></pre></div>");
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

fn render_token_to_html(token: &Token) -> String {
    match token {
        Token::Whitespace(s) => format!("<span class=\"ws\">{s}</span>"),
        Token::Punctuation(s) => format!("<span class=\"pt\">{s}</span>"),
        Token::Number(s) => format!("<span class=\"nu\">{s}</span>"),
        Token::String(s) => format!("<span class=\"st\">{s}</span>"),
        Token::LineComment(s) => format!("<span class=\"lc\">{s}</span>"),
        Token::BlockComment(s) => format!("<span class=\"bc\">{s}</span>"),
        Token::Name(s) => format!("<span class=\"nm\">{s}</span>"),
        Token::Keyword(s) => format!("<span class=\"kw\">{s}</span>"),
        _ => "".into(),
    }
}
