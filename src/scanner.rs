use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub(crate) enum Token {
    WhiteSpace(usize),
    Text(String),
    Eof,
}

#[derive(Debug)]
pub(crate) struct Line {
    tokens: Vec<Token>,
}

pub(crate) struct Scanner<'a> {
    source: &'a str,
    iter: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            source: input,
            iter: input.chars().peekable(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
        }
    }

    pub(crate) fn scan_tokens(&mut self) -> Result<&Vec<Token>, &str> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(Token::Eof);

        Ok(&self.tokens)
    }

    fn scan_token(&mut self) {
        let o = self.peek();
        let c = match o {
            None => return,
            Some(c) => c,
        };
        match c {
            ' ' => self.whitespace(),
            _ => self.text(),
        }
    }

    fn whitespace(&mut self) {
        let mut n: usize = 0;
        while let Some(' ') = self.peek() {
            n += 1;
            self.advance();
        }
        self.add_token(Token::WhiteSpace(n));
    }

    fn text(&mut self) {
        let mut buf = String::new();
        while let Some(c) = self.peek() {
            if *c == ' ' {
                break;
            }
            buf.push(*c);
            self.advance();
        }
        self.add_token(Token::Text(buf));
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn advance(&mut self) -> Option<char> {
        match self.iter.next() {
            Some(c) => {
                self.current += 1;
                Some(c)
            }
            None => None,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
