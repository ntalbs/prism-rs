use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum Token {
    WhiteSpace(usize),
    Punctuation(String),
    Number(String),
    String(String),
    Name(String),
    Keyword(String),
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
                Token::Eof => break,
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
        let c = match self.peek() {
            None => return Token::Eof,
            Some(&'\n') => {
                self.advance();
                return Token::Eol;
            }
            Some(c) => *c,
        };
        match c {
            ' ' => self.whitespace(),
            c if Self::is_punctuation(c) => self.punctuation(),
            c if c.is_digit(10) => self.number(),
            '"' => self.string(),
            _ => self.name(),
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

    fn punctuation(&mut self) -> Token {
        let mut buf = String::new();
        while let Some(&c) = self.peek() {
            if !Self::is_punctuation(c) {
                break;
            }
            buf.push(c);
            self.advance();
        }
        Token::Punctuation(buf)
    }

    fn number(&mut self) -> Token {
        let mut buf = String::new();
        while let Some(&c) = self.peek() {
            if !c.is_digit(10) && c != '.' {
                break;
            }
            buf.push(c);
            self.advance();
        }
        Token::Number(buf)
    }

    fn string(&mut self) -> Token {
        let mut buf = String::new();
        buf.push(self.advance().unwrap()); // push '"' to buf

        while let Some(&c) = self.peek() {
            if c == '"' || c == '\n' {
                break;
            }
            buf.push(c);
            self.advance();
        }

        buf.push(self.advance().unwrap()); // push '"' to buf
        Token::String(buf)
    }

    fn name(&mut self) -> Token {
        let mut buf = String::new();
        while let Some(&c) = self.peek() {
            if c == ' ' || c == '\n' || !Self::is_valid_for_identifier(c) {
                break;
            }
            buf.push(c);
            self.advance();
        }
        if Self::is_keyword(&buf) {
            Token::Keyword(buf)
        } else {
            Token::Name(buf)
        }
    }

    fn is_valid_for_identifier(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn is_punctuation(c: char) -> bool {
        match c {
            c if c.is_alphanumeric() => false,
            c if c.is_ascii_whitespace() => false,
            _ => true,
        }
    }

    fn is_keyword(name: &String) -> bool {
        let keywords: HashSet<&str> =
            HashSet::<_>::from_iter(["fn", "mod", "use", "let", "for", "in"]);
        keywords.contains(name.as_str())
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
