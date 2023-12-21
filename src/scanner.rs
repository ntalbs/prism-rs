use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum Token {
    WhiteSpace(usize),
    Text(String),
    Eol,
    Eof,
}

#[derive(Debug)]
pub(crate) struct Line {
    tokens: Vec<Token>,
}

impl Line {
    fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

pub(crate) struct Scanner<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars().peekable(),
        }
    }

    pub(crate) fn scan(&mut self) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        let mut line = Line::new();

        while !self.is_at_end() {
            let token = self.scan_token();
            match token {
                Token::Eof => return lines,
                Token::Eol => {
                    lines.push(line);
                    line = Line::new();
                }
                _ => line.add_token(token),
            }
        }
        lines
    }

    fn scan_token(&mut self) -> Token {
        let o = self.peek();
        let c = match o {
            None => return Token::Eof,
            Some(&'\n') => {
                self.advance();
                return Token::Eol;
            }
            Some(c) => c,
        };
        match c {
            ' ' => self.whitespace(),
            _ => self.text(),
        }
    }

    fn whitespace(&mut self) -> Token {
        let mut n: usize = 0;
        while let Some(' ') = self.peek() {
            n += 1;
            self.advance();
        }
        Token::WhiteSpace(n)
    }

    fn text(&mut self) -> Token {
        let mut buf = String::new();
        while let Some(c) = self.peek() {
            if *c == ' ' || *c == '\n' {
                break;
            }
            buf.push(*c);
            self.advance();
        }
        Token::Text(buf)
    }

    fn advance(&mut self) -> Option<char> {
        self.iter.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}
