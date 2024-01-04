use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Whitespace(usize),
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
    pub(crate) tokens: Vec<Token>,
}

impl Line {
    fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

pub struct Scanner<'a> {
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
        lines.push(line);
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
            '"' => self.string(),
            c if Self::is_punctuation(c) => self.punctuation(),
            c if c.is_ascii_digit() => self.number(),
            _ => self.name(),
        }
    }

    fn whitespace(&mut self) -> Token {
        let mut n: usize = 0;
        while let Some(' ') = self.peek() {
            n += 1;
            self.advance();
        }
        Token::Whitespace(n)
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
            if !c.is_ascii_digit() && c != '.' {
                break;
            }
            buf.push(c);
            self.advance();
        }
        Token::Number(buf)
    }

    fn string(&mut self) -> Token {
        let mut buf = String::new();
        let mut prev_char: char = '\n';
        buf.push(self.advance().unwrap()); // push '"' to buf

        while let Some(&c) = self.peek() {
            match c {
                '\n' => break,
                '"' if prev_char != '\\' => break, // if prev_char=='\\', then escaped
                _ => {
                    buf.push(c);
                    prev_char = c;
                    self.advance();
                }
            }
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

    fn is_keyword(name: &str) -> bool {
        let keywords: HashSet<&str> = HashSet::<_>::from_iter([
            "Self",
            "abstract",
            "as",
            "async",
            "await",
            "become",
            "box",
            "break",
            "const",
            "continue",
            "crate",
            "do",
            "dyn",
            "dyn",
            "else",
            "enum",
            "extern",
            "false",
            "final",
            "fn",
            "for",
            "if",
            "impl",
            "in",
            "let",
            "loop",
            "macro",
            "macro_rules",
            "match",
            "mod",
            "move",
            "mut",
            "override",
            "priv",
            "pub",
            "ref",
            "return",
            "self",
            "static",
            "struct",
            "super",
            "trait",
            "true",
            "try",
            "type",
            "typeof",
            "union",
            "unsafe",
            "unsized",
            "use",
            "virtual",
            "where",
            "while",
            "yield",
        ]);
        keywords.contains(name)
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

#[cfg(test)]
mod tests {
    use crate::scanner::{Scanner, Token};

    #[test]
    fn test_single_line() {
        let input = "let a = 10;";
        
        let mut scanner = Scanner::new(input);
        let scanned = scanner.scan();

        assert_eq!(scanned.len(), 1);
        assert_eq!(
            vec![
                Token::Keyword("let".to_string()),
                Token::Whitespace(1),
                Token::Name("a".to_string()),
                Token::Whitespace(1),
                Token::Punctuation("=".to_string()),
                Token::Whitespace(1),
                Token::Number("10".to_string()),
                Token::Punctuation(";".to_string())
            ],
            scanned[0].tokens
        );
    }
}
